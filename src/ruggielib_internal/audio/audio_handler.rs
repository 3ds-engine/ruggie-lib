use std::{array, collections::HashMap, hash::Hash};

use ctru::services::ndsp::Ndsp;

use crate::audio::{audio_reader::AudioReader, channel::Channel, errors::AudioError, sound::Sound, streamed_sound::StreamedSound};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;
enum LoadedSound{
    Ready(AudioReader),
    Playing(usize)
}

pub struct AudioHandler<'ndsp>{
    //Available channels in runtime
    audio_channels: [Channel<'ndsp>; NUMBER_OF_CHANNELS],

    //Loaded sounds in memory. If the audio is playing, you get an index of the channel where it is being played
    loaded_sounds: HashMap<Sound, LoadedSound>,
    loaded_streamed_sounds: HashMap<StreamedSound, LoadedSound>,
}

impl<'ndsp> AudioHandler<'ndsp>{
    pub fn new(ndsp: &'ndsp Ndsp) -> Result<Self, AudioError>{
        let audio_channels: [Channel<'ndsp>; NUMBER_OF_CHANNELS] = array::from_fn(|i|{
            Channel{
                channel :ndsp.channel(i as u8).unwrap(),
                audio_reader: None
            }
        });

        Ok(AudioHandler { 
            audio_channels, 
            loaded_sounds: HashMap::new(), 
            loaded_streamed_sounds: HashMap::new() 
        })
    }

    /* 
    pub fn update(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){
            if let Some(reader) = &mut channel.audio_reader{
                if wave.finished{
                    let next_block = reader.get_next_block();
                    channel.channel.queue_wave(&mut next_block);
                    
                    store_wave(next_block);
                }

                
            }
        }
    }

    */
}