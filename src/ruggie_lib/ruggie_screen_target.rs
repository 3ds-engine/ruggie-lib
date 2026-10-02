use citro2d_sys::*;

const TOP: u8 = 0;
const LEFT: u8 = 0;

const BOTTOM: u8 = 1;
const RIGHT: u8 = 1;

const TOP_WIDTH : u32 = 400;
const BOT_WIDTH : u32 = 320;
const SCREEN_HEIGHT : u32 = 240;

pub struct RuggieScreenTarget {
    screen: *mut C3D_RenderTarget,
    width: u32,
    height: u32,
}

impl RuggieScreenTarget {
    pub fn new_top_left() -> Self {
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(TOP, LEFT);

            RuggieScreenTarget { 
                screen: top_screen,
                width: TOP_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    pub fn new_top_right() -> Self {
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(TOP, RIGHT);

            RuggieScreenTarget { 
                screen: top_screen,
                width: TOP_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    pub fn new_bottom() -> Self {
        unsafe { 
            let bottom_screen = C2D_CreateScreenTarget(BOTTOM, LEFT);
            RuggieScreenTarget { 
                screen: bottom_screen,
                width: BOT_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_screen(&self) -> &mut C3D_RenderTarget {
        unsafe {
            self.screen.as_mut_unchecked()
        }
    }
}
