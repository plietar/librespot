use session::{SessionWeak, Component};
use futures::{oneshot, Complete, Canceled, Future};
use std::collections::HashMap;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::io::Read;
use types::*;
use std::mem;
use tokio::io::EasyBuf;
use tokio;

use protocol;
use protobuf;
use futures::Stream;

mod types;
mod sender;
pub use self::types::{MercuryMethod, MercuryRequest, MercuryResponse};
pub use self::sender::MercurySender;

pub type MercuryManager = Component<MercuryManagerInner>;
pub struct MercuryManagerInner {
    next_seq: u64,
    pending: HashMap<Vec<u8>, MercuryPending>,
    subscriptions: HashMap<String, tokio::channel::Sender<MercuryResponse>>,
}

pub struct MercuryPending {
    parts: Vec<Vec<u8>>,
    partial: Option<Vec<u8>>,
    callback: Option<Complete<SpResult<MercuryResponse>>>,
}

impl MercuryManager {
    pub fn new(session: SessionWeak) -> MercuryManager {
        Component::create(session,
                          MercuryManagerInner {
                              next_seq: 0,
                              pending: HashMap::new(),
                              subscriptions: HashMap::new(),
                          })
    }

    fn next_seq(&self) -> Vec<u8> {
        let mut seq = vec![0u8; 8];
        let mut inner = self.lock();
        BigEndian::write_u64(&mut seq, inner.next_seq);
        inner.next_seq += 1;
        seq
    }

    pub fn request<'a>(&self, req: MercuryRequest) -> SpFuture<'a, MercuryResponse> {
        let (complete, future) = oneshot();
        let seq = self.next_seq();

        let cmd = req.method.command();
        let data = req.encode(&seq);

        let pending = MercuryPending {
            parts: Vec::new(),
            partial: None,
            callback: Some(complete),
        };
        self.lock().pending.insert(seq, pending);

        self.session()
            .connection()
            .send(cmd, data).and_then(move |()| {
                future.then(|result| match result {
                    Ok(result) => result,
                    Err(Canceled)
                        => Err(SpError::InternalError("mercury complete dropped")),
                })
            }).sp_boxed()
    }

    pub fn get<'a, T: Into<String>>(&self, uri: T) -> SpFuture<'a, MercuryResponse> {
        self.request(MercuryRequest {
            method: MercuryMethod::GET,
            uri: uri.into(),
            content_type: None,
            payload: Vec::new(),
        })
    }

    pub fn send<'a, T: Into<String>>(&self,
                                     uri: T,
                                     data: Vec<u8>)
                                     -> SpFuture<'a, MercuryResponse> {
        self.request(MercuryRequest {
            method: MercuryMethod::SEND,
            uri: uri.into(),
            content_type: None,
            payload: vec![data],
        })
    }

    pub fn sender<T: Into<String>>(&self, uri: T) -> MercurySender {
        MercurySender::new(self.clone(), uri.into())
    }

    pub fn subscribe<'a, T: Into<String>>(&self,
                                          uri: T)
                                          -> SpFuture<'a, SpStream<'a, MercuryResponse>> {
        let manager = self.clone();
        self.request(MercuryRequest {
                method: MercuryMethod::SUB,
                uri: uri.into(),
                content_type: None,
                payload: Vec::new(),
            })
            .and_then(move |response| {
                let (tx, rx) = tokio::channel::channel(&manager.session().handle())?;

                let mut manager = manager.lock();
                for sub in response.payload {
                    let mut sub =
                        protobuf::parse_from_bytes::<protocol::pubsub::Subscription>(&sub)?;
                    let uri = sub.take_uri();
                    manager.subscriptions.insert(uri, tx.clone());
                }

                let rx = rx.map_err(SpError::from);
                Ok(rx.sp_boxed())
            })
            .sp_boxed()
    }

    pub fn dispatch(&self, cmd: u8, data: EasyBuf) {
        let mut packet = ::std::io::Cursor::new(data);
        let seq = {
            let len = packet.read_u16::<BigEndian>().unwrap() as usize;
            let mut seq = vec![0; len];
            packet.read_exact(&mut seq).unwrap();
            seq
        };
        let flags = packet.read_u8().unwrap();
        let count = packet.read_u16::<BigEndian>().unwrap() as usize;

        let mut pending = match self.lock().pending.remove(&seq) {
            Some(pending) => pending,
            None if cmd == 0xb5 => {
                MercuryPending {
                    parts: Vec::new(),
                    partial: None,
                    callback: None,
                }
            }
            None => {
                warn!("Ignore seq {:?} cmd {:x}", seq, cmd);
                return;
            }
        };

        for i in 0..count {
            let mut part = Self::parse_part(&mut packet);
            if let Some(mut data) = mem::replace(&mut pending.partial, None) {
                data.append(&mut part);
                part = data;
            }

            if i == count - 1 && (flags == 2) {
                pending.partial = Some(part)
            } else {
                pending.parts.push(part);
            }
        }

        if flags == 0x1 {
            self.complete_request(cmd, pending);
        } else {
            self.lock().pending.insert(seq, pending);
        }
    }

    fn parse_part<T: Read>(s: &mut T) -> Vec<u8> {
        let size = s.read_u16::<BigEndian>().unwrap() as usize;
        let mut buffer = vec![0; size];
        s.read_exact(&mut buffer).unwrap();

        buffer
    }

    fn complete_request(&self, cmd: u8, mut pending: MercuryPending) {
        let header_data = pending.parts.remove(0);
        let header: protocol::mercury::Header = protobuf::parse_from_bytes(&header_data).unwrap();

        let response = MercuryResponse {
            uri: header.get_uri().to_owned(),
            payload: pending.parts,
        };

        if cmd == 0xb5 {
            if let Some(cb) = self.lock().subscriptions.get(&response.uri) {
                cb.send(response).unwrap();
            }
        } else if let Some(cb) = pending.callback {
            cb.complete(Ok(response));
        }
    }
}
