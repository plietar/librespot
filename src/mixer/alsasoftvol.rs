use super::Mixer;
use super::AudioFilter;

use alsa;

#[derive(Clone)]
pub struct AlsaSoftVol {
    name: String
}

impl Mixer for AlsaSoftVol {
    fn open(device: Option<String>) -> AlsaSoftVol {
        let name = device.unwrap_or("default".to_string());
        AlsaSoftVol {
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

        let volume: i64 = selem.get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft).unwrap();

        volume as u16 * 256
    }

    fn set_volume(&self, volume: u16) {
        let volume: i64 = volume as i64 / 256;

        let mixer = alsa::mixer::Mixer::new(&self.name, false).unwrap();
        let selem_id = alsa::mixer::SelemId::new("Master", 0);
        let selem = mixer.find_selem(&selem_id).unwrap();

        selem.set_playback_volume(alsa::mixer::SelemChannelId::FrontLeft, volume).unwrap();
        selem.set_playback_volume(alsa::mixer::SelemChannelId::FrontRight, volume).unwrap();
    }
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}
