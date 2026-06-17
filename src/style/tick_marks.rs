//! Various styles for a [`tick_marks::Group`] in a bar meter widget
//!
//! [`tick_marks::Group`]: ../../native/tick_marks/struct.Group.html
use iced::Color;

use crate::core::Offset;
use crate::style::default_colors;

/// The placement of tick marks relative to the widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placement {
    /// Tick marks on both sides of the widget.
    BothSides {
        /// The offset from the edge of the widget.
        offset: Offset,
        /// Whether to place the tick marks inside the widget (true) or
        /// outside the widget (false).
        inside: bool,
    },
    /// Tick marks only on the outside left/top side of the widget.
    LeftOrTop {
        /// The offset from the edge of the widget.
        offset: Offset,
        /// Whether to place the tick marks inside the widget (true) or
        /// outside the widget (false).
        inside: bool,
    },
    /// Tick marks only on the right/bottom side of the widget.
    RightOrBottom {
        /// The offset from the edge of the widget.
        offset: Offset,
        /// Whether to place the tick marks inside the widget (true) or
        /// outside the widget (false).
        inside: bool,
    },
    /// Tick marks in the center of the widget.
    Center {
        /// The offset from the center of the widget.
        offset: Offset,
        /// Whether to fill the length of the widget (true), or not (false).
        /// If this is true, then the length of each tick mark will act as the
        /// padding from the edge of the widget to the tick mark.
        fill_length: bool,
    },
    /// Split tick marks in the center of the widget.
    CenterSplit {
        /// The offset from the center of the widget.
        offset: Offset,
        /// Whether to fill the length of the widget (true), or not (false).
        /// If this is true, then the length of each tick mark will extend from
        /// the edges of the widget.
        fill_length: bool,
        /// The gap between the split tick marks. This has no effect if `fill_length`
        /// is true.
        gap: f32,
    },
}

impl std::default::Default for Placement {
    fn default() -> Self {
        Placement::BothSides {
            offset: Default::default(),
            inside: false,
        }
    }
}

/// The appearance of a tick mark
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Appearance {
    /// The style of a tier 1 tick mark.
    pub tier_1: Shape,
    /// The style of a tier 2 tick mark.
    pub tier_2: Shape,
    /// The style of a tier 3 tick mark.
    pub tier_3: Shape,
}

/// The shape of a tick mark
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    /// No shape
    None,
    /// Line shape
    Line {
        /// The length of the tick mark.
        length: f32,

        /// The width (thickness) of the tick mark.
        width: f32,

        /// The color of the tick mark.
        color: Color,
    },
    /// Circle shape
    Circle {
        /// The diameter of the tick mark.
        diameter: f32,

        /// The color of the tick mark.
        color: Color,
    },
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            tier_1: Shape::Line {
                length: 4.0,
                width: 2.0,
                color: default_colors::TICK_TIER_1,
            },
            tier_2: Shape::Line {
                length: 3.0,
                width: 2.0,
                color: default_colors::TICK_TIER_2,
            },
            tier_3: Shape::Line {
                length: 2.0,
                width: 1.0,
                color: default_colors::TICK_TIER_3,
            },
        }
    }
}
