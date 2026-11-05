use std::{fs::File, io::BufReader};

use ctru::{linear::LinearAllocator, services::ndsp::AudioFormat};
use nanomp3::{Channels, Reader};

///Manages loading, unloading and data from audio.
pub struct BlockProvider{
    reader: Reader<BufReader<File>, u8>
}

impl BlockProvider{

    //Picked from symphonia's basic example
    pub fn new(file_handle: File) -> Self{
        let reader = Reader::<_, u8>::new(BufReader::new(file_handle)).unwrap();

        BlockProvider{
            reader
        }
    }

    ///Fills the buffer with the next block. Returns false if the sound needs to be stopped
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>){
        self.reader.read(buffer);
    }

    pub fn get_sample_rate(&self) -> u32{
        self.reader.sample_rate()
    }

    pub fn get_audio_format(&self) -> AudioFormat{
        let channels = self.reader.channels().unwrap();
        
        match channels{
            Channels::Mono =>{
                AudioFormat::PCM8Mono
            },

            Channels::Stereo =>{
                AudioFormat::PCM8Stereo
            }
        }
    }
}