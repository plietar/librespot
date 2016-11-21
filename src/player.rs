use futures::{self, Future, Stream, Async, Poll};

use audio_backend::{Sink, DefaultSink};
use audio_decrypt::AudioDecrypt;
use audio_file::AudioFile;
use audio_queue::{self, AudioProducer};
use metadata;
use ogg_async;
use session::Session;
use types::*;
use util::{SpotifyId, Subfile};

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
        let (producer, consumer) = audio_queue::make(32768);

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

    fn find_available_alternative<'a>(&self, track_id: SpotifyId) -> SpFuture<'a, Option<metadata::Track>> {
        let session = self.session.clone();
        let country = self.session.country();

        metadata::get::<metadata::Track>(&session, track_id)
            .and_then(move |track| {
                if track.available(&country, "premium") {
                    futures::finished(Some(track)).sp_boxed()
                } else {
                    let alternatives : Vec<_> = track.alternatives().map(Ok).collect();
                    let a = futures::stream::iter(alternatives)
                        .and_then(move |id| metadata::get::<metadata::Track>(&session, id))
                        .filter(move |track| track.available(&country, "premium"))
                        .into_future()
                        .map(|(track, _)| track)
                        .map_err(|_| panic!());

                    a.sp_boxed()
                }
            }).sp_boxed()
    }

    pub fn load(&mut self, track_id: SpotifyId) {
        let session = self.session.clone();
        let session2 = self.session.clone();
        let stream = self.find_available_alternative(track_id)
            .map(|track| track.unwrap().find_file(metadata::FileFormat::OGG_VORBIS_320).unwrap())
            .and_then(move |file_id| {
                session.audio_key()
                    .request(track_id, file_id)
                    .map(move |key| (file_id, key))
            })
            .and_then(move |(file_id, key)| {
                let file = AudioFile::new(session2, file_id);
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
        trace!("polling player state={:?} queue={}/{}", self.state,
               self.queue.size(), self.queue.capacity());

        use self::PlayerState::*;
        loop {
            match self.state {
                Stopped => return Ok(Async::NotReady),
                Loading => {
                    try_ready!(self.fill_queue());

                    debug!("loaded, start playing");
                    self.sink.start()?;
                    self.state = PlayerState::Loaded;
                    return Ok(Async::NotReady);
                }
                Loaded => {
                    if self.queue.underrun() {
                        warn!("player underrun");
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
