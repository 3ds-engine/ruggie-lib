
use thiserror::Error;

use crate::errors::FeatureEnableError;

/// Represents the ways that `RuggieLib` creation and initialization might fail
#[derive(Error, Debug)]
pub enum RuggieLibCreationError {
    #[error("Could not create ruggielib: Failed to create Apt")]
    FailedApt,

    #[error("Could not create ruggielib: Failed to create Hid")]
    FailedHid,

    #[error("Could not create ruggielib: Failed to create Gfx")]
    FailedGfx,

    #[error("Could not create ruggielib: Failed to create Ndsp")]
    FailedNdsp,
}

/// Represents the two ways `RuggieLib` initialization can fail inline
#[derive(Error, Debug)]
pub enum RuggieLibInitializationError {
    #[error("Could not initialize ruggielib: {0}")]
    Creation(RuggieLibCreationError),

    #[error("Could not initialize ruggielib: {0}")]
    FeatureEnabling(FeatureEnableError),
}

impl From<RuggieLibCreationError> for RuggieLibInitializationError {
    fn from(value: RuggieLibCreationError) -> Self {
        Self::Creation(value)
    }
}

impl From<FeatureEnableError> for RuggieLibInitializationError {
    fn from(value: FeatureEnableError) -> Self {
        Self::FeatureEnabling(value)
    }
}
