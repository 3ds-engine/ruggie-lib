pub mod errors;
use errors::RuggieLibCreationError;

pub mod draw;
use draw::RuggieDrawHandle;

pub mod screen;
use screen::RuggieScreenTarget;

pub mod features;
use crate::features::console::RuggieConsole;

pub mod input;
pub mod tests;

use citro2d_sys::{C2D_DEFAULT_MAX_OBJECTS, C2D_Fini, C2D_Init, C2D_Prepare};
use citro3d_sys::{C3D_DEFAULT_CMDBUF_SIZE, C3D_Fini, C3D_Init};
use ctru::{prelude::*, services::romfs::RomFS};
use ctru_sys::{GFX_BOTTOM, GFX_TOP};

pub const TOP_SCREEN_INDEX : u8 = GFX_TOP;
pub const BOTTOM_SCREEN_INDEX : u8 = GFX_BOTTOM;

pub struct RuggieLib {
    // Base tools
    apt: Apt,
    hid: Hid,
    gfx: Gfx,

    // Screens
    pub top_left_screen: RuggieScreenTarget,
    pub top_right_screen: RuggieScreenTarget,
    pub bottom_screen: RuggieScreenTarget,

    // Features
    romfs: Option<RomFS>,
    console: Option<RuggieConsole>,
}

impl RuggieLib {
    pub fn new() -> Result<Self, RuggieLibCreationError> {
        let apt = Apt::new().map_err(|_| RuggieLibCreationError::FailedApt)?;
        let hid = Hid::new().map_err(|_| RuggieLibCreationError::FailedHid)?;
        let gfx = Gfx::new().map_err(|_| RuggieLibCreationError::FailedGfx)?;

        unsafe {
            C3D_Init(C3D_DEFAULT_CMDBUF_SIZE as usize);
            C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize);
            C2D_Prepare();
        }

        let top_left_screen = RuggieScreenTarget::new_top_left();
        let top_right_screen = RuggieScreenTarget::new_top_right();
        let bottom_screen = RuggieScreenTarget::new_bottom();

        Ok(Self {
            apt,
            hid,
            gfx,
            top_left_screen,
            top_right_screen,
            bottom_screen,
            romfs: None,
            console: None,
        })
    }

    pub fn start_drawing(&mut self) -> RuggieDrawHandle<'_> {
        RuggieDrawHandle::new(self)
    }

    pub fn is_running(&self) -> bool {
        self.apt.main_loop()
    }

    pub fn wait_for_vblank(&self) {
        self.gfx.wait_for_vblank();
    }
}

impl Drop for RuggieLib {
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
    }
}
