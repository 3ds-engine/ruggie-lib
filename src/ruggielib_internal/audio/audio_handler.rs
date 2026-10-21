use ctru::services::ndsp::{Ndsp, wave::Status};

use crate::audio::{channel::Channel, errors::AudioError};

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
    
    ///Check if any audio channels need another block
    pub fn update_channels(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){
            //Check if the buffers need swap and fill
            channel.check_for_update();
        }
    }
    
}