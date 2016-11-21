use std::io::{Seek, ErrorKind, Read};
use futures::{Future, Poll, Async, Stream};

use ogg::{self, OggReadError};
use lewton::VorbisError;
use lewton::inside_ogg::{HeadersReader, OggStreamReader};

pub fn open<S>(stream: S) -> OggOpen<S>
    where S: Read + Seek
{
    let buf_reader = ogg::BufReader::new(stream);
    let packet_reader = ogg::PacketReader::new(buf_reader);
    let headers_reader = HeadersReader::new(packet_reader);

    OggOpen(Some(headers_reader))
}

macro_rules! try_vorbis_nb {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(VorbisError::ReadError(err)) => {
                if err.kind() == ErrorKind::WouldBlock {
                    return Ok(Async::NotReady)
                } else {
                    return Err(VorbisError::ReadError(err))
                }
            }

            Err(VorbisError::OggError(OggReadError::ReadError(err))) => {
                if err.kind() == ErrorKind::WouldBlock {
                    return Ok(Async::NotReady)
                } else {
                    return Err(VorbisError::OggError(OggReadError::ReadError(err)))
                }
            }

            Err(err) => return Err(err),
        }
    }
}

pub struct OggOpen<S: Read + Seek>(Option<HeadersReader<ogg::BufReader<S>>>);
impl<S> Future for OggOpen<S>
    where S: Read + Seek
{
    type Item = OggStream<S>;
    type Error = VorbisError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        try_vorbis_nb!(self.0.as_mut().expect("Future already resolved").try_read_headers());

        let inner = self.0.take().unwrap();
        let stream_reader = inner.into_ogg_stream_reader();
        Ok(Async::Ready(OggStream(stream_reader)))
    }
}

pub struct OggStream<S: Read + Seek>(OggStreamReader<ogg::BufReader<S>>);
impl<S> Stream for OggStream<S>
    where S: Read + Seek
{
    type Item = Vec<i16>;
    type Error = VorbisError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let packet = try_vorbis_nb!(self.0.read_dec_packet_itl());
        Ok(Async::Ready(packet))
    }
}

impl<S> ::std::ops::Deref for OggStream<S>
    where S: Read + Seek
{
    type Target = OggStreamReader<ogg::BufReader<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
