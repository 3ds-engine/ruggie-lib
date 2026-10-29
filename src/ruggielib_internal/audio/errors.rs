use thiserror::Error;

///Errors that may be retrieved when reading input.
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Could not initialize NDSP")]
    NdspInitialization,

    #[error("Could not initialize Audio Channel")]
    AudioChannelInitialization(u8),

    #[error("Could not reference NDSP Channel")]
    NdspChannelReference(u8),

    #[error("Error when queueing a Wave into a NDSP channel")]
    WaveEnqueue,

    #[error("Used wrong channel index")]
    WrongChannelIndex,

    #[error("Could not obtain raw_buf_mut of wave")]
    WaveBufferRef,

    #[error("Audio reader has a bad format")]
    BadFormat
}

#[derive(Error, Debug)]
pub enum SoundInitializationError{
    #[error("Wrong specified path")]
    WrongPath,

    #[error("Could not stream audio in channel")]
    ChannelInitialization,

    #[error("Could not initialize block provider")]
    BlockProviderInitialization,
}

#[derive(Error, Debug)]
pub enum AudioStreamReadError{
    #[error("Unrecoverable")]
    UnrecoverableSymphoniaError,
    #[error("Packet skipped")]
    PacketSkip,
}

#[derive(Error, Debug)]
pub enum SampleRateError{
    #[error("Could not retrieve sample rate")]
    RetrievalError,
}

#[derive(Error, Debug)]
pub enum BlockProviderError{
    #[error("Failed to initialize BlockProvider")]
    Initialization,
}