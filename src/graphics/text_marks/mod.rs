//! Structs for constructing a group of text marks.

use iced_native::{Point, Rectangle};
use std::cell::RefCell;
use std::sync::Arc;

pub use crate::native::text_marks::*;
pub use crate::style::text_marks::*;

mod horizontal;
mod radial;
mod vertical;

pub use horizontal::*;
pub use radial::*;
pub use vertical::*;

#[derive(Clone)]
struct PrimitiveCacheData {
    pub cache: Arc<iced_graphics::Primitive>,

    pub bounds: Rectangle,
    pub text_marks_hash: u64,
    pub style: Style,
    pub placement: Placement,
    pub inverse: bool,

    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub angle_span: f32,
}

impl Default for PrimitiveCacheData {
    fn default() -> Self {
        Self {
            cache: Arc::new(iced_graphics::Primitive::None),

            bounds: Rectangle::default(),
            text_marks_hash: 0,
            style: Style::default(),
            placement: Placement::default(),
            inverse: false,

            center: Point::default(),
            radius: 0.0,
            start_angle: 0.0,
            angle_span: 0.0,
        }
    }
}

impl std::fmt::Debug for PrimitiveCacheData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// A cache for text mark primitives.
#[derive(Debug, Clone)]
pub struct PrimitiveCache {
    data: RefCell<PrimitiveCacheData>,
}

impl PrimitiveCache {
    /// Cache and retrieve linear text marks.
    pub fn cached_linear<F: Fn() -> iced_graphics::Primitive>(
        &self,
        bounds: Rectangle,
        text_marks: &Group,
        style: Style,
        placement: Placement,
        inverse: bool,
        builder: F,
    ) -> iced_graphics::Primitive {
        let mut data = self.data.borrow_mut();

        if !(data.bounds == bounds
            && data.text_marks_hash == text_marks.hashed()
            && data.style == style
            && data.placement == placement
            && data.inverse == inverse)
        {
            data.bounds = bounds;
            data.text_marks_hash = text_marks.hashed();
            data.style = style;
            data.placement = placement;
            data.inverse = inverse;

            data.cache = Arc::new(builder());
        }

        iced_graphics::Primitive::Cached {
            cache: Arc::clone(&data.cache),
        }
    }

    /// Cache and retrieve radial text marks.
    pub fn cached_radial<F: Fn() -> iced_graphics::Primitive>(
        &self,
        center: Point,
        radius: f32,
        start_angle: f32,
        angle_span: f32,
        text_marks: &Group,
        style: Style,
        inverse: bool,
        builder: F,
    ) -> iced_graphics::Primitive {
        let mut data = self.data.borrow_mut();

        if !(data.center == center
            && data.radius == radius
            && data.start_angle == start_angle
            && data.angle_span == angle_span
            && data.text_marks_hash == text_marks.hashed()
            && data.style == style
            && data.inverse == inverse)
        {
            data.center = center;
            data.radius = radius;
            data.start_angle = start_angle;
            data.angle_span = angle_span;
            data.text_marks_hash = text_marks.hashed();
            data.style = style;
            data.inverse = inverse;

            data.cache = Arc::new(builder());
        }

        iced_graphics::Primitive::Cached {
            cache: Arc::clone(&data.cache),
        }
    }
}

impl Default for PrimitiveCache {
    fn default() -> Self {
        Self {
            data: RefCell::new(PrimitiveCacheData::default()),
        }
    }
}
