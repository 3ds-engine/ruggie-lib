use ctru::services::ndsp::{Ndsp, OutputMode};

use crate::audio::{channel::Channel, errors::AudioError};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler{
    ndsp: Ndsp,
    //Available channels in runtime
    audio_channels: [Channel; NUMBER_OF_CHANNELS],
}

impl AudioHandler{
    pub fn new() -> Result<Self, AudioError>{

        let mut ndsp = Ndsp::new().map_err(|_|AudioError::NdspInitialization)?;
        ndsp.set_output_mode(OutputMode::Stereo);

        let mut audio_channels: [Channel; NUMBER_OF_CHANNELS] = std::array::try_from_fn(|i|{
            Channel::new(i as u8)
        })?;

        //Init to hear if it sounds
        audio_channels[0].test_init(&mut ndsp);

        Ok(AudioHandler { 
            ndsp,
            audio_channels
        })
    }
    
    ///Check if any audio channels need another block
    pub fn update_channels(&mut self){
        println!("[RuggieLib::AudioHandler] Checking sound channels");
        for channel in &mut self.audio_channels.iter_mut(){
            //Check if the buffers need swap and fill
            channel.check_for_update(&self.ndsp);
        }
    }
    
}