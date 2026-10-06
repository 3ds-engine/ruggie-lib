use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuggieLibCreationError {
    #[error("Could not create ruggielib: Failed to create Apt")]
    FailedApt,

    #[error("Could not create ruggielib: Failed to create Hid")]
    FailedHid,

    #[error("Could not create ruggielib: Failed to create Gfx")]
    FailedGfx,
}

#[derive(Error, Debug)]
pub enum FeatureEnableError {
    #[error("Could not enable ruggielib features: Failed to create RomFS")]
    FailedRomFS,
}
