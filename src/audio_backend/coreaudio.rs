extern crate coreaudio;

use self::coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat, StreamFormat};
use self::coreaudio::audio_unit::audio_format::linear_pcm_flags::IS_SIGNED_INTEGER;
use self::coreaudio::audio_unit::render_callback::{self, data};
use self::coreaudio::Error as CoreAudioError;
use std::io;

use super::Sink;
use audio_queue::AudioConsumer;

pub struct CoreAudioSink {
    audio_unit: AudioUnit,
}

impl Sink for CoreAudioSink {
    fn open(arg: Option<String>, queue: AudioConsumer<i16>) -> io::Result<Self>
        where Self: Sized
    {
        if arg.is_some() {
            panic!("CoreAudio backend does not support device selection");
        }

        let mut audio_unit = coreaudio_result(AudioUnit::new(IOType::DefaultOutput))?;
        let format = StreamFormat {
            sample_rate: 44100.,
            sample_format: SampleFormat::I16,
            flags: IS_SIGNED_INTEGER,
            channels_per_frame: 2,
        };
        coreaudio_result(audio_unit.set_stream_format(format))?;

        // coreaudio-rs doesn't properly support interleaved mode yet.
        // However it seems to work fine pretending it's non interleaved.
        type Args = render_callback::Args<data::NonInterleaved<i16>>;
        audio_unit.set_render_callback(move |args| {
            let Args { num_frames, mut data, .. } = args;
            for mut channel in data.channels_mut() {
                assert_eq!(channel.len(), num_frames * 2);
                queue.read(&mut channel);
            }
            Ok(())
        }).unwrap();

        Ok(CoreAudioSink {
            audio_unit: audio_unit,
        })
    }

    fn start(&mut self) -> io::Result<()> {
        coreaudio_result(self.audio_unit.start())
    }

    fn pause(&mut self) -> io::Result<()> {
        coreaudio_result(self.audio_unit.stop())
    }
}

fn coreaudio_result<T>(r: Result<T, CoreAudioError>) -> io::Result<T> {
    r.map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}
