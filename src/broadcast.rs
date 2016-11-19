use futures::{Async, Poll, Stream};
use futures::task::{Task, park};
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

pub struct BroadcastSender<T: Clone> {
    shared: Arc<Mutex<BroadcastShared<T>>>,
}

pub struct BroadcastReceiver<T: Clone> {
    seq: u64,
    id: u64,
    shared: Arc<Mutex<BroadcastShared<T>>>,
}

struct BroadcastShared<T: Clone> {
    next_id: u64,
    seq: u64,
    value: Option<T>,
    tasks: HashMap<u64, Task>,
}

impl<T: Clone> BroadcastSender<T> {
    pub fn send(&self, state: T) {
        let mut shared = self.shared.lock().unwrap();
        shared.seq += 1;
        shared.value = Some(state);
        for task in shared.tasks.values() {
            task.unpark();
        }
    }
}

impl<T: Clone> Stream for BroadcastReceiver<T> {
    type Item = T;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<T>, io::Error> {
        let mut shared = self.shared.lock().unwrap();

        if shared.seq > self.seq {
            self.seq = shared.seq;
            shared.tasks.remove(&self.id);

            let value = shared.value.as_ref().expect("Missing state value").clone();
            Ok(Async::Ready(Some(value)))
        } else {
            let task = park();
            shared.tasks.insert(self.id, task);

            Ok(Async::NotReady)
        }
    }
}

impl<T: Clone> Clone for BroadcastReceiver<T> {
    fn clone(&self) -> BroadcastReceiver<T> {
        let mut shared = self.shared.lock().unwrap();
        let id = shared.next_id;
        shared.next_id += 1;

        BroadcastReceiver {
            id: id,
            seq: self.seq,
            shared: self.shared.clone(),
        }
    }
}

pub fn broadcast<T: Clone>() -> (BroadcastSender<T>, BroadcastReceiver<T>) {
    let shared = Arc::new(Mutex::new(BroadcastShared {
        next_id: 1,
        seq: 0,
        value: None,
        tasks: HashMap::new(),
    }));

    let sender = BroadcastSender { shared: shared.clone() };
    let receiver = BroadcastReceiver {
        id: 0,
        seq: 0,
        shared: shared,
    };

    (sender, receiver)
}
