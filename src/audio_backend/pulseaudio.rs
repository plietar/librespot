use super::{Open, Sink};
use std::io;
use libpulse_sys::*;
use pulse_simple_ng::*;
use std::ptr::{null, null_mut};
use std::ffi::CString;
use std::cell::Cell;

pub struct PulseAudioSink(Cell<*mut pa_simple>);

fn init_pa_simple() -> *mut pa_simple {
    let ss = pa_sample_spec {
        format: PA_SAMPLE_S16LE,
        channels: 2, // stereo
        rate: 44100
    };
    
    let name = CString::new("librespot").unwrap();
    let description = CString::new("A spoty client library").unwrap();

    let s = unsafe {
        pa_simple_new(null(),               // Use the default server.
                      name.as_ptr(),        // Our application's name.
                      PA_STREAM_PLAYBACK,
                      null(),               // Use the default device.
                      description.as_ptr(), // Description of our stream.
                      &ss,                  // Our sample format.
                      null(),               // Use default channel map
                      null(),               // Use default buffering attributes.
                      null_mut(),           // Ignore error code.
        )
    };
    assert!(s != null_mut());

    info!("Initialized pulse audio");
    
    s
}


impl Open for PulseAudioSink {
   fn open(device: Option<&str>) -> PulseAudioSink {
        debug!("Using PulseAudio sink");

        if device.is_some() {
            panic!("pulseaudio sink does not support specifying a device name");
        }

        PulseAudioSink(Cell::new(init_pa_simple()))

   }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        if let Err(error) = write_to_pa(self.0.get(), data)  {
            warn!("Error writing to pulseaudio: {:?}", error);
            match error {
                PaErrorCode::ErrConnectionTerminated => {
                    let old_pa = self.0.get();
                    self.0.set(init_pa_simple());
                    unsafe {
                        pa_simple_free(old_pa);
                    }
                },
                _ => info!("Could not recover error")
            }
        }
        
        Ok(())
    }
}




