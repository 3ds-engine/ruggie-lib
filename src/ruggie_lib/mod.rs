use ctru::{prelude::*, services::romfs::RomFS};

#[allow(dead_code)]
pub struct RuggieLib {
    apt: Apt,
    hid: Hid,
    gfx: Gfx,
    romfs: Option<RomFS>,
}

impl RuggieLib {
    pub fn new() -> Result<Self, RuggieLibCreationError> {
        let apt = Apt::new().map_err(|_| RuggieLibCreationError::FailedApt)?;
        let hid = Hid::new().map_err(|_| RuggieLibCreationError::FailedHid)?;
        let gfx = Gfx::new().map_err(|_| RuggieLibCreationError::FailedGfx)?;

        Ok(RuggieLib {
            apt,
            hid,
            gfx,
            romfs: None,
        })
    }
}


pub enum RuggieLibCreationError {
    FailedApt,
    FailedHid,
    FailedGfx,
}
