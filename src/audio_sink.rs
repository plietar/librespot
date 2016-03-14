use std::io;

#[cfg(not(target_os = "linux"))]
pub type DefaultSink = portaudio_sink::PortAudioSink<'static>;

#[cfg(target_os = "linux")]
pub type DefaultSink = alsa_sink::AlsaSink;

pub trait Sink {
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self) -> io::Result<()>;
    fn write(&mut self, data: &[i16]) -> io::Result<()>;
}

#[cfg(not(target_os = "linux"))]
mod portaudio_sink {
    use audio_sink::Sink;
    use std::io;
    use portaudio;
    pub struct PortAudioSink<'a>(portaudio::stream::Stream<'a, i16, i16>);

    impl <'a> PortAudioSink<'a> {
        pub fn open() -> PortAudioSink<'a> {
            portaudio::initialize().unwrap();

            let stream = portaudio::stream::Stream::open_default(
                    0, 2, 44100.0,
                    portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                    None
            ).unwrap();

            PortAudioSink(stream)
        }
    }

    impl <'a> Sink for PortAudioSink<'a> {
        fn start(&mut self) -> io::Result<()> {
            self.0.start().unwrap();
            Ok(())
        }
        fn stop(&mut self) -> io::Result<()> {
            self.0.stop().unwrap();
            Ok(())
        }
        fn write(&mut self, data: &[i16]) -> io::Result<()> {
            match self.0.write(&data) {
                Ok(_) => (),
                Err(portaudio::PaError::OutputUnderflowed) => eprintln!("Underflow"),
                Err(e) => panic!("PA Error {}", e),
            };

            Ok(())
        }
    }

    impl <'a> Drop for PortAudioSink<'a> {
        fn drop(&mut self) {
            portaudio::terminate().unwrap();
        }
    }
}

#[cfg(target_os = "linux")]
mod alsa_sink {
    use audio_sink::Sink;
    use std::io;

    use alsa::{PCM, Stream, Mode, Format, Access};

    pub struct AlsaSink(PCM);

    impl AlsaSink {
        pub fn open() -> AlsaSink {
            let pcm = PCM::open("default", Stream::Playback, Mode::Blocking,
                                Format::Signed16, Access::Interleaved, 2, 44100).ok().unwrap();

            AlsaSink(pcm)
        }
    }

    impl Sink for AlsaSink {
        fn start(&mut self) -> io::Result<()> {
            //self.0.start().unwrap();
            Ok(())
        }
        fn stop(&mut self) -> io::Result<()> {
            //self.0.pause().unwrap();
            Ok(())
        }
        fn write(&mut self, data: &[i16]) -> io::Result<()> {
            self.0.write_interleaved(data).unwrap();

            Ok(())
        }
    }
}

