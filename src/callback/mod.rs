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
            //println!("{}", sample);
            samples[i] = sample;
        }
    }
}


pub fn create_device<D>(audio: &AudioSubsystem,
                        samples: SamplesMutex<D>)
                        -> Result<SdlAudioDevice<Callback<D>>, String>
    where D: Audio + 'static,
          D::Sample: AudioFormatNum
{
   
    let freq = SAMPLE_RATE as i32;
    let channels = if D::mono() { 1 } else { 2 };
    let buffer = freq / 60;
    let spec = AudioSpecDesired { freq: Some(freq),
                                  channels: Some(channels),
                                  samples: Some(0x200 as u16) };

    audio.open_playback(None, &spec, |spec| {
             assert_eq!(freq, spec.freq,);
             assert_eq!(channels, spec.channels,);
             Callback(samples)
         })
}
const SAMPLER_DIVIDER: u32 = 95;
const SYSCLK_FREQ:      i64 = 0x400000;
pub const SAMPLE_RATE: u32 = SYSCLK_FREQ as u32 / SAMPLER_DIVIDER;