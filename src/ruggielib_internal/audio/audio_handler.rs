use std::collections::HashMap;

use crate::audio::{audio_reader::AudioReader, channel::Channel, sound::Sound, streamed_sound::StreamedSound};

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
