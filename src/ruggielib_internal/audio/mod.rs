pub mod errors;
pub mod sound;
pub mod streamed_sound;
pub mod channel;
pub mod block_provider;
pub mod audio_handler;
use ctru::services::ndsp::Ndsp;

use crate::{RuggieLib, audio::{errors::AudioError, sound::Sound, streamed_sound::StreamedSound}};

pub(crate) fn initialize_ndsp() -> Result<Ndsp, AudioError>{
    //Initializes ndsp. It expects that DSP firmware is dumped. Later, DSP dumping process should be called if it isnt dumped.
    Ok(Ndsp::new().map_err(|_| AudioError::NdspInitialization)?)
}

impl RuggieLib{
    pub fn audio_update(&self){

    }

    pub fn load_sound(&self) -> Result<Sound, AudioError>{
        Ok(Sound(0))
    }

    pub fn load_streamed_sound(&self) -> Result<StreamedSound, AudioError>{
        Ok(StreamedSound(0))
    }

    pub fn play_sound(&self, sound: &Sound){

    }

    pub fn play_streamed_sound(&self, streamed_sound: &StreamedSound){

    }

    //The variable is consumed as it must no be used after calling this function
    pub fn unload_sound(&self, sound: Sound){

    }

    //The variable is consumed as it must no be used after calling this function
    pub fn unload_streamed_sound(&self, streamed_sound: StreamedSound){

    }
}