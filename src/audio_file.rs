use byteorder::{BigEndian, WriteBytesExt};
use futures::{Async, Poll, Stream};
use std::io::{self, Read, Write};
use tokio::io::EasyBuf;

use channel::ChannelStream;
use session::SessionWeak;
use types::*;
use util::FileId;

const CHUNK_SIZE: u64 = 0x20000;

pub struct AudioFile<'a> {
    session: SessionWeak,
    file: FileId,
    next_chunk: u64,
    offset: u64,

    request: Option<SpFuture<'a, ()>>,
    channel: Option<ChannelStream>,
    data: EasyBuf,
}

impl<'a> AudioFile<'a> {
    pub fn new<T: Into<SessionWeak>>(session: T, file: FileId) -> AudioFile<'a> {
        AudioFile {
            session: session.into(),
            file: file,
            next_chunk: 0,
            offset: 0,
            request: None,
            channel: None,
            data: EasyBuf::new(),
        }
    }

    fn get_next_chunk(&mut self) {
        let (channel_id, channel) = self.session.channel().allocate();

        let start = ((self.next_chunk * CHUNK_SIZE + self.offset) / 4) as u32;
        let end = ((self.next_chunk + 1) * CHUNK_SIZE / 4) as u32;

        let mut data: Vec<u8> = Vec::new();
        data.write_u16::<BigEndian>(channel_id).unwrap();
        data.write_u16::<BigEndian>(1).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write(&self.file.0).unwrap();
        data.write_u32::<BigEndian>(start).unwrap();
        data.write_u32::<BigEndian>(end).unwrap();

        self.next_chunk += 1;
        self.offset = self.offset % 4;

        self.request = Some(self.session.connection().send(0x8, data).sp_boxed());
        self.channel = Some(channel);
    }

    fn read_buf(&mut self) -> Poll<&mut EasyBuf, SpError> {
        loop {
            if self.data.len() > 0 {
                return Ok(Async::Ready(&mut self.data));
            }

            if self.channel.is_none() {
                self.get_next_chunk();
            }

            if let Some(ref mut request) = self.request {
                try_ready!(request.poll());
            }
            self.request = None;

            let read = {
                let channel = self.channel.as_mut().expect("No channel");
                try_ready!(channel.poll())
            };

            match read {
                Some(mut data) => {
                    self.data = data.split_off(self.offset as usize);
                    self.offset = 0;
                }
                None => self.channel = None,
            }
        }
    }
}

impl<'a> Read for AudioFile<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data = match self.read_buf() {
            Ok(Async::Ready(data)) => data,
            Ok(Async::NotReady) => {
                return Err(io::Error::new(io::ErrorKind::WouldBlock, "would block"))
            }
            // TODO: handle this better than EOF
            Err(_) => return Ok(0),
        };

        let count = ::std::cmp::min(buf.len(), data.len());
        buf[..count].copy_from_slice(data.drain_to(count).as_ref());
        Ok(count)
    }
}

impl<'a> io::Seek for AudioFile<'a> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(pos) => {
                self.next_chunk = pos / CHUNK_SIZE;
                self.offset = pos % CHUNK_SIZE;
                self.channel = None;
                Ok(pos)
            }
            _ => unimplemented!(),
        }
    }
}
