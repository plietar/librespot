use protobuf::{self, Message};
use futures::{self, Future, Stream, Sink, Async};
use tokio;
use tokio::io::{Io, Framed, EasyBuf};
use tokio::net::TcpStream;
use std::io;
use std::net::ToSocketAddrs;

use authentication::Credentials;
use broadcast::{broadcast, BroadcastSender, BroadcastReceiver};
use protocol;
use session::{SessionWeak, Component};
use types::*;
use version;

mod early;
mod encryption;

use self::early::EarlyConnection;
use self::encryption::ShannonCodec;

enum State {
    NotConnected,
    Connecting,
    Connected { sender: tokio::channel::Sender<(u8, Vec<u8>)>, },
}

pub type Connection = Component<ConnectionInner>;

pub struct ConnectionInner {
    state: State,
    sender: BroadcastSender<ConnectionChange>,
    receiver: BroadcastReceiver<ConnectionChange>,
}

#[derive(Debug, Clone)]
pub enum ConnectionChange {
    Connected(String),
}

impl Connection {
    pub fn new(session: SessionWeak) -> Connection {
        let (sender, receiver) = broadcast();

        Component::create(session,
                          ConnectionInner {
                              state: State::NotConnected,
                              sender: sender,
                              receiver: receiver,
                          })
    }

    pub fn updates(&self) -> BroadcastReceiver<ConnectionChange> {
        self.lock().receiver.clone()
    }

    pub fn connected(&self) -> bool {
        match self.lock().state {
            State::NotConnected | State::Connecting => false,
            State::Connected { .. } => true,
        }
    }

    pub fn connect<'a>(&self, credentials: Credentials) -> SpFuture<'a, Credentials> {
        self.lock().state = State::Connecting;

        const AP: &'static str = "sto3-accesspoint-a51.ap.spotify.com:4070";
        let addr = AP.to_socket_addrs().unwrap().next().unwrap();

        let connection = self.clone();
        let handle = self.session().handle();

        Box::new(TcpStream::connect(&addr, &handle)
            .map_err(SpError::from)
            .and_then(|stream| early::keyexchange(EarlyConnection::new(stream)))
            .and_then(move |(stream, send_key, recv_key)| {
                let codec = ShannonCodec::new(&send_key, &recv_key);
                let stream = stream.framed(codec);
                authenticate(stream, credentials)
            })
            .map(move |(stream, credentials)| {
                let (receiver, sender) = stream.split();

                let receiver_task = connection.packet_receiver(receiver);
                let (sender_task, sender) = connection.packet_sender(sender);

                handle.spawn(receiver_task.map_err(|err| panic!(err)));
                handle.spawn(sender_task.map_err(|err| panic!(err)));

                let mut connection_ = connection.lock();
                connection_.state = State::Connected { sender: sender };
                connection_.sender.send(ConnectionChange::Connected(credentials.username.clone()));

                credentials
            }))
    }

    pub fn send<'a>(&self, cmd: u8, data: Vec<u8>) -> SpFuture<'a, ()> {
        match self.lock().state {
            State::NotConnected | State::Connecting => {
                futures::failed(SpError::ConnectionClosed).sp_boxed()
            }
            State::Connected { ref sender } => {
                sender.send((cmd, data)).unwrap();
                futures::finished(()).sp_boxed()
            }
        }
    }

    fn dispatch(&self, cmd: u8, data: EasyBuf) {
        match cmd {
            0xd | 0xe => self.session().audio_key().dispatch(cmd, data),
            0x9 | 0xa => self.session().channel().dispatch(cmd, data),
            0xb2...0xb6 => self.session().mercury().dispatch(cmd, data),

            0x04 => {
                self.session()
                    .upgrade()
                    .spawn(self.send(0x49, data.as_ref().to_owned()));
            }
            _ => {
                warn!("Unknown command {:#x} {}", cmd, data.len());
            }
        }
    }

    fn packet_receiver<'a, I>(&self, incoming: I) -> SpFuture<'a, ()>
        where I: Stream<Item = (u8, EasyBuf), Error = io::Error> + 'a
    {
        let connection = self.clone();
        Box::new(incoming.map_err(SpError::from)
            .for_each(move |(cmd, data)| {
                connection.dispatch(cmd, data);
                Ok(())
            }))
    }

    fn packet_sender<'a, O>(&self,
                            outgoing: O)
                            -> (SpFuture<'a, ()>, tokio::channel::Sender<(u8, Vec<u8>)>)
        where O: Sink<SinkItem = (u8, Vec<u8>), SinkError = io::Error> + Send + 'a
    {
        let (sender, incoming) = tokio::channel::channel(&self.session().handle()).unwrap();
        // let future = PacketSender::recv_incoming(incoming, outgoing).sp_boxed();

        use std::sync::{Arc, Mutex};
        // Meh, not really nice, but necessary unless I make an explicit state machine out of it
        let outgoing = Arc::new(Mutex::new(Some(outgoing)));
        let future = incoming.and_then(move |msg:(u8, Vec<u8>)| {
            let outgoing_ = outgoing.clone();
            let stream = outgoing.lock().unwrap().take().expect("missing outgoing stream");
            stream.send(msg)
                .and_then(move |stream| {
                    *outgoing_.lock().unwrap() = Some(stream);
                    Ok(())
                })
        }).for_each(|()| Ok(())).map_err(SpError::from).sp_boxed();

        (future, sender)
    }
}

enum PacketSender<I, O>
    where I: Stream<Item = (u8, Vec<u8>), Error = io::Error>,
          O: Sink<SinkItem = (u8, Vec<u8>), SinkError = io::Error>
{
    WaitIncoming(futures::stream::StreamFuture<I>, Option<O>),
    WaitOutgoing(futures::sink::Send<O>, Option<I>),
}

impl<I, O> PacketSender<I, O>
    where I: Stream<Item = (u8, Vec<u8>), Error = io::Error>,
          O: Sink<SinkItem = (u8, Vec<u8>), SinkError = io::Error>
{
    fn recv_incoming(incoming: I, outgoing: O) -> Self {
        let f = incoming.into_future();
        PacketSender::WaitIncoming(f, Some(outgoing))
    }

    fn send_outgoing(incoming: I, outgoing: O, msg: (u8, Vec<u8>)) -> Self {
        let f = outgoing.send(msg);
        PacketSender::WaitOutgoing(f, Some(incoming))
    }
}

impl<I, O> Future for PacketSender<I, O>
    where I: Stream<Item = (u8, Vec<u8>), Error = io::Error>,
          O: Sink<SinkItem = (u8, Vec<u8>), SinkError = io::Error>
{
    type Item = ();
    type Error = SpError;

    fn poll(&mut self) -> SpResult<Async<()>> {
        use self::PacketSender::*;
        loop {
            *self = match *self {
                WaitIncoming(ref mut f, ref mut data) => {
                    let (msg, incoming) = match try_ready!(f.poll().map_err(|(err, _)| err)) {
                        (None, _) => return Ok(Async::Ready(())),
                        (Some(result), incoming) => (result, incoming),
                    };

                    let outgoing = data.take().expect("Invalid state");
                    Self::send_outgoing(incoming, outgoing, msg)
                }
                WaitOutgoing(ref mut f, ref mut data) => {
                    let outgoing = try_ready!(f.poll());
                    let incoming = data.take().expect("Invalid state");

                    Self::recv_incoming(incoming, outgoing)
                }
            };
        }
    }
}

fn authenticate<'a, S: Io + Send + 'a>(stream: Framed<S, ShannonCodec>,
                                       credentials: Credentials)
                                       -> SpFuture<'a, (Framed<S, ShannonCodec>, Credentials)> {
    let payload = protobuf_init!(protocol::authentication::ClientResponseEncrypted::new(), {
        login_credentials => {
            username: credentials.username,
            typ: credentials.auth_type,
            auth_data: credentials.auth_data,
        },
        system_info => {
            cpu_family: protocol::authentication::CpuFamily::CPU_UNKNOWN,
            os: protocol::authentication::Os::OS_UNKNOWN,
            device_id: "foobar".to_owned(),
        },
        version_string: version::version_string(),
    }).write_to_bytes().unwrap();

    stream.send((0xab, payload))
        .and_then(|stream| stream.into_future().map_err(|(err, _stream)| err))
        .map_err(SpError::from)
        .and_then(|(packet, stream)| {
            match packet {
                Some((0xac, data)) => {
                    let welcome_data: protocol::authentication::APWelcome =
                        protobuf::parse_from_bytes(data.as_ref()).unwrap();

                    let reusable_credentials = Credentials {
                        username: welcome_data.get_canonical_username().to_owned(),
                        auth_type: welcome_data.get_reusable_auth_credentials_type(),
                        auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
                    };

                    Ok((stream, reusable_credentials))
                }

                Some((0xad, _)) => Err(SpError::AuthenticationFailed),
                Some((_, _)) => Err(SpError::ProtocolError("Unexpected packet")),
                None => Err(SpError::ConnectionClosed),
            }
        })
        .sp_boxed()
}
