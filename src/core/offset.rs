//! Offset type

use iced_native::Rectangle;

/// A 2D offset vector with a horizontal and vertical offset in pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offset {
    /// The horizontal offset in pixels.
    pub x: i16,
    /// the vertical offset in pixels.
    pub y: i16,
}

impl Offset {
    /// An [`Offset`] with zero x and zero y offset.
    ///
    /// [`Offset`]: struct.Offset.html
    pub const ZERO: Offset = Offset { x: 0, y: 0 };

    /// Creates a new [`Offset`].
    ///
    /// `x` - The horizontal offset in pixels.
    /// `y` - The vertical offset in pixels.
    ///
    /// [`Offset`]: struct.Offset.html
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Returns the x value as an `f32`.
    #[inline]
    pub fn x_f32(&self) -> f32 {
        f32::from(self.x)
    }

    /// Returns the y value as an `f32`.
    #[inline]
    pub fn y_f32(&self) -> f32 {
        f32::from(self.y)
    }

    /// Return an offsetted rectangle.
    #[inline]
    pub fn offset_rect(&self, rect: &Rectangle) -> Rectangle {
        Rectangle {
            x: rect.x + f32::from(self.x),
            y: rect.y + f32::from(self.y),
            width: rect.width,
            height: rect.height,
        }
    }

    /// Offset the given rectangle.
    #[inline]
    pub fn offset_rect_mut(&self, rect: &mut Rectangle) {
        rect.x += f32::from(self.x);
        rect.y += f32::from(self.y);
    }
}

impl Default for Offset {
    fn default() -> Self {
        Offset::ZERO
    }
}

impl From<Offset> for iced_graphics::Point {
    fn from(offset: Offset) -> Self {
        iced_graphics::Point {
            x: f32::from(offset.x),
            y: f32::from(offset.y),
        }
    }
}
