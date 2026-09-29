use std::error::Error;
use ctru::{prelude::*, services::romfs::RomFS};


pub struct RuggieLib{
    apt: Apt,
    hid: Hid,
    gfx: Gfx,
    romfs: Option<RomFS>
}

impl RuggieLib{
    pub fn new() -> Result<Self, Box<dyn Error>>{
        let apt = Apt::new()?;
        let hid = Hid::new()?;
        let gfx = Gfx::new()?;

        Ok(RuggieLib{
            apt,
            hid,
            gfx,
            romfs: None
        }
        )
    }
}