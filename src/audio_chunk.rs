use byteorder::{BigEndian, WriteBytesExt};
use futures::{Future, Stream};
use std::io::Write;
use tokio::io::EasyBuf;

use session::Session;
use types::*;
use util::FileId;

pub const CHUNK_SIZE: usize = 0x20000;

pub fn get_chunk<'a>(session: &Session, file: FileId, index: usize) -> SpFuture<'a, EasyBuf> {
    let (channel_id, channel) = session.channel().allocate();

    let start = index * CHUNK_SIZE;
    let end = start + CHUNK_SIZE;

    let mut req: Vec<u8> = Vec::new();
    req.write_u16::<BigEndian>(channel_id).unwrap();
    req.write_u16::<BigEndian>(1).unwrap();
    req.write_u16::<BigEndian>(0x0000).unwrap();
    req.write_u32::<BigEndian>(0x00000000).unwrap();
    req.write_u32::<BigEndian>(0x00000000).unwrap();
    req.write_u32::<BigEndian>(0x00000000).unwrap();
    req.write(&file.0).unwrap();
    req.write_u32::<BigEndian>((start / 4) as u32).unwrap();
    req.write_u32::<BigEndian>((end / 4) as u32).unwrap();

    session.connection()
        .send(0x8, req)
        .map(move |_| channel)
        .flatten_stream()
        .fold(EasyBuf::with_capacity(CHUNK_SIZE), |mut buffer, data| {
            buffer.get_mut().extend_from_slice(data.as_ref());
            Ok::<_, SpError>(buffer)
        }).sp_boxed()
}

pub fn get_chunk_offseted<'a>(session: &Session, file: FileId,
                              index: usize, offset: usize) -> SpFuture<'a, EasyBuf> {
    get_chunk(session, file, index).map(move |mut buf| buf.split_off(offset)).sp_boxed()
}
