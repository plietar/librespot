use std::io;

pub trait Sink {
    fn start(&self) -> io::Result<()>;
    fn stop(&self) -> io::Result<()>;
    fn write(&self, data: &[i16]) -> io::Result<()>;
}

#[cfg(feature = "portaudio-sink")]
mod portaudio_sink {
    use audio_sink::Sink;
    use std::io;
    use portaudio;
    pub struct PortAudioSink<'a>(portaudio::stream::Stream<'a, i16, i16>);

    impl <'a> PortAudioSink<'a> {
        pub fn open() -> PortAudioSink<'a> {
            println!("Using PortAudioSink");
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
        fn start(&self) -> io::Result<()> {
            self.0.start().unwrap();
            Ok(())
        }
        fn stop(&self) -> io::Result<()> {
            self.0.stop().unwrap();
            Ok(())
        }
        fn write(&self, data: &[i16]) -> io::Result<()> {
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

#[cfg(feature = "portaudio-sink")]
pub type DefaultSink = portaudio_sink::PortAudioSink<'static>;

#[cfg(feature = "pulseaudio-sink")]
mod pulseaudio_sink {
    use audio_sink::Sink;
    use std::io;
    use libpulse_sys::*;
    use std::ptr::{null, null_mut};
    use std::mem::{transmute};

    pub struct PulseAudioSink(*mut pa_simple);

    #[cfg(target_arch = "arm")]
    type PtrType = u8;

    #[cfg(target_arch = "x86_64")]
    type PtrType = i8;

    #[cfg(target_arch = "x86")]
    type PtrType = i8;
    
    
    impl PulseAudioSink {
        pub fn open() -> PulseAudioSink {
            println!("Using PulseAudioSink");

            let ss = pa_sample_spec {
                format: PA_SAMPLE_S16LE,
                channels: 2, // stereo
                rate: 44100
            };
            
            let s = unsafe {
                pa_simple_new(null(),             // Use the default server.
                              "librespot".as_ptr() as *const PtrType,  // Our application's name.
                              PA_STREAM_PLAYBACK,
                              null(),             // Use the default device.
                              "A spoty client library".as_ptr() as *const PtrType,  // Description of our stream.
                              &ss,                // Our sample format.
                              null(),             // Use default channel map
                              null(),             // Use default buffering attributes.
                              null_mut(),         // Ignore error code.
                )
            };
            assert!(s != null_mut());
            
            PulseAudioSink(s)
        }
    }

    impl Sink for PulseAudioSink {
        fn start(&self) -> io::Result<()> {
            Ok(())
        }

        fn stop(&self) -> io::Result<()> {
            Ok(())
        }

        fn write(&self, data: &[i16]) -> io::Result<()> {
            unsafe {
                let ptr = transmute(data.as_ptr());
                let bytes = data.len() * 2;
                pa_simple_write(self.0, ptr, bytes, null_mut());
            };
            
            Ok(())
        }
    }
}

#[cfg(feature = "pulseaudio-sink")]
pub type DefaultSink = pulseaudio_sink::PulseAudioSink;

