use futures::{Future, Stream, Sink};
use protobuf::ProtobufError;
use std::sync::Arc;
use std::io;
use lewton::VorbisError;

#[derive(Debug,Clone)]
pub enum SpError {
    Io(Arc<io::Error>),
    ProtocolError(&'static str),
    AuthenticationFailed,
    InternalError(&'static str),
    ConnectionClosed,
    Vorbis(Arc<VorbisError>),
}

impl From<io::Error> for SpError {
    fn from(err: io::Error) -> SpError {
        SpError::Io(Arc::new(err))
    }
}

impl From<VorbisError> for SpError {
    fn from(err: VorbisError) -> SpError {
        SpError::Vorbis(Arc::new(err))
    }
}

impl From<ProtobufError> for SpError {
    fn from(_err: ProtobufError) -> SpError {
        SpError::ProtocolError("protobuf decoding")
    }
}

/*
impl From<SpError> for io::Error {
    fn from(err: SpError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, "SpError")
    }
}
*/

pub type SpResult<T> = Result<T, SpError>;
pub type SpFuture<'a, T> = Box<Future<Item = T, Error = SpError> + 'a>;
pub type SpStream<'a, T> = Box<Stream<Item = T, Error = SpError> + 'a>;
pub type SpSink<'a, T> = Box<Sink<SinkItem = T, SinkError = SpError> + 'a>;

pub trait SpFutureExt<'a, T> {
    fn sp_boxed(self) -> SpFuture<'a, T>;
}
impl<'a, T> SpFutureExt<'a, T::Item> for T
    where T: Future<Error = SpError> + 'a
{
    fn sp_boxed(self) -> SpFuture<'a, T::Item> {
        Box::new(self)
    }
}

pub trait SpStreamExt<'a, T> {
    fn sp_boxed(self) -> SpStream<'a, T>;
}
impl<'a, T> SpStreamExt<'a, T::Item> for T
    where T: Stream<Error = SpError> + 'a
{
    fn sp_boxed(self) -> SpStream<'a, T::Item> {
        Box::new(self)
    }
}

pub trait SpSinkExt<'a, T> {
    fn sp_boxed(self) -> SpSink<'a, T>;
}
impl<'a, T> SpSinkExt<'a, T::SinkItem> for T
    where T: Sink<SinkError = SpError> + 'a
{
    fn sp_boxed(self) -> SpSink<'a, T::SinkItem> {
        Box::new(self)
    }
}
