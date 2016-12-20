use super::{Open, Sink};
use std::io;

pub struct NilSink();

impl Open for NilSink {
    fn open(_device: Option<&str>) -> NilSink {
        NilSink()
    }
}

impl Sink for NilSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, _data: &[i16]) -> io::Result<()> {
        Ok(())
    }
}
