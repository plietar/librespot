use std::collections::VecDeque;
use futures::{Async, Poll, Future, Sink, StartSend, AsyncSink};

use types::*;
use mercury::MercuryManager;

pub struct MercurySender {
    mercury: MercuryManager,
    uri: String,
    pending: VecDeque<SpFuture<'static, ()>>,
}

impl MercurySender {
    // TODO: pub(super) when stable
    pub fn new(mercury: MercuryManager, uri: String) -> MercurySender {
        MercurySender {
            mercury: mercury,
            uri: uri,
            pending: VecDeque::new(),
        }
    }
}

impl Sink for MercurySender {
    type SinkItem = Vec<u8>;
    type SinkError = SpError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let task = self.mercury.send(self.uri.clone(), item).map(|_| ()).sp_boxed();
        self.pending.push_back(task);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            match self.pending.front_mut() {
                Some(task) => {
                    try_ready!(task.poll());
                }
                None => {
                    return Ok(Async::Ready(()));
                }
            }
            self.pending.pop_front();
        }
    }
}
