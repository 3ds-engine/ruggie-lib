use std::{collections::HashMap, path::Path, rc::Rc};

use ctru::{linear::LinearAllocator, services::ndsp::{Ndsp, OutputMode}};

use crate::audio::{block_provider::BlockProvider, channel::{Channel::{self, Loaded, Streamed}, loaded_channel::LoadedChannel, streaming_channel::StreamingChannel}, errors::{AudioError, SoundInitializationError}, loaded_audio::{self, LoadedAudio}, sound::Sound};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler{
    ndsp: Ndsp,
    //Loaded audio buffers in linear memory
    loaded_sounds: HashMap<String, LoadedAudio>,
    ///Available channels in runtime.
    ///Must always be in the same spot of memory because of Ndsp's Wave reading
    audio_channels: Vec<Option<Channel>>,
}

impl AudioHandler{
    pub fn new() -> Result<Self, AudioError>{

        let mut ndsp = Ndsp::new().map_err(|_|AudioError::NdspInitialization)?;
        ndsp.set_output_mode(OutputMode::Stereo);

        //No channels being used
        let mut audio_channels: Vec<Option<Channel>> = Vec::with_capacity(NUMBER_OF_CHANNELS);
        audio_channels.resize_with(NUMBER_OF_CHANNELS, || None);

        Ok(AudioHandler {
            ndsp,
            loaded_sounds: HashMap::new(),
            audio_channels,
        })
    }

    ///Check if any audio channels need another block
    pub fn update_channels(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){
            //Checks if channel is being used
            match channel{
                Some(Streamed(streamed_channel)) =>{
                    streamed_channel.check_for_update(&self.ndsp);
                },
                Some(Loaded(loaded_audio)) =>{

                },
                _ =>{}
            }
        }
    }

    pub fn play_streamed_sound(&mut self, path: &str, channel_index: u8) -> Result<(), AudioError> {
        //Initialize the audio reader for the file
        let block_provider = BlockProvider::new(Path::new(path)).map_err(|_| AudioError::ChannelUpdate)?;
        //Append that reader to the specified channel
        if self.audio_channels[channel_index as usize].is_some(){
            return Err(AudioError::WrongChannelIndex)
        }

        let streaming_channel = StreamingChannel::new(channel_index, block_provider, &mut self.ndsp)?;
        self.audio_channels[channel_index as usize] = Some(Channel::Streamed(streaming_channel));

        //To enqueue the buffers, they need to be in their final place of the memory. No moving is done this way
        if let Some(Streamed(ref mut channel)) = self.audio_channels[channel_index as usize]{
            channel.enqueue_buffers(&self.ndsp)?;
        }

        Ok(())
    }

    pub fn play_sound(&mut self, sound: &Sound, channel_index: u8) -> Result<(), AudioError>{
        if let Some(loaded_sound) = self.loaded_sounds.get(&sound.path){
            
            if self.audio_channels[channel_index as usize].is_some(){
                return Err(AudioError::WrongChannelIndex)
            }
            
            let loaded_channel = LoadedChannel::new(channel_index, loaded_sound, &mut self.ndsp)?;

            self.audio_channels[channel_index as usize] = Some(Loaded(loaded_channel));

            if let Some(Loaded(ref mut loaded_channel)) = self.audio_channels[channel_index as usize]{
                loaded_channel.enqueue_buffer(&self.ndsp);
            }
        }
        Ok(())
    }

    pub fn load_sound(&mut self, path: &str) -> Result<Sound, SoundInitializationError>{
        //A loaded audio exists
        if let Some(loaded_audio) = self.loaded_sounds.get_mut(path){
            match loaded_audio.handle_counter.upgrade(){
                //If there are another handles existing, return a copy
                Some(rc_handle) =>{
                    return Ok(Sound{
                        handle: rc_handle.clone(),
                        path: path.to_string()
                    })
                },
                //If there are no more handles, user requested a sound that was about to be deleted, so we create a new handle and pass it to prevent deletion
                None =>{
                    let rc_handle = Rc::new(());
                    loaded_audio.handle_counter = Rc::downgrade(&rc_handle);
                    return Ok(Sound{
                        handle: rc_handle,
                        path: path.to_string()
                    })
                }
            }
        }

        //If the audio is not loaded, create it and pass the handle
        let mut block_provider = BlockProvider::new(Path::new(path))?;
        let buffer_rc = Self::load_buffered_sound(&mut block_provider)?;
        let rc_handle = Rc::new(());
        let loaded_audio = LoadedAudio{
            buffer: buffer_rc,
            block_provider,
            handle_counter: Rc::downgrade(&rc_handle)
        };
        
        self.loaded_sounds.insert(path.to_string(), loaded_audio);

        Ok(Sound{
            handle: rc_handle,
            path: path.to_string()
        })
    }

    ///Returns an Rc with a buffer filled with the audio information from path
    fn load_buffered_sound(block_provider: &mut BlockProvider) -> Result<Rc<[u8], LinearAllocator>, SoundInitializationError>{
        

        //First, a resizable vector in heap
        let mut buffer: Vec<i16> = Vec::new();
        //Fill the vector with the total data of the audio
        block_provider.fill_with_whole_data(&mut buffer).map_err(|_| SoundInitializationError::ReaderInitialization)?;

        //Now, we cast the data into a buffer of bytes.
        let buffer = unsafe{
            let mut md = std::mem::ManuallyDrop::new(buffer);
    
            let ptr = md.as_mut_ptr() as *mut u8;
            let len = md.len() * std::mem::size_of::<i16>();
            let cap = md.capacity() * std::mem::size_of::<i16>();
    
            Vec::from_raw_parts(ptr, len, cap)
        };

        //Copy the data slice into a linear memory Rc and return it
        let buffer: Rc<[u8], LinearAllocator> = Rc::clone_from_ref_in(buffer.as_slice(), LinearAllocator);
        
        Ok(buffer)
    }
    
}