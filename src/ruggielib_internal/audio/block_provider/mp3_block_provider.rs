use std::{fs::File, io::BufReader};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};
use nanomp3::{Channels, Reader};

use crate::audio::errors::AudioError;

pub struct Mp3BlockProvider{
    reader: Reader<BufReader<File>, i16>
}

impl Mp3BlockProvider{
    pub fn new(file_handle: File) -> Self{
        let reader = Reader::<_, i16>::new(BufReader::new(file_handle)).unwrap();

        Mp3BlockProvider {
            reader
        }
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, pcm_slice_i16: &mut [i16]) -> Result<(), AudioError>{

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

    pub fn num_channels(&self) -> u16{
        match self.reader.channels(){
            Some(channels) => channels.num() as u16,
            None => 0
        }
    }
}