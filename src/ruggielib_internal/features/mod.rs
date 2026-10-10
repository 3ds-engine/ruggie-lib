pub mod console;

use crate::{features::console::RuggieConsole, ruggielib_internal::{RuggieLib, errors::FeatureEnableError}};
use ctru::services::romfs::RomFS;

pub enum Feature {
    RomFS,
    Stereoscopic3D,
    Console(u8),
}

impl RuggieLib {
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
