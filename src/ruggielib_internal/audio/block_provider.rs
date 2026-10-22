use core::f32;

use ctru::linear::LinearAllocator;

const SAMPLE_SIZE: usize = 2200;

const A_NOTE_HZ : usize = 440; //440HZ

///Manages loading, unloading and data from audio.
pub enum BlockProvider{
    Test
}

impl BlockProvider{

    ///Returns a buffer of bytes. These bytes will be decoded as the audio format of the Channel in PCM
    pub fn get_next_block(&mut self) -> Vec<u8, LinearAllocator>{
        // TEST!!!

        //STEP 1: Get a block sample (Generated)

        //Add to the accumulative value each step for sin value. In radians
        let step_add = (A_NOTE_HZ as f32 / SAMPLE_SIZE as f32) * f32::consts::PI;
        
        let mut sin_vec = Vec::new_in(LinearAllocator);
        let mut step_acc = 0.0;

        //Calculate sin function based on the step_add
        sin_vec.resize_with(SAMPLE_SIZE, ||{
            let res = step_acc;
            step_acc += step_add;
            if step_acc >= f32::consts::PI * 2.0{
                step_acc -= f32::consts::PI * 2.0;
            }

            res.sin()
        });
        
        //STEP 2: Convert the float block sample into a byte buffer

        let bytes: Vec<u8> = sin_vec
        .iter()
        .flat_map(|&s| {
            let s = s.clamp(-1.0, 1.0);
            let pcm = (s * 32767.0) as i16;
            pcm.to_le_bytes()
        })
        .collect();

        let mut linear_buffer: Vec<u8, LinearAllocator> = Vec::with_capacity_in(bytes.len(), LinearAllocator);

        //Copies vector in normal memory to vector in linear memory. Should be done once in linear memory
        linear_buffer.extend_from_slice(&bytes);

        linear_buffer
    }
}