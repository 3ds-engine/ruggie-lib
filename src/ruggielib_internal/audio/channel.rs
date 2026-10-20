use ctru::{linear::LinearAllocator, services::ndsp::{AudioFormat, Channel as NdspChannel, Ndsp, wave::Wave}};

use crate::audio::{audio_reader::AudioReader, errors::AudioError};

/// Struct that represents a channel of the NDSP processor of the 3DS. It holds an option to an audio_reader. 
/// 
/// In the update method, the channel requests blocks from the audio_reader.
pub struct Channel<'ndsp>{
    ///Channel associated to the NDSP processor
    pub channel:  NdspChannel<'ndsp>,

    ///Audio reader which will be providing the audio blocks to the Channel's NDSP Channel
    pub audio_reader: Option<AudioReader>, 

    ///Audio buffers being used by the channel. Whenever one channel is playing, the audio reader must fill the other
    double_wave_buffer: [Wave<Vec<u8, LinearAllocator>>; 2],

    current_wave: usize
}

impl<'ndsp> Channel<'ndsp>{
    pub fn new(channel_index: u8, ndsp: &'ndsp Ndsp) -> Result<Self, AudioError>{
        Ok(Channel{
                //Get a reference to the NDSP channel. If it fails returns which channel failed
                channel :ndsp.channel(channel_index).map_err(|_| AudioError::AudioChannelInitialization(channel_index))?,
                //Not audio reader selected
                audio_reader: None,
                //Double wave buffer initialization
                double_wave_buffer: [Wave::new(Vec::new_in(LinearAllocator), AudioFormat::PCM16Stereo, false),
                                    Wave::new(Vec::new_in(LinearAllocator), AudioFormat::PCM16Stereo, false)],
                current_wave: 0
            }
        )
    }

    pub fn swap_wave_buffer(&mut self){
        //Swaps 0 to 1, or 1 to 0
        self.current_wave = 1 - self.current_wave;
    }

    pub fn get_current_wave(&mut self) -> &mut Wave<Vec<u8, LinearAllocator>>{
        &mut self.double_wave_buffer[self.current_wave]
    }
}