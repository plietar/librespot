use super::{Open, Sink};
use std::io;
use libpulse_sys::*;
use pulse_simple_ng::*;
use std::ptr::{null, null_mut};
use std::ffi::CString;
use std::cell::Cell;
use libc::*;

pub struct PulseAudioSink(Cell<*mut pa_simple>);

fn init_pa_simple() -> *mut pa_simple {
    let ss = pa_sample_spec {
        format: PA_SAMPLE_S16LE,
        channels: 2, // stereo
        rate: 44100
    };
    
    let attr = pa_buffer_attr {
        maxlength: !0 as c_uint,
        tlength: !0 as c_uint,
        prebuf: !0 as c_uint,
        minreq: !0 as c_uint,
        fragsize: 0
    };

    let name = CString::new("librespot").unwrap();
    let description = CString::new("A spoty client library").unwrap();

    let mut error_value: c_int = PA_OK as c_int;
    let s = unsafe {
        let error = &mut error_value;
            pa_simple_new(null(),           // Use the default server.
                      name.as_ptr(),        // Our application's name.
                      PA_STREAM_PLAYBACK,
                      null(),               // Use the default device.
                      description.as_ptr(), // Description of our stream.
                      &ss,                  // Our sample format.
                      null(),               // Use default channel map
                      &attr,                // Buffering attributes
                      error                 // Ignore error code.
        )
    };
    if s == null_mut() {
        error!("Could not connect to PulseAudio: {:?}", map_error_code(error_value as c_uint));
        panic!("Exiting due to unrecoverable error");
    }

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

fn reconnect_pulse(error: PaErrorCode, cell: &Cell<*mut pa_simple>) {
    info!("Trying to recover from {:?}", error);
    let old_pa = cell.get();
    cell.set(init_pa_simple());
    unsafe {
        pa_simple_free(old_pa);
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
                PaErrorCode::ErrConnectionTerminated => reconnect_pulse(error, &self.0),
                PaErrorCode::ErrKilled => reconnect_pulse(error, &self.0),
                _ => info!("Could not recover error")
            }
        }
        
        Ok(())
    }
}




