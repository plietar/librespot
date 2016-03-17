use std::io;

#[cfg(not(target_os = "linux"))]
pub type DefaultSink = portaudio_sink::PortAudioSink<'static>;

#[cfg(target_os = "linux")]
#[cfg(not(feature = "enigma2"))]
pub type DefaultSink = alsa_sink::AlsaSink;

#[cfg(target_os = "linux")]
#[cfg(feature = "enigma2")]
pub type DefaultSink = gstreamer_sink::GstreamerSink;

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
#[cfg(not(feature = "enigma2"))]
mod alsa_sink {
    use audio_sink::Sink;
    use std::io;

    use alsa::{PCM, Stream, Mode, Format, Access};

    pub struct AlsaSink(PCM);

    impl AlsaSink {
        pub fn open() -> AlsaSink {
            let pcm = PCM::open("default", Stream::Playback, Mode::Blocking,
                                Format::Signed16, Access::Noninterleaved, 2, 44100).ok().unwrap();

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

#[cfg(target_os = "linux")]
#[cfg(feature = "enigma2")]
mod gstreamer_sink {
    use audio_sink::Sink;
    use std::io;
    use std::thread;
    use std::sync::{Condvar,Mutex};
    use gst;
    use gst::{BinT, ElementT};
    pub struct GstreamerSink;

    impl GstreamerSink {
        pub fn open() -> GstreamerSink {
            gst::init();
            let pipeline_str = "appsrc caps=\"audio/x-raw,format=S8,channels=2\" name=appsrc0 ! audioconvert ! autoaudiosink";
            let mut pipeline = gst::Pipeline::new_from_str(pipeline_str).unwrap();
            let mut mainloop = gst::MainLoop::new();
            let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
            let bus_receiver = bus.receiver();
            let appsrc = pipeline.get_by_name("appsrc0").expect("Couldn't get appsrc from pipeline");
            let mut appsrc = gst::AppSrc::new_from_element(appsrc);
            let bufferpool = gst::BufferPool::new().unwrap();
            let appsrc_caps = appsrc.caps().unwrap();
            bufferpool.set_params(&appsrc_caps,64,0,0);
            if bufferpool.set_active(true).is_err(){
                panic!("Couldn't activate buffer pool");
            }
            mainloop.spawn();
            pipeline.play();

            thread::spawn(move||{
                let condvar = Condvar::new();
                let mutex = Mutex::new(());
                loop {
                    if let Some(mut buffer) = bufferpool.acquire_buffer(){
                        appsrc.push_buffer(buffer);
                        let guard = mutex.lock().unwrap();
                        condvar.wait_timeout_ms(guard,(1000./60.) as u32).ok();
                    }else{
                        println!("Couldn't get buffer, sending EOS and finishing thread");
                        appsrc.end_of_stream();
                        break;
                    }
                }
            });
            for message in bus_receiver.iter(){
                match message.parse(){
                    gst::Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => {
                        println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
                    }
                    gst::Message::ErrorParsed{ref msg, ref error, ref debug} => {
                        println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
                        break;
                    }
                    gst::Message::Eos(ref msg) => {
                        println!("eos received quiting");
                        break;
                    }
                    _ => {
                        println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
                    }
                }
            }
            bus.receiver();
            mainloop.quit();
        }
    }
    impl Sink for GstreamerSink {
        fn start(&mut self) -> io::Result<()> {
            //self.o.start().unwrap();
            Ok(())
        }
        fn stop(&mut self) -> io::Result<()> {
            //self.0.stop().unwrap();
            Ok(())
        }
        fn write(&mut self, data: &[i16]) -> io::Result<()> {
            //self.0.write_interleaved(data).unwrap();

            Ok(())
        }
    }        
}