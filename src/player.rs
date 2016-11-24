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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PlayerState {
    Stopped,
    Loading(bool),
    Loaded,
    Playing,
}

pub type AudioStream = ogg_async::OggStream<Subfile<AudioDecrypt<AudioFile>>>;
pub struct Player {
    session: Session,
    state: PlayerState,

    stream: Option<AudioStream>,
    future: SpFuture<'static, AudioStream>,
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
            future: futures::empty().sp_boxed(),
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

        let future = self.find_available_alternative(track_id)
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
            })
            .sp_boxed();

        debug!("loading track {:?}", track_id);

        if self.state == PlayerState::Loaded {
            self.sink.pause().unwrap();
        }
        self.queue.clear();
        self.packet = Vec::new();
        self.offset = 0;
        self.stream = None;
        self.future = future;
        self.state = PlayerState::Loading(false);
    }

    pub fn stop(&mut self) {
        if self.state == PlayerState::Playing {
            self.sink.pause().unwrap();
        }

        self.queue.clear();
        self.packet = Vec::new();
        self.offset = 0;
        self.stream = None;
        self.future = futures::empty().sp_boxed();
        self.state = PlayerState::Stopped;
    }

    pub fn pause(&mut self) {
        match self.state {
            PlayerState::Playing => {
                self.sink.pause().unwrap();
                self.state = PlayerState::Loaded;
            }
            PlayerState::Loaded => {
                info!("called pause when already paused");
            },
            PlayerState::Loading(ref mut play) => {
                *play = false;
            }
            s => {
                warn!("called pause from invalid state {:?}", s);
            }
        }
    }

    pub fn play(&mut self) {
        match self.state {
            PlayerState::Playing => {
                info!("called play when already playing");
            }
            PlayerState::Loaded => {
                self.sink.start().unwrap();
                self.state = PlayerState::Playing;
            },
            PlayerState::Loading(ref mut play) => {
                *play = true;
            }
            s => {
                warn!("called play from invalid state {:?}", s);
            }
        }
    }

    fn stream(&mut self) -> Poll<&mut AudioStream, SpError> {
        assert_ne!(self.state, PlayerState::Stopped);

        if self.stream.is_none() {
            let stream = try_ready!(self.future.poll());
            self.stream = Some(stream);
        }

        Ok(Async::Ready(self.stream.as_mut().unwrap()))
    }

    fn fill_queue(&mut self) -> Poll<Option<PlayerEvent>, SpError> {
        loop {
            while self.offset < self.packet.len() {
                let count = self.queue.try_write(&self.packet[self.offset..]);
                self.offset += count;

                if count == 0 {
                    return Ok(Async::Ready(None));
                }
            }

            self.packet = Vec::new();
            self.offset = 0;

            let poll = try_ready!(try_ready!(self.stream()).poll());
            match poll {
                Some(packet) => {
                    self.packet = packet;
                    self.offset = 0;
                }
                None => {
                    return Ok(Async::Ready(Some(PlayerEvent::TrackEnd)));
                }
            }
        }
    }
}

pub enum PlayerEvent {
    TrackEnd,
    Playing(u32),
}

impl Stream for Player {
    type Item = PlayerEvent;
    type Error = SpError;

    fn poll(&mut self) -> Poll<Option<PlayerEvent>, SpError> {
        trace!("polling player state={:?} queue={}/{}", self.state,
               self.queue.size(), self.queue.capacity());

        use self::PlayerState::*;
        loop {
            match self.state {
                Stopped => return Ok(Async::NotReady),
                Loading(play) => {
                    if self.queue.size() > self.queue.capacity() / 2 {
                        if play {
                            debug!("loaded, start playing");
                            self.sink.start()?;
                            self.state = Playing;
                        } else {
                            debug!("loaded, stay paused");
                            self.state = Loaded;
                        }
                    }
                }
                Loaded => (),
                Playing => {
                    if self.queue.underrun() {
                        warn!("player underrun");
                        self.sink.pause().unwrap();
                        self.state = Loading(true);
                    }
                }
            }

            let event = try_ready!(self.fill_queue());
            match (event, self.state) {
                (Some(PlayerEvent::TrackEnd), _) => {
                    self.stop();
                    return Ok(Async::Ready(Some(PlayerEvent::TrackEnd)));
                }

                (None, Playing) => {
                    let absgp = try_ready!(self.stream()).last_absgp();
                    let position = (absgp * 1000 / 44_100) as u32;
                    return Ok(Async::Ready(Some(PlayerEvent::Playing(position))));
                },

                (None, Loaded) => return Ok(Async::Ready(None)),

                _ => (),
            }
        }
    }
}
