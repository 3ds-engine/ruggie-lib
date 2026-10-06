use citro2d_sys::{C2D_CreateScreenTarget, C3D_RenderTarget};


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
    #[must_use]
    pub fn new_top_left() -> Self {
        // SAFETY: Using an ffi function that creates a mutable pointer, otherwise the pointer is
        // never derefed in this block
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(TOP, LEFT);

            Self { 
                screen: top_screen,
                width: TOP_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    #[must_use]
    pub fn new_top_right() -> Self {
        // SAFETY: Using an ffi function that creates a mutable pointer, otherwise the pointer is
        // never derefed in this block
        unsafe { 
            let top_screen = C2D_CreateScreenTarget(TOP, RIGHT);

            Self { 
                screen: top_screen,
                width: TOP_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    #[must_use]
    pub fn new_bottom() -> Self {
        // SAFETY: Using an ffi function that creates a mutable pointer, otherwise the pointer is
        // never derefed in this block
        unsafe { 
            let bottom_screen = C2D_CreateScreenTarget(BOTTOM, LEFT);
            Self { 
                screen: bottom_screen,
                width: BOT_WIDTH,
                height: SCREEN_HEIGHT,
            }
        }
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn get_screen(&self) -> &mut C3D_RenderTarget {
        // SAFETY: Converting a mutable pointer that must be valid (since the only way to access a
        // RuggieScreenTarget is by creating said pointer) to a mutable reference is always safe
        unsafe {
            self.screen.as_mut_unchecked()
        }
    }
}
