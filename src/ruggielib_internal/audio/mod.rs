pub mod errors;
pub mod sound;
pub mod streamed_sound;
pub mod channel;
pub mod block_provider;
pub mod audio_handler;
pub mod constants;
mod loaded_audio;

use crate::{RuggieLib, audio::{errors::SoundInitializationError, sound::Sound}};

impl RuggieLib{
    pub fn audio_update(&mut self){
        self.audio_handler.update_channels();
    }

    pub fn play_streamed_sound(&mut self, path: &str){
        self.audio_handler.play_streamed_sound(path, 0);
    }

    
    pub fn load_sound(&mut self, path: &str) -> Result<Sound, SoundInitializationError>{
        self.audio_handler.load_sound(path)
    }

    pub fn play_sound(&mut self, sound: &Sound){
        self.audio_handler.play_sound(sound)
    }
    /*
    pub fn load_streamed_sound(&self) -> Result<StreamedSound, AudioError>{
        Ok(StreamedSound(0))
    }


    

    //The variable is consumed as it must no be used after calling this function
    pub fn unload_sound(&self, sound: Sound){

    }

    //The variable is consumed as it must no be used after calling this function
    
    pub fn unload_streamed_sound(&self, streamed_sound: StreamedSound){

    }
    */
}