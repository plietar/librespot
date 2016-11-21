use futures::Async;
use std::cmp;
use std::io::{self, Read};
use tokio::io::EasyBuf;

use audio_chunk::{get_chunk, get_chunk_offseted, CHUNK_SIZE};
use session::Session;
use types::*;
use util::FileId;

pub struct AudioFile {
    session: Session,
    file: FileId,

    eof: bool,
    chunk: usize,
    offset: usize,

    request: Option<SpFuture<'static, EasyBuf>>,
    data: EasyBuf,
}

impl AudioFile {
    pub fn new(session: Session, file: FileId) -> AudioFile {
        AudioFile {
            session: session,
            file: file,

            eof: false,
            chunk: 0,
            offset: 0,

            request: None,
            data: EasyBuf::new(),
        }
    }

}

impl Read for AudioFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!("polling audio file self.data={} buf={}", self.data.len(), buf.len());
        loop {
            if self.eof {
                return Ok(0);
            }

            if self.data.len() > 0 {
                let count = cmp::min(buf.len(), self.data.len());
                buf[..count].copy_from_slice(self.data.drain_to(count).as_ref());
                return Ok(count);
            }

            if self.request.is_none() {
                debug!("fetch chunk {}", self.chunk);
                self.request = Some(get_chunk_offseted(&self.session, self.file, self.chunk, self.offset));
                self.offset = 0;
            }

            match self.request.as_mut().unwrap().poll() {
                Ok(Async::Ready(data)) => {
                    self.data = data;
                    self.chunk += 1;
                    self.request = Some(get_chunk(&self.session, self.file, self.chunk));
                    debug!("prefetch chunk {}", self.chunk);
                },

                Ok(Async::NotReady) => {
                    return Err(io::Error::new(io::ErrorKind::WouldBlock, "would block"))
                }

                // TODO: check error code, may not actually be EOF
                Err(_) => self.eof = true,
            }
        }
    }
}

impl io::Seek for AudioFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.eof = false;
        match pos {
            io::SeekFrom::Start(pos) => {
                self.chunk = pos as usize / CHUNK_SIZE;
                self.offset = pos as usize % CHUNK_SIZE;

                debug!("seeking pos={} chunk={}, offset={}", pos, self.chunk, self.offset);

                self.data = EasyBuf::new();
                self.request = None;
                Ok(pos)
            }
            _ => unimplemented!(),
        }
    }
}
