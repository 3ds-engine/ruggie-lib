use thiserror::Error;

/// Feature enabling error
///
/// This error might be returned by the RuggieLib::with() function, when enabling a new feature
#[derive(Error, Debug)]
pub enum FeatureEnableError {
    #[error("Could not enable ruggielib features: Failed to create RomFS")]
    FailedRomFS,
}
