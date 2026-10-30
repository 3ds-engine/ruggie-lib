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
    pub fn new(channel_index: u8, mut block_provider: BlockProvider) -> Result<Self, AudioError>{

        //Create buffers and fill them
        let mut buffer_0: Vec<u8, LinearAllocator> = Vec::with_capacity_in(44100, LinearAllocator);
        let mut buffer_1: Vec<u8, LinearAllocator> = Vec::with_capacity_in(44100, LinearAllocator);
        block_provider.fill_with_next_block(&mut buffer_0);
        block_provider.fill_with_next_block(&mut buffer_1);

        let format = match block_provider.get_channel_count(){
            1 =>{
                AudioFormat::PCM8Mono
            },
            2 =>{
                AudioFormat::PCM8Stereo
            },
            _ =>{
                return Err(AudioError::BadFormat)
            }
        };

        //Store them in waves
        let audio_buffers = [Wave::new(buffer_0, format, false),
                            Wave::new(buffer_1, format, false)];

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

        let sample_rate = self.block_provider.get_sample_rate().map_err(|_| AudioError::BadFormat)?;
        channel.set_sample_rate(sample_rate as f32);

        match self.block_provider.get_channel_count(){
            1 =>{
                channel.set_format(AudioFormat::PCM8Mono);
            },
            2 =>{
                channel.set_format(AudioFormat::PCM8Stereo);
            },
            _ =>{
                return Err(AudioError::BadFormat)
            }
        }
        channel.set_interpolation(InterpolationType::None);
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