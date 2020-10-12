//! Structs for constructing a group of tick marks.

pub use crate::native::tick_marks::*;
pub use crate::style::tick_marks::*;

mod horizontal;
mod radial;
mod vertical;

pub use horizontal::*;
pub use radial::*;
pub use vertical::*;
