use std::{array};

use ctru::services::ndsp::{Ndsp, wave::Status};

use crate::audio::{audio_reader::AudioReader, channel::Channel, errors::AudioError};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler<'ndsp>{
    //Available channels in runtime
    audio_channels: [Channel<'ndsp>; NUMBER_OF_CHANNELS],
}

impl<'ndsp> AudioHandler<'ndsp>{
    pub fn new(ndsp: &'ndsp Ndsp) -> Result<Self, AudioError>{

        let audio_channels: [Channel<'ndsp>; NUMBER_OF_CHANNELS] = std::array::try_from_fn(|i|{
            Channel::new(i as u8, &ndsp)
        })?;

        Ok(AudioHandler { 
            audio_channels
        })
    }
    
    pub fn update(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){

            //if current buffer ended playing, load the next chunk in that one and swap buffers
            if channel.audio_reader.is_some() && channel.get_current_wave().status() == Status::Done{
                if let Some(ref mut reader) = channel.audio_reader{
                    
                    *channel.get_current_wave().get_raw_buffer_mut().unwrap() = reader.get_next_block();

                    channel.swap_wave_buffer();

                }
            }
        }
    }
    
}