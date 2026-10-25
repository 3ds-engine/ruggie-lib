use ctru::services::ndsp::{Ndsp, OutputMode};

use crate::audio::{block_provider::BlockProvider, channel::Channel, errors::AudioError};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler{
    ndsp: Ndsp,
    ///Available channels in runtime.
    ///Must always be in the same spot of memory because of Ndsp's Wave reading
    audio_channels: Box<[Option<Channel>; NUMBER_OF_CHANNELS]>,
}

impl AudioHandler{
    pub fn new() -> Result<Self, AudioError>{

        let mut ndsp = Ndsp::new().map_err(|_|AudioError::NdspInitialization)?;
        ndsp.set_output_mode(OutputMode::Stereo);

        //No channels being used
        let audio_channels: Box<[Option<Channel>; NUMBER_OF_CHANNELS]> = Box::new(std::array::from_fn(|_|{
            None
        }));

        let mut audio_handler = AudioHandler {
            ndsp,
            audio_channels
        };

        //This is for testing
        audio_handler.init_channel(0, BlockProvider::Test)?;

        Ok(audio_handler)
    }

    ///Init the channel number __channel_index__ with the block provider
    pub fn init_channel(&mut self, channel_index: u8, block_provider: BlockProvider) -> Result<(), AudioError>{
        if self.audio_channels[channel_index as usize].is_some(){
            return Err(AudioError::WrongChannelIndex)
        }
        self.audio_channels[channel_index as usize] = Some(Channel::new(channel_index, block_provider)?);

        //To enqueue the buffers, they need to be in their final place of the memory. No moving is done like this. 
        if let Some(ref mut channel) = self.audio_channels[channel_index as usize]{
            channel.enqueue_buffers(&self.ndsp)?;
        }

        Ok(())
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