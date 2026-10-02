pub mod deprecated;

pub mod errors;
use ctru_sys::osGet3DSliderState;
use errors::RuggieLibCreationError;

pub mod draw;
use draw::RuggieDrawHandle;

pub mod screen;
use screen::RuggieScreenTarget;

use citro2d_sys::*;
use citro3d_sys::*;
use ctru::{prelude::*, services::{gfx::TopScreen3D, romfs::RomFS}};

pub mod tests;
pub struct RuggieLib {
    // Base tools
    apt: Apt,
    hid: Hid,
    gfx: Gfx,

    // Screens
    pub top_left_screen : RuggieScreenTarget,
    pub top_right_screen : RuggieScreenTarget,
    pub bottom_screen : RuggieScreenTarget,

    // Features
    romfs: Option<RomFS>,
}

impl RuggieLib {
    pub fn new() -> Result<Self, RuggieLibCreationError> {
        let apt = Apt::new().map_err(|_| RuggieLibCreationError::FailedApt)?;
        let hid = Hid::new().map_err(|_| RuggieLibCreationError::FailedHid)?;
        let gfx = Gfx::new().map_err(|_| RuggieLibCreationError::FailedGfx)?;
        
        unsafe{
            citro3d_sys::C3D_Init(C3D_DEFAULT_CMDBUF_SIZE as usize);
            citro2d_sys::C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize);
            C2D_Prepare();
        }

        let top_left_screen = RuggieScreenTarget::new_top_left();
        let top_right_screen = RuggieScreenTarget::new_top_right();
        let bottom_screen = RuggieScreenTarget::new_bottom();

        Ok(RuggieLib {
            apt,
            hid,
            gfx,
            top_left_screen,
            top_right_screen,
            bottom_screen,
            romfs: None,
        })
    }

    pub fn start_drawing(&mut self) -> RuggieDrawHandle<'_> {
        RuggieDrawHandle::new(self)
    }

    pub fn with_romfs(mut self) -> ctru::Result<Self> {
        let romfs = RomFS::new()?;
        self.romfs = Some(romfs);

        Ok(self)
    }

    pub fn with(mut self, features: impl IntoIterator<Item=Feature>) -> ctru::Result<Self> {
        for feature in features.into_iter() {
            match feature {
                Feature::RomFS => {
                    let romfs = RomFS::new()?;
                    self.romfs = Some(romfs)
                },
                Feature::Stereoscopic3D =>{
                    unsafe{
                        ctru_sys::gfxSet3D(true);
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn is_running(&self) -> bool{
        self.apt.main_loop()
    }

    pub fn wait_for_vblank(&self){
        self.gfx.wait_for_vblank();
    }

    pub fn get_3d_slider_state(&self) -> f32{
        unsafe{
            osGet3DSliderState()
        }
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

pub enum Feature {
    RomFS,
    Stereoscopic3D
}
