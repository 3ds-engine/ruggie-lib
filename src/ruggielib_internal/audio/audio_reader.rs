use ctru::{linear::LinearAllocator, services::ndsp::{AudioFormat, wave::Wave}};


///Manages loading, unloading and data from audio.
pub enum AudioReader{

}

impl AudioReader{
    pub fn get_next_block(&mut self) -> Wave<Box<[u8], LinearAllocator>>{
        // Zeroed box allocated in the LINEAR memory.
        let audio_data: Box<[_], _> = Box::new_in([0u8; 96], LinearAllocator);

        Wave::new(audio_data, AudioFormat::PCM16Stereo, false)
    }
}