use thiserror::Error;

///Errors that may be retrieved when reading input.
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Could not initialize NDSP")]
    NdspInitialization,

    #[error("Could not reference NDSP Channel")]
    NdspChannelReference(u8),

    #[error("Error when queueing a Wave into a NDSP channel")]
    WaveEnqueue,

    #[error("Used wrong channel index")]
    WrongChannelIndex,

    #[error("Could not obtain raw_buf_mut of wave")]
    WaveBufferRef,

    #[error("Error when updating channel")]
    ChannelUpdate,
}

#[derive(Error, Debug)]
pub enum SoundInitializationError{
    #[error("Wrong specified path")]
    WrongPath,

    #[error("Could not stream audio in channel")]
    ChannelInitialization,

    #[error("Could not initialize reader")]
    ReaderInitialization,

    #[error("Extension not provided or not supported")]
    BadExtension,
}

#[derive(Error, Debug)]
pub enum SoundReadingError{
    #[error("Could not obtain next block audio")]
    NextBlock,

    #[error("Could not obtain next block audio")]
    WholeSoundRead
}

