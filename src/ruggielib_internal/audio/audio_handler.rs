use std::{collections::HashMap, path::Path, rc::{Rc, Weak}};

use ctru::{linear::LinearAllocator, services::ndsp::{Ndsp, OutputMode}};

use crate::audio::{block_provider::BlockProvider, channel::streaming_channel::StreamingChannel, errors::{AudioError, SoundInitializationError}, loaded_audio::{self, LoadedAudio}, sound::Sound};

///Number of channels available of NDSP
const NUMBER_OF_CHANNELS: usize = 24;

pub struct AudioHandler{
    ndsp: Ndsp,
    //Loaded audio buffers in linear memory
    loaded_sounds: HashMap<String, LoadedAudio>,
    ///Available channels in runtime.
    ///Must always be in the same spot of memory because of Ndsp's Wave reading
    audio_channels: Vec<Option<StreamingChannel>>,
}

impl AudioHandler{
    pub fn new() -> Result<Self, AudioError>{

        let mut ndsp = Ndsp::new().map_err(|_|AudioError::NdspInitialization)?;
        ndsp.set_output_mode(OutputMode::Stereo);

        //No channels being used
        let mut audio_channels: Vec<Option<StreamingChannel>> = Vec::with_capacity(NUMBER_OF_CHANNELS);
        audio_channels.resize_with(NUMBER_OF_CHANNELS, || None);

        Ok(AudioHandler {
            ndsp,
            loaded_sounds: HashMap::new(),
            audio_channels,
        })
    }

    ///Init the channel number __channel_index__ with the block provider
    pub fn init_channel(&mut self, channel_index: u8, block_provider: BlockProvider) -> Result<(), AudioError>{
        if self.audio_channels[channel_index as usize].is_some(){
            return Err(AudioError::WrongChannelIndex)
        }
        self.audio_channels[channel_index as usize] = Some(StreamingChannel::new(channel_index, block_provider, &mut self.ndsp)?);

        //To enqueue the buffers, they need to be in their final place of the memory. No moving is done this way
        if let Some(ref mut channel) = self.audio_channels[channel_index as usize]{
            channel.enqueue_buffers(&self.ndsp)?;
        }

        Ok(())
    }
    
    ///Check if any audio channels need another block
    pub fn update_channels(&mut self){
        for channel in &mut self.audio_channels.iter_mut(){
            //Checks if channel is being used
            if let Some(channel) = channel{
                //Check if the buffers need swap and fill
                channel.check_for_update(&self.ndsp);
            }
        }
    }

    pub fn play_streamed_sound(&mut self, path: &str, channel_index: u8) -> Result<(), SoundInitializationError> {
        //Initialize the audio reader for the file
        let block_provider = BlockProvider::new(Path::new(path))?;
        //Append that reader to the specified channel
        self.init_channel(channel_index, block_provider).map_err(|_| SoundInitializationError::ChannelInitialization)?;

        Ok(())
    }

    pub fn play_sound(&mut self, sound: &Sound){

    }

    pub fn load_sound(&mut self, path: &str) -> Result<Sound, SoundInitializationError>{
        //A loaded audio exists
        if let Some(loaded_audio) = self.loaded_sounds.get_mut(path){
            match loaded_audio.handle_counter.upgrade(){
                //If there are another handles existing, return a copy
                Some(rc_handle) =>{
                    return Ok(Sound(rc_handle.clone()))
                },
                //If there are no more handles, user requested a sound that was about to be deleted, so we create a new handle and pass it to prevent deletion
                None =>{
                    let rc_handle = Rc::new(());
                    loaded_audio.handle_counter = Rc::downgrade(&rc_handle);
                    return Ok(Sound(rc_handle))
                }
            }
        }
        //If the audio is not loaded, create it and pass the handle

        let mut block_provider = BlockProvider::new(Path::new(path))?;
        let buffer_rc = Self::load_buffered_sound(&mut block_provider)?;
        let handle_rc = Rc::new(());
        let loaded_audio = LoadedAudio{
            buffer: buffer_rc,
            block_provider,
            handle_counter: Rc::downgrade(&handle_rc)
        };
        
        self.loaded_sounds.insert(path.to_string(), loaded_audio);

        Ok(Sound(handle_rc))
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