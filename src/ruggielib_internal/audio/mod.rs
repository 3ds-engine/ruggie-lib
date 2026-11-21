use crate::{RuggieLib, audio::streamed_sound::StreamedSound};

pub mod ruggie_audio_handle;
pub mod sound;
pub mod streamed_sound;
pub mod errors;

impl RuggieLib{

    
    pub fn load_streamed_sound(&mut self, path: &str) -> std::result::Result<StreamedSound, ()>{
        self.audio_handler.load_streamed_sound(path)
    }

    pub fn play_streamed_sound(&mut self, streamed_sound: &StreamedSound) -> std::result::Result<(),()>{
        self.audio_handler.play_streamed_audio(streamed_sound)
    }

    pub fn pause_streamed_sound(&mut self, streamed_sound: &StreamedSound) -> std::result::Result<(),()>{
        self.audio_handler.pause_streamed_audio(streamed_sound)
    }

    pub fn is_sound_paused(&self, streamed_sound: &StreamedSound) -> bool{
        self.audio_handler.is_paused(streamed_sound)
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