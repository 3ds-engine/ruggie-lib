use std::rc::{Rc, Weak};

use ctru::{linear::LinearAllocator, services::ndsp::{AudioMix, InterpolationType, Ndsp, wave::{Status, Wave}}};

use crate::audio::{errors::AudioError, loaded_audio::LoadedAudio};

pub struct LoadedChannel{
    channel_index: u8,
    wave: Wave<Rc<[u8], LinearAllocator>>
}

impl LoadedChannel{
    pub fn new(channel_index: u8, loaded_audio: &LoadedAudio, ndsp: &mut Ndsp) -> Result<Self, AudioError>{
        //Wave creation
        let wave = Wave::new(loaded_audio.buffer.clone(), loaded_audio.block_provider.get_audio_format(), false);

        //Channel_initialization
        let mut channel = ndsp.channel(channel_index).map_err(|_| AudioError::NdspChannelReference(channel_index))?;

        let sample_rate = loaded_audio.block_provider.get_sample_rate();
        channel.set_sample_rate(sample_rate as f32);

        channel.set_format(loaded_audio.block_provider.get_audio_format());
        channel.set_interpolation(InterpolationType::Linear);
        channel.set_mix(&AudioMix::default());

        Ok(LoadedChannel { channel_index, wave })
    }

    pub fn enqueue_buffer(&mut self, ndsp: &Ndsp) -> Result<(), AudioError>{
        let mut channel = ndsp.channel(self.channel_index).map_err(|_| AudioError::NdspChannelReference(self.channel_index))?;
        channel.queue_wave(&mut self.wave).map_err(|_| AudioError::WaveEnqueue)?;
        Ok(())
    }

    pub fn finished(&self) -> bool{
        self.wave.status() == Status::Done
    }
}