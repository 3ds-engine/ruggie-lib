use ctru::{linear::LinearAllocator, services::ndsp::{AudioFormat, wave::Wave}};


///Manages loading, unloading and data from audio.
pub enum AudioReader{

}

impl AudioReader{

    //this is a placeholder
    pub fn get_next_block(&mut self) -> Vec<u8, LinearAllocator>{
        // Zeroed vector allocated in the LINEAR memory.
        
        let mut vec = Vec::new_in(LinearAllocator);

        vec.resize_with(96, || 0);

        vec
    }
}