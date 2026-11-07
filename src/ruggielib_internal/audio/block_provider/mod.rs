use std::{ffi::OsStr, fs::File, path::Path};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};


use crate::audio::{block_provider::{self, BlockProvider::{Mp3, Wav}, mp3_block_provider::Mp3BlockProvider, wav_block_provider::WavBlockProvider}, errors::{AudioError, SoundInitializationError::{self, BadExtension}}};

pub mod mp3_block_provider;
pub mod wav_block_provider;

///Manages loading, unloading and data from audio.
pub enum BlockProvider{
    Mp3(Mp3BlockProvider),
    Wav(WavBlockProvider)
}

impl BlockProvider{

    //Picked from symphonia's basic example
    pub fn new(path: &Path) -> Result<Self, SoundInitializationError>{
        //Load file as a handle
        let file = File::open(path).map_err(|_| SoundInitializationError::WrongPath)?;

        match path.extension().and_then(OsStr::to_str){
            Some("mp3") =>{
                let bp = Mp3BlockProvider::new(file);
                Ok(Mp3(bp))
            },
            Some("wav") => {
                let bp = WavBlockProvider::new(file);
                Ok(Wav(bp))
            }
            _ =>{
                Err(BadExtension)
            }
        }
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>) -> Result<(), AudioError>{

        //Unsafe fast cast from u8 to i16 without copying. Better to operate with i16 vector than with raw byte vector
        let pcm_slice_i16: &mut [i16] = unsafe {
            std::slice::from_raw_parts_mut(
                buffer.as_mut_ptr() as *mut i16,
                buffer.len() / std::mem::size_of::<i16>(),
            )
        };

        match self{
            Mp3(block_provider) => block_provider.fill_with_next_block(pcm_slice_i16),
            Wav(block_provider) => block_provider.fill_with_next_block(pcm_slice_i16)
        }
    }

    pub fn get_sample_rate(&self) -> u32{
        match self{
            Mp3(block_provider) => block_provider.get_sample_rate(),
            Wav(block_provider) => block_provider.get_sample_rate()
        }
    }

    pub fn get_audio_format(&self) -> AudioFormat{
        match self{
            Mp3(block_provider) => block_provider.get_audio_format(),
            Wav(block_provider) => block_provider.get_audio_format()
        }
    }
}