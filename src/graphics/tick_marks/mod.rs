//! Structs for constructing a group of tick marks.

use iced_native::{Point, Rectangle};
use std::cell::RefCell;
use std::sync::Arc;

pub use crate::native::tick_marks::*;
pub use crate::style::tick_marks::*;

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
    pub tick_marks_hash: u64,
    pub style: Style,
    pub placement: Placement,
    pub inverse: bool,

    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub angle_span: f32,
    pub inside: bool,
}

impl Default for PrimitiveCacheData {
    fn default() -> Self {
        Self {
            cache: Arc::new(iced_graphics::Primitive::None),

            bounds: Rectangle::default(),
            tick_marks_hash: 0,
            style: Style::default(),
            placement: Placement::default(),
            inverse: false,

            center: Point::default(),
            radius: 0.0,
            start_angle: 0.0,
            angle_span: 0.0,
            inside: false,
        }
    }
}

impl std::fmt::Debug for PrimitiveCacheData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// A cache for tick mark primitives.
#[derive(Debug, Clone)]
pub struct PrimitiveCache {
    data: RefCell<PrimitiveCacheData>,
}

impl PrimitiveCache {
    /// Cache and retrieve linear tick marks.
    pub fn cached_linear<F: Fn() -> iced_graphics::Primitive>(
        &self,
        bounds: Rectangle,
        tick_marks: &Group,
        style: Style,
        placement: Placement,
        inverse: bool,
        builder: F,
    ) -> iced_graphics::Primitive {
        let mut data = self.data.borrow_mut();

        if !(data.bounds == bounds
            && data.tick_marks_hash == tick_marks.hashed()
            && data.style == style
            && data.placement == placement
            && data.inverse == inverse)
        {
            data.bounds = bounds;
            data.tick_marks_hash = tick_marks.hashed();
            data.style = style;
            data.placement = placement;
            data.inverse = inverse;

            data.cache = Arc::new(builder());
        }

        iced_graphics::Primitive::Cached {
            cache: Arc::clone(&data.cache),
        }
    }

    /// Cache and retrieve radial tick marks.
    pub fn cached_radial<F: Fn() -> iced_graphics::Primitive>(
        &self,
        center: Point,
        radius: f32,
        start_angle: f32,
        angle_span: f32,
        inside: bool,
        tick_marks: &Group,
        style: Style,
        inverse: bool,
        builder: F,
    ) -> iced_graphics::Primitive {
        let mut data = self.data.borrow_mut();

        if !(data.center == center
            && data.radius == radius
            && data.start_angle == start_angle
            && data.angle_span == angle_span
            && data.inside == inside
            && data.tick_marks_hash == tick_marks.hashed()
            && data.style == style
            && data.inverse == inverse)
        {
            data.center = center;
            data.radius = radius;
            data.start_angle = start_angle;
            data.angle_span = angle_span;
            data.inside = inside;
            data.tick_marks_hash = tick_marks.hashed();
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
