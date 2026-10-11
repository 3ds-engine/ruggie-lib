use ctru_sys::{
    GFX_BOTTOM, GFX_TOP, PrintConsole, consoleClear, consoleInit, consoleSelect, gfxScreen_t,
};

use crate::RuggieLib;

/// The necessary identifier to build a top screen console
///
/// # Example
///
/// ```
/// let rl = RuggieLib::new().unwrap().with([ Feature::Console(TOP_SCREEN_INDEX) ]).unwrap();
/// ```
pub const TOP_SCREEN_INDEX: u8 = GFX_TOP;

/// The necessary identifier to build a bottom screen console
///
/// # Example
///
/// ```
/// let rl = RuggieLib::new().unwrap().with([ Feature::Console(BOTTOM_SCREEN_INDEX) ]).unwrap();
/// ```
pub const BOTTOM_SCREEN_INDEX: u8 = GFX_BOTTOM;

/// Ruggie's console output handler
pub struct RuggieConsole(/*Box<PrintConsole>*/);

/// Main console funtionality
#[doc(hidden)]
impl RuggieConsole {
    #[must_use]
    pub fn new(screen: gfxScreen_t) -> Self {
        let mut console: Box<PrintConsole> = Box::new(unsafe { std::mem::zeroed() });

        unsafe {
            consoleInit(screen, console.as_mut());
            consoleSelect(console.as_mut());
        }

        Self(/*console*/)
    }
}

/// Console funtionality
impl RuggieLib {
    /// Clears the active console buffer if there is one
    pub fn clear_console(&mut self) {
        if self.console.is_some() {
            unsafe {
                consoleClear();
            }
        }
    }
}
