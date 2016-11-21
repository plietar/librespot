extern crate alsa;

use self::alsa::{Direction, ValueOr};
use self::alsa::pcm::{PCM, HwParams, Format, Access};
use std::ffi::CString;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

use super::Sink;
use audio_queue::AudioConsumer;

pub struct AlsaSink {
    running: Arc<(Mutex<bool>, Condvar)>,
}

fn open_pcm(device: Option<String>) -> Result<PCM, ()> {
    let device = device.unwrap_or("default".to_owned());
    let device = CString::new(device.into_bytes()).unwrap();

    let pcm = PCM::open(&device, Direction::Playback, false).unwrap();

    {
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(2).unwrap();
        hwp.set_rate(44100, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
    }

    Ok(pcm)
}

impl Sink for AlsaSink {
    fn open(arg: Option<String>, queue: AudioConsumer<i16>) -> io::Result<Self>
        where Self: Sized
    {
        let pcm = open_pcm(arg).unwrap();

        let running = Arc::new((Mutex::new(false), Condvar::new()));
        let running_ = running.clone();

        thread::spawn(move || {
            let io = pcm.io_i16().unwrap();
            let mut buffer = [0i16; 4096];

            loop {
                let mut guard = running_.0.lock().unwrap();
                if !*guard {
                    pcm.drain().unwrap();
                    while !*guard {
                        guard = running_.1.wait(guard).unwrap();
                    }
                    pcm.prepare().unwrap();
                }

                queue.read(&mut buffer);
                io.writei(&buffer)
                  .unwrap_or_else(|err| {
                      pcm.recover(err.code(), false).unwrap();
                      io.writei(&buffer).unwrap()
                  });
            }
        });

        Ok(AlsaSink { running: running })
    }

    fn start(&mut self) -> io::Result<()> {
        *self.running.0.lock().unwrap() = true;
        self.running.1.notify_one();
        Ok(())
    }

    fn pause(&mut self) -> io::Result<()> {
        *self.running.0.lock().unwrap() = false;
        self.running.1.notify_one();
        Ok(())
    }
}
