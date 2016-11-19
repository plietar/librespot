use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat, StreamFormat};
use coreaudio::audio_unit::audio_format::linear_pcm_flags::IS_SIGNED_INTEGER;
use coreaudio::audio_unit::render_callback::{self, data};

use audio_queue;
use futures::{Future, Stream, Async, Poll};

use audio_file::AudioFile;
use audio_decrypt::AudioDecrypt;
use util::Subfile;
use ogg_async;
use types::*;
use metadata;
use session::Session;
use util::SpotifyId;

struct AudioSink {
    audio_unit: AudioUnit,
}

impl AudioSink {
    pub fn new(consumer: audio_queue::AudioConsumer<i16>) -> AudioSink {
        let mut audio_unit = AudioUnit::new(IOType::DefaultOutput).unwrap();
        let format = StreamFormat {
            sample_rate: 44100.,
            sample_format: SampleFormat::I16,
            flags: IS_SIGNED_INTEGER,
            channels_per_frame: 2,
        };
        audio_unit.set_stream_format(format).unwrap();

        let mut total = Box::new(0usize);
        type Args = render_callback::Args<data::NonInterleaved<i16>>;
        audio_unit.set_render_callback(move |args| {
                let Args { num_frames, mut data, .. } = args;

                for mut channel in data.channels_mut() {
                    assert_eq!(channel.len(), num_frames * 2);
                    consumer.read(&mut channel);
                    *total += channel.len();
                }

                Ok(())
            })
            .unwrap();

        AudioSink { audio_unit: audio_unit }
    }

    pub fn start(&mut self) {
        self.audio_unit.start().unwrap();
    }

    pub fn pause(&mut self) {
        self.audio_unit.stop().unwrap();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PlayerState {
    Stopped,
    Loading,
    Loaded,
}

pub struct Player {
    session: Session,
    state: PlayerState,

    stream: Option<SpStream<'static, Vec<i16>>>,
    packet: Vec<i16>,
    offset: usize,

    sink: AudioSink,
    queue: audio_queue::AudioProducer<i16>,

    total: usize,
}

impl Player {
    pub fn new(session: Session) -> Player {
        let (producer, consumer) = audio_queue::make(32768 * 16);

        Player {
            session: session,
            state: PlayerState::Stopped,

            stream: None,
            packet: Vec::new(),
            offset: 0,

            sink: AudioSink::new(consumer),
            queue: producer,

            total: 0,
        }
    }

    pub fn load(&mut self, track_id: SpotifyId) {
        let session = self.session.clone();
        let session2 = self.session.clone();
        let stream = metadata::get::<metadata::Track>(&self.session, track_id)
            .map(|track| track.find_file(metadata::FileFormat::OGG_VORBIS_320).unwrap())
            .and_then(move |file_id| {
                session.audio_key()
                    .request(track_id, file_id)
                    .map(move |key| (file_id, key))
            })
            .and_then(move |(file_id, key)| {
                let file = AudioFile::new(&session2, file_id);
                let file = AudioDecrypt::new(file, key);
                let file = Subfile::new(file, 0xa7);

                ogg_async::open(file)
                    .map_err(SpError::from)
                    .map(|s| s.map_err(SpError::from))
            })
            .flatten_stream()
            .sp_boxed();

        self.queue.clear();
        self.packet = Vec::new();
        self.offset = 0;
        self.stream = Some(stream);
        if self.state == PlayerState::Loaded {
            self.sink.pause();
        }
        self.state = PlayerState::Loading;
    }

    fn fill_queue(&mut self) -> Poll<(), SpError> {
        loop {
            while self.offset < self.packet.len() {
                let count = self.queue.try_write(&self.packet[self.offset..]);
                self.offset += count;
                self.total -= count;

                if count == 0 {
                    return Ok(Async::Ready(()));
                }
            }

            self.packet = Vec::new();
            self.offset = 0;

            let stream = self.stream.as_mut().unwrap();
            match try_ready!(stream.poll()) {
                Some(packet) => {
                    self.total += packet.len();

                    self.packet = packet;
                    self.offset = 0;
                }
                None => panic!("Song over")
            }
        }
    }
}

impl Future for Player {
    type Item = ();
    type Error = SpError;

    fn poll(&mut self) -> Poll<(), SpError> {
        use self::PlayerState::*;
        loop {
            trace!("polling state={:?} packet={}/{} queue={}/{}",
                   self.state,
                   self.packet.len() - self.offset, self.packet.len(),
                   self.queue.size(), self.queue.capacity());

            match self.state {
                Stopped => return Ok(Async::NotReady),
                Loading => {
                    try_ready!(self.fill_queue());

                    trace!("loaded, start playing");
                    self.sink.start();
                    self.state = PlayerState::Loaded;
                    return Ok(Async::NotReady);
                }
                Loaded => {
                    if self.queue.underrun() {
                        trace!("************ Player underrun");
                        self.sink.pause();
                        self.state = PlayerState::Loading;
                    } else {
                        try_ready!(self.fill_queue());

                        return Ok(Async::NotReady);
                    }
                }
            }
        }
    }
}
