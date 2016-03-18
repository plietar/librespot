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
    use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
    use gst;
    use gst::{BinT, ElementT, PipelineT};

    pub struct GstreamerSink {
        tx: SyncSender<Vec<i16>>,
        rx: Receiver<Vec<i16>>,
        pipe: gst::Pipeline
    }

    impl GstreamerSink {
        pub fn open() -> GstreamerSink {
            gst::init();
            let pipeline_str = "appsrc caps=\"audio/x-raw,format=S16LE,layout=interleaved,channels=2,rate=44100\" name=appsrc0 ! audioconvert ! autoaudiosink";
            let mut pipeline = gst::Pipeline::new_from_str(pipeline_str).expect("New Pipeline error");
            let mut mainloop = gst::MainLoop::new();
            //let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
            //let bus_receiver = bus.receiver();
            let appsrc_element = pipeline.get_by_name("appsrc0").expect("Couldn't get appsrc from pipeline");
            let appsrc = gst::AppSrc::new_from_element(appsrc_element.to_element());
            let bufferpool = gst::BufferPool::new().expect("New Buffer Pool error");
            let appsrc_caps = appsrc.caps().expect("set appsrc caps failed");
            bufferpool.set_params(&appsrc_caps,64,0,0);
            if bufferpool.set_active(true).is_err(){
                panic!("Couldn't activate buffer pool");
            }
            mainloop.spawn();

            let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
            thread::spawn(move || {
            let bus_receiver = bus.receiver();
                for message in bus_receiver.iter() {
                    match message.parse() {
                        gst::Message::StateChangedParsed{msg: _, ref old, ref new, pending: _} =>
                            println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new),
                        gst::Message::StateChanged(_) =>
                            println!("element `{}` state changed", message.src_name()),
                        gst::Message::ErrorParsed{msg: _, ref error, debug: _} => {
                            println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
                            break;
                        },
                        gst::Message::Eos(ref _msg) => {
                            println!("eos received quiting");
                            break;
                        },
                        _ =>
                            println!("Pipe message: {} from {} at {}", message.type_name(), message.src_name(), message.timestamp())
                    }
                }
            });



            let (tx, rx) = sync_channel(64);
            thread::spawn(move||{
                for data in self.rx.recv().expect("rx recive failed in thread")::Vec<i16> {
                    println!("thread running...");
                    let mut buffer = bufferpool.acquire_buffer().expect("acquire buffer");
                    if let Err(e) = buffer.map_write(|mut mapping| {
                        for (i, c) in mapping.iter_mut::<i16>().enumerate() {
                            *c = data[i];
                        }
                    }) {
                        println!("Cannot write to buffer: {:?}", e);
                        return false;
                    }
                    buffer.set_live(true);
                    let res = appsrc.push_buffer(buffer);
                    if res != 0 {
                        panic!("push_buffer: {}", res);
                    }
                }
            });
            pipeline.play();
            GstreamerSink {
                tx: tx,
                rx: rx,
                pipe: pipeline
            }
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
            // Copy expensively to avoid thread synchronization
            let data = data.to_vec();
            self.tx.send(data).expect("tx send failed in write function");

            Ok(())
        }
    }
}