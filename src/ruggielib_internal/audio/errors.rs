use thiserror::Error;

///Errors that may be retrieved when reading input.
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Could not initialize NDSP")]
    NdspInitialization,

    #[error("Could not initialize Audio Channel")]
    AudioChannelInitialization(u8),

    #[error("Error getting reference of NDSP's Wave buffer")]
    WaveBufferReference,

    #[error("Error when queueing a Wave into a NDSP channel")]
    WaveEnqueue,
}
