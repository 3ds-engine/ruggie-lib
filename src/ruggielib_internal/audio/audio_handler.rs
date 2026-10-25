use ctru::services::ndsp::{Ndsp, OutputMode};

use crate::audio::{channel::Channel, errors::AudioError};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler{
    ndsp: Ndsp,
    //Available channels in runtime.
    //Must always be in the same spot of memory because of Ndsp's Wave reading
    audio_channels: Box<[Option<Channel>; NUMBER_OF_CHANNELS]>,
}

impl AudioHandler{
    pub fn new() -> Result<Self, AudioError>{

        let mut ndsp = Ndsp::new().map_err(|_|AudioError::NdspInitialization)?;
        ndsp.set_output_mode(OutputMode::Stereo);

        //No channels being used
        let mut audio_channels: Box<[Option<Channel>; NUMBER_OF_CHANNELS]> = Box::new(std::array::from_fn(|_|{
            None
        }));

        //Use channel 0 for testing
        audio_channels[0] = Some(Channel::new(0, &ndsp)?);

        Ok(AudioHandler { 
            ndsp,
            audio_channels
        })
    }
    
    ///Check if any audio channels need another block
    pub fn update_channels(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){
            //Checks if channel is being used
            if let Some(channel) = channel{
                //Check if the buffers need swap and fill
                channel.check_for_update(&self.ndsp);
            }
        }
    }
    
}