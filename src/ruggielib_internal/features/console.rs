use ctru_sys::{PrintConsole, consoleClear, consoleInit, consoleSelect, gfxScreen_t};

use crate::RuggieLib;

pub struct RuggieConsole(Box<PrintConsole>);

impl RuggieConsole {
    pub fn new(screen: gfxScreen_t) -> Self {
        let mut console: Box<PrintConsole> = Box::new(unsafe { std::mem::zeroed() });

        unsafe {
            consoleInit(screen, console.as_mut());
            consoleSelect(console.as_mut());
        }

        Self(console)
    }
}

impl RuggieLib {
    pub fn clear_console(&mut self) {
        if self.console.is_some() {
            unsafe {
                consoleClear();
            }
        }
    }
}
