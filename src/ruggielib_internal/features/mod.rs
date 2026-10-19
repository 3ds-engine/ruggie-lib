/// The console feature
///
/// Creates a console output in one of the screens, where stdout will be redirected
pub mod console;

/// Feature errors
///
/// This contains all errors that `RuggieLib`'s features can return in Results
pub mod errors;
use errors::FeatureEnableError;

use crate::{features::console::RuggieConsole, ruggielib_internal::RuggieLib};
use ctru::services::romfs::RomFS;

/// The representation of a feature that can optionally be enabled to add functionality to `RuggieLib`
pub enum Feature {
    RomFS,          // Enables reading files from the application
    Stereoscopic3D, // Enables support for overlap 3D rendering using the 3D slider
    Console(u8),    // Creates a console output in a target screen to redirect stdout
}

/// Main feature implementation
impl RuggieLib {
    /// Enables features for `RuggieLib`. Intended to be used during construction of the library
    /// handle, but features can be enabled as needed using this function
    ///
    /// # Example
    /// ```
    /// let rl = RuggieLib::new().unwrap().with([
    ///     Feature::RomFS,
    ///     Feature::Stereoscopic3D,
    ///     Feature::Console(TOP_SCREEN_INDEX)
    /// ]);
    ///
    /// ```
    ///
    /// # Errors
    /// This function can fail when trying to add a feature that requires a hardware/firmware
    /// feature to be present and it can't find/load it
    pub fn with(
        mut self,
        features: impl IntoIterator<Item = Feature>,
    ) -> Result<Self, FeatureEnableError> {
        for feature in features {
            match feature {
                Feature::RomFS => {
                    let romfs = RomFS::new().map_err(|_| FeatureEnableError::FailedRomFS)?;
                    self.romfs = Some(romfs);
                }
                Feature::Stereoscopic3D => unsafe { ctru_sys::gfxSet3D(true) },
                Feature::Console(screen_index) => {
                    self.console = Some(RuggieConsole::new(screen_index));
                }
            }
        }

        Ok(self)
    }
}
