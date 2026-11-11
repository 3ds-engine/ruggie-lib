use std::{fs::File, io::BufReader};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};
use nanomp3::{Channels, Reader};

use crate::audio::errors::{AudioError, SoundInitializationError, SoundReadingError};

pub struct Mp3BlockProvider{
    reader: Reader<BufReader<File>, i16>
}

impl Mp3BlockProvider{
    pub fn new(file_handle: File) -> Result<Self, SoundInitializationError>{
        let reader = Reader::<_, i16>::new(BufReader::new(file_handle)).map_err(|_| SoundInitializationError::ReaderInitialization)?;

        Ok(Mp3BlockProvider {
            reader
        })
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, pcm_slice_i16: &mut [i16]) -> Result<(), SoundReadingError>{

        self.reader.read(pcm_slice_i16).map_err(|_| SoundReadingError::NextBlock)?;

        Ok(())
    }

    pub fn fill_with_whole_data(&mut self, pcm_i16: &mut Vec<i16>) -> Result<(), SoundReadingError>{
        if let Some(sample_count) = self.reader.total_samples(){

            pcm_i16.resize(sample_count as usize, 0);
            self.reader.read(pcm_i16).map_err(|_| SoundReadingError::WholeSoundRead)?;
            
            return Ok(())
        }
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