use ctru::{linear::LinearAllocator, services::ndsp::{AudioFormat, AudioMix, InterpolationType, Ndsp, wave::{Status, Wave}}};

use crate::audio::{block_provider::BlockProvider, errors::AudioError};

/// Struct that represents a channel of the NDSP processor of the 3DS. It holds an option to an audio_reader. 
/// 
/// In the update method, the channel requests blocks from the audio_reader.
pub struct Channel{
    ///Channel associated to the NDSP processor
    pub channel_index: u8,

    ///Block provider which will be providing the audio blocks to the Channel's current buffer
    pub block_provider: BlockProvider, 
    
    ///Audio buffers being used by the channel. Whenever one channel is playing, the block provider must fill the other
    audio_buffers: [Wave<Vec<u8, LinearAllocator>>; 2],
    
    current_buffer: usize
}

impl Channel{
    pub fn new(channel_index: u8, block_provider: BlockProvider) -> Result<Self, AudioError>{
        let audio_buffers = [Wave::new(Vec::new_in(LinearAllocator), AudioFormat::PCM16Mono, false),
                            Wave::new(Vec::new_in(LinearAllocator), AudioFormat::PCM16Mono, false)];

        Ok(Channel{
                //Get a reference to the NDSP channel. If it fails returns which channel failed
                channel_index,
                //Not audio reader selected
                block_provider,
                //Double wave buffer initialization
                audio_buffers,

                current_buffer: 0
            }
        )
    }

    pub fn enqueue_buffers(&mut self,ndsp: &Ndsp) -> Result<(), AudioError>{
        let mut channel = ndsp.channel(self.channel_index).map_err(|_| AudioError::AudioChannelInitialization(self.channel_index))?;

        channel.set_sample_rate(44100.);
        channel.set_format(AudioFormat::PCM16Mono);
        channel.set_interpolation(InterpolationType::Linear);
        channel.set_mix(&AudioMix::default());

        channel.queue_wave(&mut self.audio_buffers[0]).map_err(|_| AudioError::WaveEnqueue)?;
        channel.queue_wave(&mut self.audio_buffers[1]).map_err(|_| AudioError::WaveEnqueue)?;

        Ok(())
    }

    //Fills back buffer with new blocks by the block provider
    pub fn check_for_update(&mut self, ndsp: &Ndsp) -> Result<(), AudioError>{

        //If front buffer ended, switch it for the back-buffer
        if self.audio_buffers[self.current_buffer].status() == Status::Done {

            //Swaps current buffer
            self.current_buffer = 1 - self.current_buffer;
            
            let back_buffer_index = 1- self.current_buffer;

            //Fill the now back-buffer with the provider's next block
            let raw_buffer = self.audio_buffers[back_buffer_index].get_raw_buffer_mut().map_err(|_| AudioError::WaveBufferRef)?;
            self.block_provider.fill_with_next_block(raw_buffer);

            //Append the back-buffer to the channel
            let mut channel = ndsp.channel(self.channel_index).map_err(|_| AudioError::NdspChannelReference(self.channel_index))?;
            channel.queue_wave(&mut self.audio_buffers[back_buffer_index]).map_err(|_| AudioError::WaveEnqueue)?;
        }

        Ok(())
    }
}