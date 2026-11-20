use crate::{RuggieLib, audio::sound::Sound};

pub mod ruggie_audio_handle;
pub mod sound;
pub mod errors;

impl RuggieLib{

    
    pub fn load_sound(&mut self, path: &str){

    }

    pub fn play_sound(&mut self, sound: &str) -> std::result::Result<(),()>{
        self.audio_handler.play_audio(sound)
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