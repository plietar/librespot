use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::sync::mpsc;
use futures::{Future, Stream, BoxFuture, IntoFuture, Poll, Async};
use std::io;
use std::result::Result;
use std::str::FromStr;
use std::sync::{RwLock, Arc, Weak};
use tokio_core::io::EasyBuf;
use tokio_core::reactor::{Handle, Remote};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use uuid::Uuid;

use apresolve::apresolve_or_fallback;
use authentication::Credentials;
use cache::Cache;
use component::Lazy;
use connection;
use version;

use audio_key::AudioKeyManager;
use channel::ChannelManager;
use mercury::MercuryManager;
use metadata::MetadataManager;
use audio_file::AudioFileManager;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}
impl FromStr for Bitrate {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Bitrate::Bitrate96),
            "160" => Ok(Bitrate::Bitrate160),
            "320" => Ok(Bitrate::Bitrate320),
            _ => Err(s.into()),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub user_agent: String,
    pub device_id: String,
    pub bitrate: Bitrate,
    pub onstart: Option<String>,
    pub onstop: Option<String>,
    pub discovery_port: Option<u16>
}

impl Default for Config {
    fn default() -> Config {
        let device_id = Uuid::new_v4().hyphenated().to_string();
        Config {
            user_agent: version::version_string(),
            device_id: device_id,
            bitrate: Bitrate::Bitrate160,
            onstart: None,
            onstop: None,
            discovery_port: None,
        }
    }
}

pub struct SessionData {
    country: String,
    canonical_username: String,
}

pub struct SessionInternal {
    config: Config,
    data: RwLock<SessionData>,

    tx_connection: mpsc::UnboundedSender<(u8, Vec<u8>)>,

    audio_key: Lazy<AudioKeyManager>,
    audio_file: Lazy<AudioFileManager>,
    channel: Lazy<ChannelManager>,
    mercury: Lazy<MercuryManager>,
    metadata: Lazy<MetadataManager>,
    cache: Option<Arc<Cache>>,

    handle: Remote,

    session_id: usize,
}

static SESSION_COUNTER : AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

pub fn device_id(name: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(name);
    h.result_str()
}

impl Session {
    pub fn connect(config: Config, credentials: Credentials,
                   cache: Option<Cache>, handle: Handle)
        -> Box<Future<Item=Session, Error=io::Error>>
    {
        let access_point = apresolve_or_fallback::<io::Error>(&handle);

        let handle_ = handle.clone();
        let connection = access_point.and_then(move |addr| {
            info!("Connecting to AP \"{}\"", addr);
            connection::connect::<&str>(&addr, &handle_)
        });

        let device_id = config.device_id.clone();
        let authentication = connection.and_then(move |connection| {
            connection::authenticate(connection, credentials, device_id)
        });

        let result = authentication.map(move |(transport, reusable_credentials)| {
            info!("Authenticated as \"{}\" !", reusable_credentials.username);
            if let Some(ref cache) = cache {
                cache.save_credentials(&reusable_credentials);
            }

            let (session, task) = Session::create(
                &handle, transport, config, cache, reusable_credentials.username.clone()
            );

            handle.spawn(task.map_err(|e| panic!(e)));

            session
        });
        
        Box::new(result)
    }

    fn create(handle: &Handle, transport: connection::Transport,
              config: Config, cache: Option<Cache>, username: String)
        -> (Session, BoxFuture<(), io::Error>)
    {
        let (sink, stream) = transport.split();

        let (sender_tx, sender_rx) = mpsc::unbounded();
        let session_id = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);

        debug!("new Session[{}]", session_id);

        let session = Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: username,
            }),

            tx_connection: sender_tx,

            cache: cache.map(Arc::new),

            audio_key: Lazy::new(),
            audio_file: Lazy::new(),
            channel: Lazy::new(),
            mercury: Lazy::new(),
            metadata: Lazy::new(),

            handle: handle.remote().clone(),

            session_id: session_id,
        }));

        let sender_task = sender_rx
            .map_err(|e| -> io::Error { panic!(e) })
            .forward(sink).map(|_| ());
        let receiver_task = DispatchTask(stream, session.weak());

        let task = (receiver_task, sender_task).into_future()
            .map(|((), ())| ()).boxed();

        (session, task)
    }

    pub fn audio_key(&self) -> &AudioKeyManager {
        self.0.audio_key.get(|| AudioKeyManager::new(self.weak()))
    }

    pub fn audio_file(&self) -> &AudioFileManager {
        self.0.audio_file.get(|| AudioFileManager::new(self.weak()))
    }

    pub fn channel(&self) -> &ChannelManager {
        self.0.channel.get(|| ChannelManager::new(self.weak()))
    }

    pub fn mercury(&self) -> &MercuryManager {
        self.0.mercury.get(|| MercuryManager::new(self.weak()))
    }

    pub fn metadata(&self) -> &MetadataManager {
        self.0.metadata.get(|| MetadataManager::new(self.weak()))
    }

    pub fn spawn<F, R>(&self, f: F)
        where F: FnOnce(&Handle) -> R + Send + 'static,
              R: IntoFuture<Item=(), Error=()>,
              R::Future: 'static
    {
        self.0.handle.spawn(f)
    }

    fn debug_info(&self) {
        debug!("Session[{}] strong={} weak={}",
               self.0.session_id, Arc::strong_count(&self.0), Arc::weak_count(&self.0));
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    fn dispatch(&self, cmd: u8, data: EasyBuf) {
        match cmd {
            0x4 => {
                self.debug_info();
                self.send_packet(0x49, data.as_ref().to_owned());
            },
            0x4a => (),
            0x1b => {
                let country = String::from_utf8(data.as_ref().to_owned()).unwrap();
                info!("Country: {:?}", country);
                self.0.data.write().unwrap().country = country;
            }

            0x9 | 0xa => self.channel().dispatch(cmd, data),
            0xd | 0xe => self.audio_key().dispatch(cmd, data),
            0xb2...0xb6 => self.mercury().dispatch(cmd, data),
            _ => (),
        }
    }

    pub fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.0.tx_connection.send((cmd, data)).unwrap();
    }

    pub fn cache(&self) -> Option<&Arc<Cache>> {
        self.0.cache.as_ref()
    }

    pub fn config(&self) -> &Config {
        &self.0.config
    }

    pub fn username(&self) -> String {
        self.0.data.read().unwrap().canonical_username.clone()
    }

    pub fn country(&self) -> String {
        self.0.data.read().unwrap().country.clone()
    }

    pub fn device_id(&self) -> &str {
        &self.config().device_id
    }

    pub fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }

    pub fn session_id(&self) -> usize {
        self.0.session_id
    }
}

#[derive(Clone)]
pub struct SessionWeak(pub Weak<SessionInternal>);

impl SessionWeak {
    pub fn try_upgrade(&self) -> Option<Session> {
        self.0.upgrade().map(Session)
    }

    pub fn upgrade(&self) -> Session {
        self.try_upgrade().expect("Session died")
    }
}

impl Drop for SessionInternal {
    fn drop(&mut self) {
        debug!("drop Session[{}]", self.session_id);
    }
}

struct DispatchTask<S>(S, SessionWeak)
    where S: Stream<Item = (u8, EasyBuf)>;

impl <S> Future for DispatchTask<S>
    where S: Stream<Item = (u8, EasyBuf)>
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let session = match self.1.try_upgrade() {
            Some(session) => session,
            None => {
                return Ok(Async::Ready(()))
            },
        };

        loop {
            let (cmd, data) = try_ready!(self.0.poll()).expect("connection closed");
            session.dispatch(cmd, data);
        }
    }
}

impl <S> Drop for DispatchTask<S>
    where S: Stream<Item = (u8, EasyBuf)>
{
    fn drop(&mut self) {
        debug!("drop Dispatch");
    }
}
