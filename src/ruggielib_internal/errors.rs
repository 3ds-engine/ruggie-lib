
use thiserror::Error;

use crate::errors::FeatureEnableError;

/// Represents the ways that RuggieLib creation and initialization might fail
#[derive(Error, Debug)]
pub enum RuggieLibCreationError {
    #[error("Could not create ruggielib: Failed to create Apt")]
    FailedApt,

    #[error("Could not create ruggielib: Failed to create Hid")]
    FailedHid,

    #[error("Could not create ruggielib: Failed to create Gfx")]
    FailedGfx,
}

/// Represents the two ways RuggieLib initialization can fail inline
#[derive(Error, Debug)]
pub enum RuggieInitializationError {
    #[error("Could not initialize ruggielib: {0}")]
    Creation(RuggieLibCreationError),

    #[error("Could not initialize ruggielib: {0}")]
    FeatureEnabling(FeatureEnableError),
}
