#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_console)]

pub mod ruggielib_internal;
pub use ruggielib_internal::*;

pub mod prelude{
    pub use crate::RuggieLib;
    pub use crate::screen::RuggieScreenTarget;
    pub use crate::draw::color::Color;
}
