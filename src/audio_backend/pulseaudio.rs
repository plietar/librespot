use super::{Open, Sink};
use std::io;
use libpulse_sys::*;
use std::ptr::{null, null_mut};
use std::mem::{transmute};
use std::ffi::CString;
use libc::*;

pub struct PulseAudioSink(*mut pa_simple);

impl Open for PulseAudioSink {
   fn open() -> PulseAudioSink {
        info!("Using PulseAudioSink");

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
        
        PulseAudioSink(s)
    }
}

#[derive(Debug)]
enum PaErrorCode {
    Ok,
    ErrAccess,
    ErrCommand,
    ErrInvalid,
    ErrExist,
    ErrNoEntity,
    ErrConnectionRefused,
    ErrProtocol,
    ErrTimeout,
    ErrAuthkey,
    ErrInternal,
    ErrConnectionTerminated,
    ErrKilled,
    ErrInvalidServer,
    ErrModuleInitFailed,
    ErrBadState,
    ErrNoData,
    ErrVersion,
    ErrTooLarge,
    ErrNotSupported,
    ErrUnknown,
    ErrNoExtension,
    ErrObsolete,
    ErrNotImplemented,
    ErrForked,
    ErrIo,
    ErrBusy,
    ErrMax,
    Unknown
}

fn map_error_code(error: c_uint) -> PaErrorCode {
    match error {
        PA_OK => PaErrorCode::Ok,
        PA_ERR_ACCESS => PaErrorCode::ErrAccess,
        PA_ERR_COMMAND => PaErrorCode::ErrCommand,
        PA_ERR_INVALID => PaErrorCode::ErrInvalid,
        PA_ERR_EXIST => PaErrorCode::ErrExist,
        PA_ERR_NOENTITY => PaErrorCode::ErrNoEntity,
        PA_ERR_CONNECTIONREFUSED => PaErrorCode::ErrConnectionRefused,
        PA_ERR_PROTOCOL => PaErrorCode::ErrProtocol,
        PA_ERR_TIMEOUT => PaErrorCode::ErrTimeout,
        PA_ERR_AUTHKEY => PaErrorCode::ErrAuthkey,
        PA_ERR_INTERNAL => PaErrorCode::ErrInternal,
        PA_ERR_CONNECTIONTERMINATED => PaErrorCode::ErrConnectionTerminated,
        PA_ERR_KILLED => PaErrorCode::ErrKilled,
        PA_ERR_INVALIDSERVER => PaErrorCode::ErrInvalidServer,
        PA_ERR_MODINITFAILED => PaErrorCode::ErrModuleInitFailed,
        PA_ERR_BADSTATE => PaErrorCode::ErrBadState,
        PA_ERR_NODATA => PaErrorCode::ErrNoData,
        PA_ERR_VERSION => PaErrorCode::ErrVersion,
        PA_ERR_TOOLARGE => PaErrorCode::ErrTooLarge,
        PA_ERR_NOTSUPPORTED => PaErrorCode::ErrNotSupported,
        PA_ERR_UNKNOWN => PaErrorCode::ErrUnknown,
        PA_ERR_NOEXTENSION => PaErrorCode::ErrNoExtension,
        PA_ERR_OBSOLETE => PaErrorCode::ErrObsolete,
        PA_ERR_NOTIMPLEMENTED => PaErrorCode::ErrNotImplemented,
        PA_ERR_FORKED => PaErrorCode::ErrForked,
        PA_ERR_IO => PaErrorCode::ErrIo,
        PA_ERR_BUSY => PaErrorCode::ErrBusy,
        PA_ERR_MAX => PaErrorCode::ErrMax,
        _ => PaErrorCode::Unknown
    }
}

fn write_to_pa(pa: *mut pa_simple, data: &[i16]) -> Result<(),PaErrorCode> {
    unsafe {
        let ptr = transmute(data.as_ptr());
        let bytes = data.len() as usize * 2;
        trace!("Writing {} bytes of data", bytes);

        let mut error_value: c_int = PA_OK as c_int;
        let write_result = {
            let error = &mut error_value;
            pa_simple_write(pa, ptr, bytes, error)
        };
        
        if write_result < 0 {
            Err(map_error_code(error_value as c_uint))
        } else {
            Ok(())
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&self) -> io::Result<()> {
        //trace!("Pulse Audio Sink start called")
        Ok(())
    }

    fn stop(&self) -> io::Result<()> {
        //trace!("Pulse Audio Sink stop called")
        Ok(())
    }

    fn write(&self, data: &[i16]) -> io::Result<()> {
        if let Err(error) = write_to_pa(self.0, data)  {
            warn!("Error writing to pulseaudio: {:?}", error);
        }
        
        Ok(())
    }
}




