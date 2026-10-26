use core::f32;

use ctru::linear::LinearAllocator;

use crate::audio::constants::*;


const A_NOTE_HZ : usize = 440; //440HZ

///Manages loading, unloading and data from audio.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockProvider{
    Test
}

impl BlockProvider{

    ///Returns a buffer of bytes. These bytes will be decoded as the audio format of the Channel in PCM
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>) {
        // TEST!!!

        //STEP 1: Get a block sample (Generated)

        //Add to the accumulative value each step for sin value. In radians
        let step_add = (A_NOTE_HZ as f32 / SAMPLE_SIZE as f32) * f32::consts::PI;
        
        let mut sin_vec = Vec::new();
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

        //If the buffer cant hold the data, resize it
        if buffer.len() != SAMPLE_SIZE * BYTES_PER_F32  {
            buffer.resize(SAMPLE_SIZE * BYTES_PER_F32, 0);
        }

        let mut buffer_index =0;

        for value in sin_vec{
            let value = value.clamp(-1.0, 1.0);
            let pcm = (value * 32767.0) as i16;

            for byte in pcm.to_le_bytes(){
                buffer[buffer_index] = byte;

                buffer_index+=1;
            }
        }

        //If there was not enough info to fill the entire buffer, fill it with 0
        for i in buffer_index..buffer.len(){
            buffer[i] = 0;
        }
    }
}