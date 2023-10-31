#[cfg(feature = "audio")]
mod test;

use crate::apu::{device::Audio, samples::SamplesMutex};
use sdl2::{
    audio::{AudioCallback, AudioDevice as SdlAudioDevice, AudioFormatNum, AudioSpecDesired},
    AudioSubsystem,
};

/// Audio callback.
pub struct Callback<D: Audio>(SamplesMutex<D>);

impl<D> AudioCallback for Callback<D>
    where D: Audio + 'static,
          D::Sample: AudioFormatNum
{
    type Channel = D::Sample;

    fn callback(&mut self, samples: &mut [Self::Channel]) {
        let lock = self.0.lock();
        for (i, sample) in lock.take(samples.len()).enumerate() {
            samples[i] = sample;
        }
    }
}

/// Wraps APU in an SDL audio device.
///
/// # Panic
/// Panics if the device can't support the emulated sound.
pub fn create_device<D>(audio: &AudioSubsystem,
                        samples: SamplesMutex<D>)
                        -> Result<SdlAudioDevice<Callback<D>>, String>
    where D: Audio + 'static,
          D::Sample: AudioFormatNum
{
    let freq = D::sample_rate() as _;
    let channels = if D::mono() { 1 } else { 2 };
    let buffer = freq / 60;
    let spec = AudioSpecDesired { freq: Some(freq),
                                  channels: Some(channels),
                                  samples: Some(buffer as _) };

    audio.open_playback(None, &spec, |spec| {
             assert_eq!(freq, spec.freq,);
             assert_eq!(channels, spec.channels,);
             Callback(samples)
         })
}