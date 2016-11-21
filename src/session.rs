use anymap::any::CloneAny;
use anymap;
use futures::Future;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, Weak, Mutex, MutexGuard};
use std::time::Instant;
use tokio::reactor::Handle;
use uuid::Uuid;

use connection::Connection;
use audio_key::AudioKeyManager;
use mercury::MercuryManager;
use channel::ChannelManager;

#[derive(Clone)]
pub struct Session(Arc<SessionInner>);
#[derive(Clone)]
pub struct SessionWeak(Weak<SessionInner>);

pub struct SessionInner {
    components: Components,
    handle: Handle,
    device_id: String,
    country: Mutex<Option<String>>,
    time: Mutex<(u64, Instant)>,
}

impl From<Session> for SessionWeak {
    fn from(session: Session) -> SessionWeak {
        session.weak()
    }
}

impl<'a> From<&'a Session> for SessionWeak {
    fn from(session: &'a Session) -> SessionWeak {
        session.weak()
    }
}

impl Session {
    pub fn new(handle: &Handle) -> Session {
        let device_id = Uuid::new_v4().hyphenated().to_string();
        let session = Session(Arc::new(SessionInner {
            components: Components::new(),
            handle: handle.clone(),
            device_id: device_id,
            country: Mutex::new(None),
            time: Mutex::new((0, Instant::now())),
        }));

        session.add(Connection::new(session.weak()));
        session.add(AudioKeyManager::new(session.weak()));
        session.add(MercuryManager::new(session.weak()));
        session.add(ChannelManager::new(session.weak()));
        session
    }

    pub fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }

    pub fn handle(&self) -> Handle {
        self.0.handle.clone()
    }

    pub fn spawn<F>(&self, task: F)
        where F: Future + 'static,
              F::Error: Debug,
    {
        let task = task.map(|_| ()).map_err(|err| panic!("{:?}", err));
        self.0.handle.spawn(task);
    }

    pub fn add<T: Any + Clone>(&self, component: T) {
        self.0.components.add(component)
    }

    pub fn get<T: Any + Clone>(&self) -> T {
        self.0.components.get()
    }

    pub fn audio_key(&self) -> AudioKeyManager { self.get::<AudioKeyManager>() }
    pub fn connection(&self) -> Connection { self.get::<Connection>() }
    pub fn channel(&self) -> ChannelManager { self.get::<ChannelManager>() }
    pub fn mercury(&self) -> MercuryManager { self.get::<MercuryManager>() }

    pub fn device_id(&self) -> String {
        self.0.device_id.clone()
    }

    pub fn country(&self) -> String {
        self.0.country.lock().unwrap().as_ref()
              .expect("No country set yet").clone()
    }

    pub fn set_country(&self, country: String) {
        *self.0.country.lock().unwrap() = Some(country);
    }

    pub fn time(&self) -> u64 {
        let (timestamp, instant) = *self.0.time.lock().unwrap();
        let elapsed = instant.elapsed();

        let seconds = timestamp + elapsed.as_secs();
        let millis = elapsed.subsec_nanos() / 1_000_000;

        seconds * 1000 + millis as u64
    }

    pub fn update_time(&self, timestamp: u64) {
        *self.0.time.lock().unwrap() = (timestamp, Instant::now());
    }
}

impl SessionWeak {
    pub fn upgrade(&self) -> Session {
        Session(self.0.upgrade().expect("Session died"))
    }
}

pub struct Components(Mutex<anymap::Map<CloneAny>>);
impl Components {
    pub fn new() -> Components {
        Components(Mutex::new(anymap::Map::new()))
    }

    pub fn add<T: Any + Clone>(&self, component: T) {
        #[cfg(nightly)]
        fn error<T>() -> ! {
            panic!("Component already exists: {:?}",
                   std::intrinsics::type_name::<T>());
        }

        #[cfg(not(nightly))]
        fn error<T>() -> ! {
            panic!("Component already exists");
        }

        let mut map = self.0
            .lock()
            .expect("Lock poisoned");

        match map.entry::<T>() {
            anymap::Entry::Occupied(_) => error::<T>(),
            anymap::Entry::Vacant(entry) => {
                entry.insert(component);
            }
        }
    }

    pub fn get<T: Any + Clone>(&self) -> T {
        #[cfg(nightly)]
        fn error_msg<T>() -> String {
            format!("Component not found: {:?}",
                    std::intrinsics::type_name::<T>())
        }

        #[cfg(not(nightly))]
        fn error_msg<T>() -> String {
            String::from("Component not found")
        }

        self.0
            .lock()
            .expect("Lock poisoned")
            .get::<T>()
            .expect(&error_msg::<T>())
            .clone()
    }
}

pub struct Component<T> {
    session: SessionWeak,
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for Component<T> {
    fn clone(&self) -> Self {
        Component {
            session: self.session.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T> Component<T> {
    pub fn create(session: SessionWeak, inner: T) -> Component<T> {
        Component {
            session: session,
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock().expect("Mutex poisoned")
    }

    pub fn session(&self) -> Session {
        self.session.upgrade()
    }
}
