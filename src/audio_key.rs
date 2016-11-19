use session::{SessionWeak, Component};
use futures::{oneshot, Complete, Canceled, Future};
use std::collections::HashMap;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use std::io::Write;
use types::*;
use tokio::io::EasyBuf;

use util::{SpotifyId, FileId};

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct AudioKey([u8; 16]);

pub type AudioKeyManager = Component<AudioKeyManagerInner>;
pub struct AudioKeyManagerInner {
    next_seq: u32,
    pending: HashMap<u32, Complete<SpResult<AudioKey>>>,
}

impl AudioKeyManager {
    pub fn new(session: SessionWeak) -> AudioKeyManager {
        Component::create(session,
                          AudioKeyManagerInner {
                              next_seq: 0,
                              pending: HashMap::new(),
                          })
    }

    pub fn request<'a>(&self, track: SpotifyId, file: FileId) -> SpFuture<'a, AudioKey> {
        let (complete, future) = oneshot();
        let seq = {
            let mut inner = self.lock();
            let seq = inner.next_seq;
            inner.next_seq += 1;
            seq
        };

        let manager = self.clone();
        self.send_key_request(seq, track, file).and_then(move |()| {
            manager.lock().pending.insert(seq, complete);

            future.then(|result| {
                match result {
                    Ok(result) => result,
                    Err(Canceled) => {
                        Err(SpError::InternalError("audio key complete dropped"))
                    },
                }
            })
        }).sp_boxed()
    }

    pub fn dispatch(&self, cmd: u8, data: EasyBuf) {
        let data = data.as_ref();
        let seq = BigEndian::read_u32(&data);
        let complete = {
            let mut inner = self.lock();
            inner.pending.remove(&seq)
        };

        if let Some(complete) = complete {
            match cmd {
                0xd => {
                    let mut key = [0u8; 16];
                    key.copy_from_slice(&data[4..20]);
                    complete.complete(Ok(AudioKey(key)));
                }
                0xe => {
                    println!("{:x} {:x}", data[4], data[5]);
                    complete.complete(Err(SpError::ProtocolError("no key found")));
                }
                _ => (),
            }
        }
    }

    fn send_key_request<'a>(&self, seq: u32, track: SpotifyId, file: FileId) -> SpFuture<'a, ()> {
        let mut data: Vec<u8> = Vec::new();
        data.write(&file.0).unwrap();
        data.write(&track.to_raw()).unwrap();
        data.write_u32::<BigEndian>(seq).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();

        self.session().connection().send(0xc, data)
    }
}

impl AsRef<[u8]> for AudioKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
