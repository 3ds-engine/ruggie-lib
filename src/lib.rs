pub mod ruggielib_internal;
pub use ruggielib_internal::*;

pub mod prelude{
    use super::*;
    pub use crate::RuggieLib;
    pub use screen::RuggieScreenTarget;
    pub use draw::color::Color;
}
