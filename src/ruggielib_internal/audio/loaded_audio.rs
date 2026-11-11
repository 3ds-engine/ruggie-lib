use std::rc::{Rc, Weak};

use ctru::linear::LinearAllocator;

use crate::audio::block_provider::BlockProvider;

pub struct LoadedAudio{
    ///Shared pointer to audio buffer in linear mem
    pub buffer: Rc<[u8], LinearAllocator>,
    ///Handle counter. If in update the weak_ptr is not valid, it calls for channel cleanup and deletion of buffer audio
    pub handle_counter: Weak<()>,
    ///Block provider for audio specs
    pub block_provider: BlockProvider
}