use crate::audio::channel::{loaded_channel::LoadedChannel, streaming_channel::StreamingChannel};

pub mod streaming_channel;
pub mod loaded_channel;
pub enum Channel{
    StreamingChannel(StreamingChannel),
    LoadedChannel(LoadedChannel)
}