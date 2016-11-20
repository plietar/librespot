use super::Sink;
use audio_queue::AudioConsumer;
use std::io;

pub struct WasapiSink { }

impl Sink for WasapiSink {
    fn open(arg: Option<String>, queue: AudioConsumer<i16>) -> io::Result<Self>
        where Self: Sized
    {
        unimplemented!();
    }

    fn start(&mut self) -> io::Result<()> {
        unimplemented!();
    }

    fn pause(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}
