use std::{fs::File, io::BufReader};

use ctru::services::ndsp::AudioFormat;
use hound::WavReader;

use crate::audio::errors::AudioError;

pub struct WavBlockProvider{
    reader: WavReader<BufReader<File>>
}

impl WavBlockProvider{
    pub fn new(file_handle: File) -> Self{
        let reader = WavReader::new(BufReader::new(file_handle)).unwrap();

        WavBlockProvider { reader }
    }

    ///Fills the buffer with the next block.
    pub fn fill_with_next_block(&mut self, pcm_slice_i16: &mut [i16]) -> Result<(), AudioError>{
        let mut samples_iter = self.reader.samples::<i16>();

        //Fill slice with iter
        for elem in pcm_slice_i16.iter_mut(){
            match samples_iter.next(){
                Some(val) =>{
                    let val = val.map_err(|_| AudioError::NextBlock)?;
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
}