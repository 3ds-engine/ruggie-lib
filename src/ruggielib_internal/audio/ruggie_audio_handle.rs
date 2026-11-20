use std::{ffi::CString, ptr::{null, null_mut}};

use ctru_sys::R_SUCCEEDED;
use opendsp_sys::*;

use crate::audio::errors::RuggieAudioHandleError;

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
            let res = opendspInit();

            if !R_SUCCEEDED(res){
                return Err(RuggieAudioHandleError::Initialization)
            }
        }
        Ok(RuggieAudioHandle { })
    }
    
    pub fn play_audio(&mut self, path: &str) -> std::result::Result<(), ()>{
        unsafe{
            let mut res = opendspAudioInit(3);
            if !R_SUCCEEDED(res){ return Err(())}
            println!("1");

            let config = OPENDSP_INITIAL_AUDIO_CONFIG;
            let mut music: *mut opendspAudioStream = null_mut();
            let c_path = CString::new(path).map_err(|_| ())?;
            println!("{}", path);

            res = opendspAudioOpen(&mut music, c_path.as_ptr(), &config);
            if !R_SUCCEEDED(res){ return Err(())}
            println!("2");

            if music == null_mut(){ return Err(())}

            res = opendspAudioPlay(music);
            if !R_SUCCEEDED(res){ return Err(())}
            println!("3");
        }
        Ok(())
    }
}

impl Drop for RuggieAudioHandle{
    fn drop(&mut self) {
        unsafe{
            opendspExit();
        }
    }
}