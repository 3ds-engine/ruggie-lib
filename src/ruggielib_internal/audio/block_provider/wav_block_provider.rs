use std::{fs::File, io::BufReader};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};
use hound::WavReader;

use crate::audio::errors::{AudioError, SoundInitializationError, SoundReadingError};

pub struct WavBlockProvider{
    reader: WavReader<BufReader<File>>
}

impl WavBlockProvider{
    pub fn new(file_handle: File) -> Result<Self, SoundInitializationError>{
        let reader = WavReader::new(BufReader::new(file_handle)).map_err(|_| SoundInitializationError::ReaderInitialization)?;

        Ok(WavBlockProvider { reader })
    }

    ///Fills the buffer with the next block.
    pub fn fill_with_next_block(&mut self, pcm_slice_i16: &mut [i16]) -> Result<(), SoundReadingError>{
        let mut samples_iter = self.reader.samples::<i16>();

        //Fill slice with iter
        for elem in pcm_slice_i16.iter_mut(){
            match samples_iter.next(){
                Some(val) =>{
                    let val = val.map_err(|_| SoundReadingError::NextBlock)?;
                    *elem = val;
                },
                None =>{
                    *elem = 0;
                }
            }
        }

        Ok(())
    }

    pub fn get_sample_rate(&self) -> u32{
        self.reader.spec().sample_rate
    }

    pub fn get_audio_format(&self) -> AudioFormat{
        match self.reader.spec().channels {
            0 =>AudioFormat::PCM8Mono,
            1 => AudioFormat::PCM16Mono,
            _ => AudioFormat::PCM16Stereo
        }
    }

    pub fn num_channels(&self) -> u16{
        self.reader.spec().channels
    }

    pub fn fill_with_whole_data(&mut self, buffer: &mut Vec<i16>) -> Result<(), SoundReadingError>{
        let mut samples_iter = self.reader.samples::<i16>();
        let total_samples = samples_iter.len();

        buffer.resize(total_samples, 0);

        //Fill slice with iter
        for elem in buffer.iter_mut(){
            match samples_iter.next(){
                Some(val) =>{
                    let val = val.map_err(|_| SoundReadingError::WholeSoundRead)?;
                    *elem = val;
                },
                None =>{
                    return Err(SoundReadingError::WholeSoundRead)
                }
            }
        }

        Ok(())
    }
}