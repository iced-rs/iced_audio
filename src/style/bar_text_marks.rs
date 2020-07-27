//! Various styles for a [`TextMarkGroup`] in a bar meter widget
///
/// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
use iced::{Color, Font};

use crate::style::default_colors;

/// The placement of text marks relative to the widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placement {
    /// Text marks only on the left/top side
    LeftOrTop,
    /// Text marks only on the right/bottom side
    RightOrBottom,
    /// Text marks on both sides
    BothSides,
}

impl std::default::Default for Placement {
    fn default() -> Self {
        Placement::LeftOrTop
    }
}

/// The style of a [`TextMarkGroup`] for a bar meter widget
///
/// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the text
    pub color: Color,
    /// The offset of the text marks from the side of the widget
    pub offset: u16,
    /// The size of the text
    pub text_size: u16,
    /// The font of the text
    pub font: Font,
    /// The width of the text bounds
    pub bounds_width: u16,
    /// The height of the text bounds
    pub bounds_height: u16,
    /// The placement of the text marks
    pub placement: Placement,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            color: default_colors::TEXT_MARK,
            offset: 8,
            text_size: 12,
            font: Default::default(),
            bounds_width: 30,
            bounds_height: 14,
            placement: Placement::default(),
        }
    }
}
