use super::Mixer;
use super::AudioFilter;

use alsa;

#[derive(Clone)]
pub struct AlsaMixer {
    name: String
}

impl Mixer for AlsaMixer {
    fn open(device: Option<String>) -> AlsaMixer {
        let name = device.unwrap_or("default".to_string());
        AlsaMixer {
            name: name
        }
    }

    fn start(&self) {
    }

    fn stop(&self) {
    }

    fn volume(&self) -> u16 {
        let mixer = alsa::mixer::Mixer::new(&self.name, false).unwrap();
        let selem_id = alsa::mixer::SelemId::new("Master", 0);
        let selem = mixer.find_selem(&selem_id).unwrap();
        let (min, max) = selem.get_playback_volume_range();
        let volume: i64 = selem.get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft).unwrap();

        // Spotify uses a volume range from 0 to 65535, but the ALSA mixers resolution might
        // differ, e.g. most ALSA mixers uses a resolution of 256. Therefore, we have to calculate
        // the multiplier to use, to get the corresponding Spotify volume value from the ALSA
        // mixers volume.
        let resolution = max - min + 1;
        let multiplier: u16 = (((0xFFFF + 1) / resolution) - 1) as u16;

        volume as u16 * multiplier
    }

    fn set_volume(&self, volume: u16) {
        let mixer = alsa::mixer::Mixer::new(&self.name, false).unwrap();
        let selem_id = alsa::mixer::SelemId::new("Master", 0);
        let selem = mixer.find_selem(&selem_id).unwrap();
        let (min, max) = selem.get_playback_volume_range();

        // Spotify uses a volume range from 0 to 65535, but the ALSA mixers resolution might
        // differ, e.g. most ALSA mixers uses a resolution of 256. Therefore, we have to calculate
        // the factor to use, to get the corresponding ALSA mixers volume value from the Spotify
        // volume.
        let resolution = max - min + 1;
        let factor: u16 = (((0xFFFF + 1) / resolution) - 1) as u16;
        let volume: i64 = (volume / factor) as i64;

        selem.set_playback_volume_all(volume).unwrap();
    }

    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}
