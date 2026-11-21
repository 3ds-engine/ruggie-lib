use std::{ffi::CString, ptr::{null, null_mut}};

use ctru_sys::R_SUCCEEDED;
use opendsp_sys::*;

use crate::audio::{errors::RuggieAudioHandleError, streamed_sound::StreamedSound};

const OPENDSP_INITIAL_AUDIO_CONFIG: opendspAudioConfig = opendspAudioConfig{
    struct_size: std::mem::size_of::<opendspAudioConfig>() as u32,
    channel: OPENDSP_AUDIO_CHANNEL_AUTO, 
    buffer_count: 0, 
    buffer_frames: 0, 
    aac_backend: OPENDSP_AAC_BACKEND_ARM11, 
    flags: 0
};

pub struct RuggieAudioHandle{
}

impl RuggieAudioHandle{
    pub fn new() -> std::result::Result<Self, RuggieAudioHandleError>{
        unsafe{
            let mut res = opendspInit();

            if !R_SUCCEEDED(res){
                return Err(RuggieAudioHandleError::Initialization)
            }

            //Initialize all 24 channels
            res = opendspAudioInit(0b111111111111111111111111);

            if !R_SUCCEEDED(res){
                return Err(RuggieAudioHandleError::Initialization)
            }
        }
        Ok(RuggieAudioHandle { })
    }

    pub fn load_streamed_sound(&self, path: &str) -> std::result::Result<StreamedSound, ()>{
        let mut music: *mut opendspAudioStream = null_mut();
        let config = OPENDSP_INITIAL_AUDIO_CONFIG;
        let c_path = CString::new(path).map_err(|_| ())?;

        unsafe{
            let res = opendspAudioOpen(&mut music, c_path.as_ptr(), &config);
            if !R_SUCCEEDED(res) || music == null_mut(){ return Err(())}

        }

        Ok(StreamedSound::new(music))
    }
    
    pub fn play_streamed_audio(&mut self, streamed_sound: &StreamedSound) -> std::result::Result<(), ()>{
        unsafe{
            if self.is_paused(streamed_sound){
                let res = opendspAudioResume(streamed_sound.music);
                if !R_SUCCEEDED(res){ return Err(())}
            }
            else{
                let res = opendspAudioPlay(streamed_sound.music);
                if !R_SUCCEEDED(res){ return Err(())}
            }

        }
        Ok(())
    }

    pub fn pause_streamed_audio(&mut self, streamed_sound: &StreamedSound) -> std::result::Result<(), ()> {
        unsafe{
            let res = opendspAudioPause(streamed_sound.music);
            if !R_SUCCEEDED(res){ return Err(())}
        }
        Ok(())
    }

    pub fn is_paused(&self, streamed_sound: &StreamedSound) -> bool {
        unsafe{
            let state = opendspAudioGetState(streamed_sound.music);
            return state == opendspAudioState_OPENDSP_AUDIO_PAUSED
        }
    }
}

impl Drop for RuggieAudioHandle{
    fn drop(&mut self) {
        unsafe{
            opendspExit();
        }
    }
}