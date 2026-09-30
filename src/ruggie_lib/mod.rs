use citro2d_sys::*;
use citro3d_sys::*;
use ctru::{prelude::*, services::romfs::RomFS};
pub mod ruggie_draw_handle;
pub mod ruggie_screen_target;
use ruggie_screen_target::RuggieScreenTarget;

use crate::ruggie_lib::ruggie_draw_handle::RuggieDrawHandle;

#[allow(dead_code)]
pub struct RuggieLib {
    apt: Apt,
    hid: Hid,
    gfx: Gfx,
    top_screen: RuggieScreenTarget,
    bottom_screen: RuggieScreenTarget,
    romfs: Option<RomFS>,
}

impl RuggieLib {
    pub fn new() -> Result<Self, RuggieLibCreationError> {
        let apt = Apt::new().map_err(|_| RuggieLibCreationError::FailedApt)?;
        let hid = Hid::new().map_err(|_| RuggieLibCreationError::FailedHid)?;
        let gfx = Gfx::new().map_err(|_| RuggieLibCreationError::FailedGfx)?;

        let top_screen = RuggieScreenTarget::new_top_screen();
        let bottom_screen = RuggieScreenTarget::new_bottom_screen();

        unsafe{
            C3D_Init(C3D_DEFAULT_CMDBUF_SIZE as usize);
            C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize);
            C2D_Prepare();
        }

        

        Ok(RuggieLib {
            apt,
            hid,
            gfx,
            top_screen,
            bottom_screen,
            romfs: None,
        })
    }

    pub fn start_drawing_top(&self) -> RuggieDrawHandle{
        RuggieDrawHandle::new(&self.top_screen)
    }

    pub fn start_drawing_bottom(&self) -> RuggieDrawHandle{
        RuggieDrawHandle::new(&self.bottom_screen)
    }

    pub fn with_romfs(mut self) -> ctru::Result<Self> {
        let romfs = RomFS::new()?;
        self.romfs = Some(romfs);

        Ok(self)
    }

    pub fn is_running(&self) -> bool{
        self.apt.main_loop()
    }

    pub fn wait_for_vblank(&self){
        self.gfx.wait_for_vblank();
    }

}

impl Drop for RuggieLib{
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
    }
}

pub enum RuggieLibCreationError {
    FailedApt,
    FailedHid,
    FailedGfx,
}
