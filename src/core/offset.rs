//! Offset type

use iced::{Point, Rectangle};

/// A 2D offset vector with a horizontal and vertical offset in pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offset {
    /// The horizontal offset in pixels.
    pub x: f32,
    /// the vertical offset in pixels.
    pub y: f32,
}

impl Offset {
    /// An [`Offset`] with zero x and zero y offset.
    ///
    /// [`Offset`]: struct.Offset.html
    pub const ZERO: Offset = Offset { x: 0.0, y: 0.0 };

    /// Creates a new [`Offset`].
    ///
    /// `x` - The horizontal offset in pixels.
    /// `y` - The vertical offset in pixels.
    ///
    /// [`Offset`]: struct.Offset.html
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Return an offsetted rectangle.
    #[inline]
    pub fn offset_rect(&self, rect: &Rectangle) -> Rectangle {
        Rectangle {
            x: rect.x + self.x,
            y: rect.y + self.y,
            width: rect.width,
            height: rect.height,
        }
    }

    /// Offset the given rectangle.
    #[inline]
    pub fn offset_rect_mut(&self, rect: &mut Rectangle) {
        rect.x += self.x;
        rect.y += self.y;
    }
}

impl Default for Offset {
    fn default() -> Self {
        Offset::ZERO
    }
}

impl From<Offset> for Point {
    fn from(offset: Offset) -> Self {
        Point {
            x: offset.x,
            y: offset.y,
        }
    }
}
