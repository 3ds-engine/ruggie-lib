use citro2d_sys::{C2D_CreateScreenTarget, C3D_RenderTarget};

const TOP: u8 = 0;
const LEFT: u8 = 0;

const BOTTOM: u8 = 1;
const RIGHT: u8 = 1;

const TOP_WIDTH : u32 = 400;
const BOT_WIDTH : u32 = 320;
const SCREEN_HEIGHT : u32 = 240;

/// Ruggie's handler for the 3ds screens
pub struct RuggieScreenTarget {
    screen: *mut C3D_RenderTarget,
    width: u32,
    height: u32,
}

impl RuggieScreenTarget {

    /// Creates a new handler of the top left screen. It is meant to be used by `RuggieLib` in its
    /// initialization
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

    /// Creates a new handler of the top right screen. It is meant to be used by `RuggieLib` in its
    /// initialization
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

    /// Creates a new handler of the bottom screen. It is meant to be used by `RuggieLib` in its
    /// initialization
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

    /// Gets the width of the contained screen. 
    ///
    /// # Example 
    ///
    /// ```
    /// let screen = RuggieScreenTarget::new_top_left();
    /// assert_eq!(screen.width(), 400);
    /// ```
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Gets the height of the contained screen. 
    ///
    /// # Example 
    ///
    /// ```
    /// let screen = RuggieScreenTarget::new_top_left();
    /// assert_eq!(screen.height(), 240);
    /// ```
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[doc(hidden)]
    #[must_use]
    pub const unsafe fn get_screen_raw_mut(&self) -> *mut C3D_RenderTarget {
        // Accesing the screen as a raw pointer can deref it and therefore it is not safe
        self.screen
    }
}
