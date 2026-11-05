use std::{fs::File, io::BufReader};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};
use nanomp3::{Channels, Reader};

use crate::audio::errors::AudioError;

pub struct Mp3BlockProvider{
    reader: Reader<BufReader<File>, i16>
}

impl Mp3BlockProvider{
    //Picked from symphonia's basic example
    pub fn new(file_handle: File) -> Self{
        let reader = Reader::<_, i16>::new(BufReader::new(file_handle)).unwrap();

        Mp3BlockProvider {
            reader
        }
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>) -> Result<(), AudioError>{

        //Unsafe fast cast from u8 to i16 without copying
        let pcm_slice_i16: &mut [i16] = unsafe {
            std::slice::from_raw_parts_mut(
                buffer.as_mut_ptr() as *mut i16,
                buffer.len() / std::mem::size_of::<i16>(),
            )
        };

        self.reader.read(pcm_slice_i16).map_err(|_| AudioError::NextBlock)?;

        Ok(())
    }

    pub fn get_sample_rate(&self) -> u32{
        self.reader.sample_rate()
    }

    pub fn get_audio_format(&self) -> AudioFormat{
        let channels = self.reader.channels();
        
        match channels{
            Some(Channels::Mono) =>{
                AudioFormat::PCM16Mono
            },

            Some(Channels::Stereo) =>{
                AudioFormat::PCM16Stereo
            },
            None =>{
                AudioFormat::PCM8Mono
            }
        }
    }
}