use ctru_sys::{PrintConsole, consoleClear, consoleInit, consoleSelect, gfxScreen_t};

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

    pub fn select(&mut self) {
        unsafe { consoleSelect(self.0.as_mut()) };
    }

    pub fn clear(&mut self) {
        unsafe { consoleClear() };
    }
}
