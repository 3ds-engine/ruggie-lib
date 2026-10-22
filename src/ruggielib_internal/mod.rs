/// `RuggieLib` errors
///
/// This contains all errors that `RuggieLib`'s base functionality can return in Results
pub mod errors;
use errors::RuggieLibCreationError;

/// The module for drawing 2D graphics on screen
///
/// This module holds not only the `RuggieDrawHandle` for drawing on screen but also important modules for
/// rendering like color, `sprite` and `sprite_sheet`
pub mod draw;
use draw::RuggieDrawHandle;

/// The `ScreenTarget` module
///
/// Holds the screen representation and some useful consts that define window size and internal
/// indexing
pub mod screen;
use screen::RuggieScreenTarget;

/// The features module
///
/// Contains all the optional functionality that `RuggieLib` may have if the user decides to enable it
pub mod features;
use crate::{audio::audio_handler::AudioHandler, features::console::RuggieConsole};

/// The input module
///
/// Contains the functions associated with the input of the 3DS.
pub mod input;

pub mod audio;

use citro2d_sys::{C2D_DEFAULT_MAX_OBJECTS, C2D_Fini, C2D_Init, C2D_Prepare};
use citro3d_sys::{C3D_DEFAULT_CMDBUF_SIZE, C3D_Fini, C3D_Init};
use ctru::{prelude::*, services::romfs::RomFS};

/// The main API that abstracts all 3DS libraries away
///
/// It contains all the functionality outside 2D and 3D rendering
///
/// # Example
///
/// ```
/// fn main() {
///     let rl = RuggieLib::new().unwrap();
///     while(rl.is_running()) {
///         rl.wait_for_vblank();
///         // Your code here
///     }
/// }
/// ```
///
/// # Notes
///
/// If you try to use some of `RuggieLib`'s funtionality without creating an instance of `RuggieLib`
/// frist, you may encounter undefined behaviour. **THIS STRUCT SHOULD BE THE FIRST THING YOU CREATE IN YOUR
/// PROGRAM**
pub struct RuggieLib {
    // Base tools
    apt: Apt,
    hid: Hid,
    gfx: Gfx,
    audio_handler: AudioHandler,
    // Screens
    pub top_left_screen: RuggieScreenTarget,
    pub top_right_screen: RuggieScreenTarget,
    pub bottom_screen: RuggieScreenTarget,

    // Features
    romfs: Option<RomFS>,
    console: Option<RuggieConsole>,
}

/// Generic `RuggieLib` utilities and struct construction
impl RuggieLib {
    /// The main way to create an API instance
    ///
    /// # Errors
    ///
    /// It may return a `RuggieLibCreationError` if it fails to build Apt, Hid or Gfx, which are
    /// necesarry components of the 3DS application architecture
    pub fn new() -> Result<Self, RuggieLibCreationError> {
        let apt = Apt::new().map_err(|_| RuggieLibCreationError::FailedApt)?;
        let hid = Hid::new().map_err(|_| RuggieLibCreationError::FailedHid)?;
        let gfx = Gfx::new().map_err(|_| RuggieLibCreationError::FailedGfx)?;
        let audio_handler = AudioHandler::new().map_err(|_| RuggieLibCreationError::FailedAudioHandler)?;

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
            audio_handler,
            top_left_screen,
            top_right_screen,
            bottom_screen,
            romfs: None,
            console: None,
        })
    }

    /// Get access to the draw handle
    ///
    /// While the draw handle is in use, you lose access to `RuggieLib`'s API because `RuggieDrawHandle`
    /// takes a mutable reference of `RuggieLib`. This is meant as a restriction: **the rendering
    /// process should be done in one step, and rendering should be independent of any of the
    /// library's other utilities**
    ///
    /// # Example
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_vblank();
    ///     let mut render = rl.start_drawing();
    ///     render.clear_screen(Color::white());
    ///     render.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::red());
    /// }
    /// ```
    pub fn start_drawing(&mut self) -> RuggieDrawHandle<'_> {
        RuggieDrawHandle::new(self)
    }

    /// Check if the application is still running
    ///
    /// Should be used as a main loop condition
    ///
    /// # Example
    ///
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     // Your code here
    /// }
    /// ```
    pub fn is_running(&self) -> bool {
        self.apt.main_loop()
    }

    /// Waits until screen renderer is drawing vblank.
    ///
    /// Used to apply vsync to frame rendering. Should be used before rendering anything to screen
    ///
    /// # Example
    ///
    /// ```
    /// let rl = RuggieLib::new().unwrap();
    /// while(rl.is_running()) {
    ///     rl.wait_for_blank();
    ///     // Your render code here
    /// }
    /// ```
    pub fn wait_for_vblank(&self) {
        self.gfx.wait_for_vblank();
    }
}

/// Makes it so that every initialized resource is de-initialized when `RuggieLib` is dropped
impl Drop for RuggieLib {
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
    }
}
