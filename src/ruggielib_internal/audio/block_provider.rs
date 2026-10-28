use std::fs::File;

use ctru::linear::LinearAllocator;
use symphonia::core::{audio::GenericAudioBufferRef, codecs::audio::{AudioDecoder, AudioDecoderOptions}, formats::{FormatOptions, FormatReader, TrackType, probe::Hint}, io::MediaSourceStream, meta::MetadataOptions};

use crate::audio::errors::AudioStreamReadError;
use symphonia::core::errors::Error;

///Manages loading, unloading and data from audio.
pub struct BlockProvider{
    format: Box<dyn FormatReader>,
    decoder: Box<dyn AudioDecoder>,
    track_id: u32
}

impl BlockProvider{

    //Picked from symphonia's basic example
    pub fn new(file_handle: Box<File>) -> Self /*Result<Self, >*/{
        // Create the media source stream.
        let mss = MediaSourceStream::new(file_handle, Default::default());

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source.
        let format = symphonia::default::get_probe()
            .probe(&Hint::new(), mss, fmt_opts, meta_opts)
            .expect("unsupported format");

        // Find the first audio track with a known (decodeable) codec.
        let track = format.default_track(TrackType::Audio).expect("no audio track");

        // Use the default options for the decoder.
        let dec_opts: AudioDecoderOptions = Default::default();

        // Create a decoder for the track.
        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(
                track.codec_params.as_ref().expect("codec parameters missing").audio().unwrap(),
                &dec_opts,
            )
            .expect("unsupported codec");

        // Store the track identifier, it will be used to filter packets.
        let track_id = track.id;

        BlockProvider {
            format,
            decoder,
            track_id
        }
    }

    ///Returns a buffer of bytes. These bytes will be decoded as the audio format of the Channel in PCM
    pub fn fill_with_next_block(&mut self, buffer: &mut Vec<u8, LinearAllocator>) {
        
        let mut audio_buffer = self.get_next_block();

        //Get blocks until no need to skip
        while let Err(AudioStreamReadError::PacketSkip) = audio_buffer{
            audio_buffer = self.get_next_block();
        }

        match audio_buffer{
            Ok(Some(buf)) =>{
                //Buffer with info
            },
            Ok(None) =>{
                //Buffer ended streaming
            },
            Err(_)=>{
                //If there is an error, return unrecoverable err
            }
        }
    }

    //Gets the next buffer from the decoder
    fn get_next_block(&mut self) -> Result<Option<GenericAudioBufferRef<'_>>, AudioStreamReadError>{
        // Get the next packet from the media format.
        let packet = match self.format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => {
                // Reached the end of the stream.
                return Ok(None)
            }
            Err(_) => {
                // A unrecoverable error occurred, halt decoding.
                return Err(AudioStreamReadError::UnrecoverableSymphoniaError)
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !self.format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            self.format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id != self.track_id {
            return Err(AudioStreamReadError::PacketSkip)
        }

        // Decode the packet into audio samples.
        match self.decoder.decode(&packet) {
            Ok(_decoded) => {
                return Ok(Some(_decoded))
                // Consume the decoded audio samples (see below).
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                return Err(AudioStreamReadError::PacketSkip)
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                return Err(AudioStreamReadError::PacketSkip)
            }
            Err(_) => {
                // An unrecoverable error occurred, halt decoding.
                return Err(AudioStreamReadError::UnrecoverableSymphoniaError)
            }
        }
    }
}