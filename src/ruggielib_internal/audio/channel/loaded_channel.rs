use std::rc::{Rc, Weak};

use ctru::{linear::LinearAllocator, services::ndsp::{Ndsp, wave::Wave}};

use crate::audio::{errors::AudioError, loaded_audio::LoadedAudio};

pub struct LoadedChannel{
    channel_index: u8,
    wave: Wave<Rc<[u8], LinearAllocator>>
}

impl LoadedChannel{
    /*pub fn new(channel_index: u8, loaded_audio: &LoadedAudio, ndsp: &mut Ndsp) -> Result<Self, AudioError>{
        
    }

    pub fn enqueue_buffers(&mut self, ndsp: &Ndsp) -> Result<(), AudioError>{
    }

    pub fn finished(&mut self, ndsp: &Ndsp) -> Result<(), AudioError>{

    }
    */
}