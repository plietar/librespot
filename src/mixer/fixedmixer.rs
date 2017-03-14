use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::Mixer;

#[derive(Clone)]
pub struct FixedMixer {
  volume: Arc<AtomicUsize>
}

impl Mixer for FixedMixer {
    fn open() -> FixedMixer {
        FixedMixer {
            volume: Arc::new(AtomicUsize::new(0xFFFF))
        }
    }
    fn start(&self) {
    }
    fn stop(&self) {
    }
    fn volume(&self) -> u16 {
        self.volume.load(Ordering::Relaxed) as u16
    }
    fn set_volume(&self, volume: u16) {
        self.volume.store(volume as usize, Ordering::Relaxed);
        debug!("volume {}", volume)
    }
}
