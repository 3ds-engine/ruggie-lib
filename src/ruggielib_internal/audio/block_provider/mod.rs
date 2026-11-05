use std::{ffi::OsStr, fs::File, path::Path};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};


use crate::audio::{block_provider::{BlockProvider::MP3, mp3_block_provider::Mp3BlockProvider}, errors::{AudioError, SoundInitializationError::{self, BadExtension}}};

pub mod mp3_block_provider;

///Manages loading, unloading and data from audio.
pub enum BlockProvider{
    MP3(Mp3BlockProvider)
}

impl BlockProvider{

    //Picked from symphonia's basic example
    pub fn new(path: &Path) -> Result<Self, SoundInitializationError>{
        //Load file as a handle
        let file = File::open(path).map_err(|_| SoundInitializationError::WrongPath)?;

        match path.extension().and_then(OsStr::to_str){
            Some("mp3") =>{
                let bp = Mp3BlockProvider::new(file);
                Ok(MP3(bp))
            },
            _ =>{
                Err(BadExtension)
            }
        }
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>) -> Result<(), AudioError>{

        match self{
            MP3(block_provider) =>{
                block_provider.fill_with_next_block(buffer)
            }
        }
    }

    pub fn get_sample_rate(&self) -> u32{
        match self{
            MP3(block_provider) =>{
                block_provider.get_sample_rate()
            }
        }
    }

    pub fn get_audio_format(&self) -> AudioFormat{
        match self{
            MP3(block_provider) =>{
                block_provider.get_audio_format()
            }
        }
    }
}