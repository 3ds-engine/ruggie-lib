use citro2d_sys::*;

const TOP: u8 = 0;
const LEFT: u8 = 0;

const BOTTOM: u8 = 1;
const RIGHT: u8 = 1;

#[derive(Copy, Clone)]
pub struct RuggieScreenTarget{
    pub screen: *mut C3D_RenderTarget,
}

impl RuggieScreenTarget{
    pub fn new_top_screen() -> Self{
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(TOP, LEFT);
            RuggieScreenTarget { screen: top_screen }
        }
    }

    pub fn new_bottom_screen() -> Self {
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(BOTTOM, LEFT);
            RuggieScreenTarget { screen: top_screen }
        }
    }
}