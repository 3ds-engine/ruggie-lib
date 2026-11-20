use thiserror::Error;

/// Represents the ways that `RuggieLib` creation and initialization might fail
#[derive(Error, Debug)]
pub enum RuggieAudioHandleError {
    #[error("Could not initialize OpenDSP")]
    Initialization,
}
