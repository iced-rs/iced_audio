//! Structs for constructing a group of text marks.

pub use crate::native::text_marks::*;
pub use crate::style::text_marks::*;

mod horizontal;
mod radial;
mod vertical;

pub use horizontal::*;
pub use radial::*;
pub use vertical::*;
