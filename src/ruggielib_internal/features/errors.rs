use thiserror::Error;


#[derive(Error, Debug)]
pub enum FeatureEnableError {
    #[error("Could not enable ruggielib features: Failed to create RomFS")]
    FailedRomFS,
}
