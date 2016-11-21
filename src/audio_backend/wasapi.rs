extern crate winapi;
extern crate ole32;
extern crate kernel32;

use std::io;
use std::mem;
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

use super::Sink;
use audio_queue::AudioConsumer;

pub struct WasapiSink {
    running: Arc<(Mutex<bool>, Condvar)>,
}

fn open_device() -> Result<(*mut winapi::IAudioClient, *mut winapi::IAudioRenderClient), ()> {
    unsafe {
        let mut enumerator: *mut winapi::IMMDeviceEnumerator = ptr::null_mut();
        let hr = ole32::CoCreateInstance(
            &winapi::CLSID_MMDeviceEnumerator, ptr::null_mut(),
            winapi::CLSCTX_ALL, &winapi::IID_IMMDeviceEnumerator,
            &mut enumerator as *mut *mut winapi::IMMDeviceEnumerator as *mut _,
        );

        assert!(hr >= 0);
        assert!(!enumerator.is_null());

        let mut device : *mut winapi::IMMDevice = ptr::null_mut();
        let hr = (*enumerator).GetDefaultAudioEndpoint(
            winapi::eRender, winapi::eConsole, &mut device
        );

        assert!(hr >= 0);
        assert!(!device.is_null());

        let mut audio_client : *mut winapi::IAudioClient = ptr::null_mut();
        let hr = (*device).Activate(
            &winapi::IID_IAudioClient,
            winapi::CLSCTX_ALL,
            ptr::null_mut(),
            &mut audio_client as *mut *mut winapi::IAudioClient as *mut _,
        );

        assert!(hr >= 0);
        assert!(!audio_client.is_null());

        let fmt = winapi::WAVEFORMATEXTENSIBLE {
            Format: winapi::WAVEFORMATEX {
                wFormatTag: winapi::WAVE_FORMAT_PCM,
                nChannels: 2,
                nSamplesPerSec: 44100,
                nAvgBytesPerSec: 2 * 44100 * 2,
                nBlockAlign: 2 * 2,
                wBitsPerSample: 16,
                cbSize: 0,
            },
            Samples: 16,
            dwChannelMask: winapi::SPEAKER_FRONT_LEFT | winapi::SPEAKER_FRONT_RIGHT,
            SubFormat: winapi::KSDATAFORMAT_SUBTYPE_PCM,
        };

        let hr = (*audio_client).Initialize(
            winapi::AUDCLNT_SHAREMODE_SHARED,
            winapi::AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
            0, 0,
            &fmt.Format,
            ptr::null(),
        );

        assert!(hr >= 0);

        let mut render_client: *mut winapi::IAudioRenderClient = ptr::null_mut();
        let hr = (*audio_client).GetService(
            &winapi::IID_IAudioRenderClient,
            &mut render_client as *mut *mut winapi::IAudioRenderClient as *mut _
        );

        assert!(hr >= 0);
        assert!(!render_client.is_null());

        Ok((audio_client, render_client))
    }
}

fn player_thread(queue: AudioConsumer<i16>, running: Arc<(Mutex<bool>, Condvar)>) {
    unsafe {
        let hr = ole32::CoInitializeEx(ptr::null_mut(), winapi::COINIT_MULTITHREADED);
        assert!(hr >= 0);

        let (audio_client, render_client) = open_device().unwrap();
        let mut max_frames_in_buffer = mem::uninitialized();
        let hr = (*audio_client).GetBufferSize(&mut max_frames_in_buffer);
        assert!(hr >= 0);
        println!("max_frames_in_buffer = {:?}", max_frames_in_buffer);

        let event = kernel32::CreateEventA(ptr::null_mut(), 0, 0, ptr::null());
        let hr = (*audio_client).SetEventHandle(event);
        assert!(hr >= 0);

        let hr = (*audio_client).Start();
        assert!(hr >= 0);

        loop {
            let mut guard = running.0.lock().unwrap();
            if !*guard {
                let hr = (*audio_client).Stop();
                assert!(hr >= 0);

                while !*guard {
                    guard = running.1.wait(guard).unwrap();
                }

                let hr = (*audio_client).Start();
                assert!(hr >= 0);
            }

            kernel32::WaitForSingleObject(event, winapi::INFINITE);
            let frames_available = {
                let mut padding = 0;
                let hr = (*audio_client).GetCurrentPadding(&mut padding);
                assert!(hr >= 0);
                max_frames_in_buffer - padding
            };

            if frames_available == 0 {
                continue;
            }

            let mut raw_buffer: *mut winapi::BYTE = ptr::null_mut();
            let hr = (*render_client).GetBuffer(frames_available,
                                                &mut raw_buffer as *mut *mut _);
            assert!(!raw_buffer.is_null());
            assert!(hr >= 0);

            let mut buffer = slice::from_raw_parts_mut(raw_buffer as *mut i16, frames_available as usize * 2);
            queue.read(&mut buffer);

            let hr = (*render_client).ReleaseBuffer(frames_available as u32, 0);
            assert!(hr >= 0);
        }
    }
}

impl Sink for WasapiSink {
    fn open(arg: Option<String>, queue: AudioConsumer<i16>) -> io::Result<Self>
        where Self: Sized
    {
        if arg.is_some() {
            panic!("WASAPI backend does not support device selection");
        }

        let running = Arc::new((Mutex::new(false), Condvar::new()));
        let running_ = running.clone();

        thread::spawn(move || player_thread(queue, running_));

        Ok(WasapiSink { running: running })
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
