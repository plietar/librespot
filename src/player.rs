use futures::{Future, Stream, Async, Poll};

use audio_file::AudioFile;
use audio_decrypt::AudioDecrypt;
use util::Subfile;
use ogg_async;
use types::*;
use metadata;
use session::Session;
use util::SpotifyId;
use audio_backend::{Sink, DefaultSink};

use audio_queue::{self, AudioProducer};

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

    sink: Box<Sink>,
    queue: AudioProducer<i16>,
}

impl Player {
    pub fn new(session: Session) -> Player {
        let (producer, consumer) = audio_queue::make(32768 * 16);

        let sink = DefaultSink::open(None, consumer).unwrap();

        Player {
            session: session,
            state: PlayerState::Stopped,

            stream: None,
            packet: Vec::new(),
            offset: 0,

            sink: Box::new(sink),
            queue: producer,
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

        if self.state == PlayerState::Loaded {
            self.sink.pause().unwrap();
        }
        self.queue.clear();
        self.packet = Vec::new();
        self.offset = 0;
        self.stream = Some(stream);
        self.state = PlayerState::Loading;
    }

    fn fill_queue(&mut self) -> Poll<(), SpError> {
        loop {
            while self.offset < self.packet.len() {
                let count = self.queue.try_write(&self.packet[self.offset..]);
                self.offset += count;

                if count == 0 {
                    return Ok(Async::Ready(()));
                }
            }

            self.packet = Vec::new();
            self.offset = 0;

            let stream = self.stream.as_mut().unwrap();
            match try_ready!(stream.poll()) {
                Some(packet) => {
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
                    self.sink.start()?;
                    self.state = PlayerState::Loaded;
                    return Ok(Async::NotReady);
                }
                Loaded => {
                    if self.queue.underrun() {
                        trace!("player underrun");
                        self.sink.pause().unwrap();
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
