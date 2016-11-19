use session::{SessionWeak, Component};
use types::*;

use byteorder::{BigEndian, ByteOrder};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom};
use tokio as tokio;
use futures::{Poll, Async};
use futures::stream::Stream;
use tokio::io::EasyBuf;
use byteorder::ReadBytesExt;

pub type ChannelManager = Component<ChannelManagerInner>;
pub struct ChannelManagerInner {
    next_id: u16,
    channels: HashMap<u16, tokio::channel::Sender<(u8, EasyBuf)>>,
}

pub struct ChannelStream {
    receiver: tokio::channel::Receiver<(u8, EasyBuf)>,
    state: ChannelState,
}

#[derive(Debug)]
enum ChannelState {
    Header,
    Data,
    Closed,
}

impl ChannelManager {
    pub fn new(session: SessionWeak) -> ChannelManager {
        Component::create(session,
                          ChannelManagerInner {
                              next_id: 0,
                              channels: HashMap::new(),
                          })
    }

    pub fn allocate(&self) -> (u16, ChannelStream) {
        let handle = self.session().handle();
        let (tx, rx) = tokio::channel::channel(&handle).unwrap();

        let mut inner = self.lock();
        let id = inner.next_id;
        inner.next_id += 1;
        inner.channels.insert(id, tx);

        (id,
         ChannelStream {
            receiver: rx,
            state: ChannelState::Header,
        })
    }

    pub fn dispatch(&self, cmd: u8, mut data: EasyBuf) {
        use std::collections::hash_map::Entry;

        let id: u16 = BigEndian::read_u16(data.drain_to(2).as_ref());

        if let Entry::Occupied(entry) = self.lock().channels.entry(id) {
            let _ = entry.get().send((cmd, data));
        }
    }
}

impl ChannelStream {
    fn recv_packet(&mut self) -> Poll<EasyBuf, SpError> {
        let (cmd, packet) = try_ready!(self.receiver.poll()).expect("Channel manager died ?");

        if cmd == 0xa {
            let code = BigEndian::read_u16(&packet.as_ref()[..2]);
            error!("error: {} {}", packet.len(), code);
            self.state = ChannelState::Closed;
            Err(SpError::ProtocolError("channel error"))
        } else {
            Ok(Async::Ready(packet))
        }
    }
}

impl Stream for ChannelStream {
    type Item = EasyBuf;
    type Error = SpError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state {
                ChannelState::Closed => panic!("Polling already terminated channel"),
                ChannelState::Header => {
                    let data = try_ready!(self.recv_packet());
                    let mut packet = Cursor::new(&data);

                    while packet.position() < data.as_ref().len() as u64 {
                        let length = packet.read_u16::<BigEndian>().unwrap();
                        packet.seek(SeekFrom::Current(length as i64)).unwrap();

                        if length == 0 {
                            self.state = ChannelState::Data;
                            break;
                        }
                    }
                }

                ChannelState::Data => {
                    let data = try_ready!(self.recv_packet());
                    if data.as_ref().is_empty() {
                        self.state = ChannelState::Closed;
                        return Ok(Async::Ready(None));
                    } else {
                        return Ok(Async::Ready(Some(data)));
                    }
                }
            }
        }
    }
}
