
use thiserror::Error;

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
