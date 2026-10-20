#![feature(custom_test_frameworks)]
#![feature(array_try_from_fn)]
#![test_runner(test_runner::run_console)]
#![doc = include_str!("../README.md")]

/// The internal module.
///
/// This module exists solely for organization sake, allowing lib.rs to be clean of structs and
/// module declarations. It contains all of the library functionality.
pub mod ruggielib_internal;
pub use ruggielib_internal::*;

/// The `RuggieLib` errors module
///
/// This module is meant to bring all possible errors implemented throught the library togheter into
/// one huge module, without sacrificing development ergonomics
pub mod errors {
    pub use crate::ruggielib_internal::errors::*;
    pub use crate::ruggielib_internal::features::errors::*;
    pub use crate::ruggielib_internal::input::errors::*;
}

/// The `RuggieLib` prelude
///
/// This module is meant to bring the most useful types, functions and constants into scope all at
/// once 
///
/// # Examples
///
/// ```
/// use ruggie_lib::prelude::*;
/// ```
pub mod prelude {
    // RuggieLib
    pub use crate::RuggieLib;
    pub use crate::features::console::{BOTTOM_SCREEN_INDEX, TOP_SCREEN_INDEX};

    // Features
    pub use crate::features::Feature;

    // 2D Rendering
    pub use crate::screen::RuggieScreenTarget;
    pub use crate::draw::color::Color;
    pub use crate::draw::sprite::Sprite;
    pub use crate::draw::sprite_sheet::SpriteSheet;

    // Errors
    pub use crate::errors::*;
}
