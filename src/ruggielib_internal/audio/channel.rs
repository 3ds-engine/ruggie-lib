use ctru::services::ndsp::Channel as NdspChannel;

use crate::audio::audio_reader::AudioReader;

/// Struct that represents a channel of the NDSP processor of the 3DS. It holds an option to an audio_reader. 
/// 
/// In the update method, the channel requests blocks from the audio_reader.
pub struct Channel<'ndsp>{
    channel:  NdspChannel<'ndsp>,
    audio_reader: Option<AudioReader>
}