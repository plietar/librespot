use futures::task::{self, Task};
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use bounded_spsc_queue::{self as queue, Consumer, Producer};

pub struct Signaling {
    writer: Mutex<Option<Task>>,
    underrun: AtomicBool,
}

pub struct AudioConsumer<T> {
    queue: Consumer<T>,
    signaling: Arc<Signaling>,
}

pub struct AudioProducer<T> {
    queue: Producer<T>,
    signaling: Arc<Signaling>,
}

pub fn make<T>(size: usize) -> (AudioProducer<T>, AudioConsumer<T>) {
    let (producer, consumer) = queue::make(size);
    let signaling = Arc::new(Signaling {
        writer: Mutex::new(None),
        underrun: AtomicBool::new(false),
    });

    (AudioProducer {
        queue: producer,
        signaling: signaling.clone(),
    },
     AudioConsumer {
        queue: consumer,
        signaling: signaling,
    })
}

impl<T> AudioProducer<T> {
    pub fn free_space(&self) -> usize {
        self.queue.free_space()
    }

    pub fn size(&self) -> usize {
        self.queue.size()
    }

    pub fn capacity(&self) -> usize {
        self.queue.capacity()
    }

    pub fn underrun(&mut self) -> bool {
        self.signaling.underrun.swap(false, Ordering::Relaxed)
    }

    pub fn clear(&mut self) {
        self.queue.clear()
    }
}

impl<T: Copy> AudioProducer<T> {
    pub fn try_write(&self, data: &[T]) -> usize {
        let count = self.queue.try_write(data);

        self.signaling.park_writer();

        count
    }
}

impl AudioConsumer<i16> {
    pub fn read(&self, data: &mut [i16]) {
        let count = self.queue.try_read(data);
        if count < data.len() {
            trace!("Underrun");
            self.signaling.underrun.store(true, Ordering::Relaxed);

            self.signaling.unpark_writer();
            for i in count..data.len() {
                data[i] = 0;
            }
        }

        if self.queue.size() < self.queue.capacity() / 2 {
            trace!("requesting more data {}/{}",
                   self.queue.size(), self.queue.capacity());
            self.signaling.unpark_writer();
        }
    }
}

impl<T> AudioConsumer<T> {
    pub fn size(&self) -> usize {
        self.queue.size()
    }

    pub fn unpark(&self) {
        self.signaling.unpark_writer();
    }
}

impl Signaling {
    fn unpark_writer(&self) {
        if let Some(ref writer) = *self.writer.lock().unwrap() {
            writer.unpark()
        }
    }

    fn park_writer(&self) {
        *self.writer.lock().unwrap() = Some(task::park());
    }
}
