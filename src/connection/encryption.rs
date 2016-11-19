pub use tokio::io::{Codec, EasyBuf};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use shannon::Shannon;
use std::io;

const HEADER_SIZE: usize = 3;
const MAC_SIZE: usize = 4;

struct ShannonEncode {
    nonce: u32,
    cipher: Shannon,
}

impl ShannonEncode {
    fn new(key: &[u8]) -> ShannonEncode {
        ShannonEncode {
            nonce: 0,
            cipher: Shannon::new(key),
        }
    }

    fn encode(&mut self, item: (u8, Vec<u8>), buf: &mut Vec<u8>) {
        let (cmd, payload) = item;
        let offset = buf.len();

        // println!("-> Header {:x} {}", cmd, payload.len());

        buf.write_u8(cmd).unwrap();
        buf.write_u16::<BigEndian>(payload.len() as u16).unwrap();
        buf.extend_from_slice(&payload);

        self.cipher.nonce_u32(self.nonce);
        self.nonce += 1;
        self.cipher.encrypt(&mut buf[offset..]);
        let mac = self.cipher.finish(MAC_SIZE as u32);
        buf.extend_from_slice(&mac);
    }
}

#[derive(Debug)]
enum DecodeState {
    Header,
    Payload(u8, usize),
}

struct ShannonDecode {
    nonce: u32,
    cipher: Shannon,
    state: DecodeState,
}

impl ShannonDecode {
    fn new(key: &[u8]) -> ShannonDecode {
        ShannonDecode {
            nonce: 0,
            cipher: Shannon::new(key),
            state: DecodeState::Header,
        }
    }

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<(u8, EasyBuf)>, io::Error> {
        loop {
            match self.state {
                DecodeState::Header if buf.len() >= HEADER_SIZE => {
                    let mut header = [0u8; HEADER_SIZE];
                    header.copy_from_slice(buf.drain_to(HEADER_SIZE).as_slice());

                    self.cipher.nonce_u32(self.nonce);
                    self.nonce += 1;
                    self.cipher.decrypt(&mut header);

                    let cmd = header[0];
                    let size = BigEndian::read_u16(&header[1..]) as usize;
                    self.state = DecodeState::Payload(cmd, size);
                    // println!("<- Header {:x} {}", cmd, size);
                }

                DecodeState::Payload(cmd, size) if buf.len() >= size + MAC_SIZE => {
                    self.state = DecodeState::Header;

                    let mut payload = buf.drain_to(size + MAC_SIZE);

                    self.cipher.decrypt(&mut payload.get_mut()[..size]);
                    let mac = payload.split_off(size);
                    self.cipher.check_mac(mac.as_slice())?;

                    return Ok(Some((cmd, payload)));
                }

                _ => {
                    return Ok(None);
                }
            }
        }
    }
}

pub struct ShannonCodec {
    encoder: ShannonEncode,
    decoder: ShannonDecode,
}

impl ShannonCodec {
    pub fn new(send_key: &[u8], recv_key: &[u8]) -> ShannonCodec {
        ShannonCodec {
            encoder: ShannonEncode::new(send_key),
            decoder: ShannonDecode::new(recv_key),
        }
    }
}

impl Codec for ShannonCodec {
    type Out = (u8, Vec<u8>);
    type In = (u8, EasyBuf);

    fn encode(&mut self, item: (u8, Vec<u8>), buf: &mut Vec<u8>) {
        self.encoder.encode(item, buf)
    }

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<(u8, EasyBuf)>, io::Error> {
        self.decoder.decode(buf)
    }
}
