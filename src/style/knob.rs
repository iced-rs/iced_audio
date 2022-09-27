//! Various styles for the [`Knob`] widget
//!
//! [`Knob`]: ../native/knob/struct.Knob.html

use iced_native::Color;
//use iced_native::image;

pub use iced::widget::canvas::{Canvas, LineCap};

use crate::style::{default_colors, text_marks, tick_marks};
use crate::KnobAngleRange;

/// The appearance of a [`Knob`],
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub enum Style {
    //Texture(TextureStyle),
    /// A classic circular style
    Circle(CircleStyle),
    /// A modern arc style
    Arc(ArcStyle),
    /// A modern arc style with. It can display different colors
    /// for left, right, and center positions.
    ArcBipolar(ArcBipolarStyle),
}

/*
/// A [`Style`] for a [`Knob`] that uses an image texture for the knob
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`Handle`]: https://docs.rs/iced/0.1.1/iced/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    /// the [`Handle`] to the image texture
    pub texture: image::Handle,
    /// the width of the knob, not including padding
    pub knob_width: u16,
    /// the height of the knob, not including padding
    pub knob_height: u16,
    /// the texture padding around the knob bounding
    /// rectangle. This is useful when the texture is of a glowing handle or has
    /// a drop shadow, etc.
    pub texture_padding: Option<TexturePadding>,
}
*/

/// A length in a [`Knob`] stylesheet
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub enum StyleLength {
    /// The diameter of the knob scaled to this value
    Scaled(f32),
    /// Absolute length in pixels
    Units(f32),
}

impl StyleLength {
    /// Returns the length based on the given knob diameter
    #[inline]
    pub fn from_knob_diameter(&self, knob_diameter: f32) -> f32 {
        match self {
            StyleLength::Scaled(scale) => knob_diameter * *scale,
            StyleLength::Units(units) => *units,
        }
    }
}

/// Circle notch
#[derive(Debug, Clone)]
pub struct CircleNotch {
    /// The color of the circle
    pub color: Color,
    /// The width of the border
    pub border_width: f32,
    /// The color of the border
    pub border_color: Color,
    /// The diameter of the circle
    pub diameter: StyleLength,
    /// The offset from the edge of the knob to the center of the notch.
    pub offset: StyleLength,
}

/// Line notch
#[derive(Debug, Clone)]
pub struct LineNotch {
    /// The color of the line
    pub color: Color,
    /// The width (thickness) of the line
    pub width: StyleLength,
    /// The length of the line
    pub length: StyleLength,
    /// The cap at the ends of the line
    pub cap: LineCap,
    /// The offset from the edge of the knob to the center of the notch.
    pub offset: StyleLength,
}

/// The shape of the notch
#[derive(Debug, Clone)]
pub enum NotchShape {
    /// No notch
    None,
    /// Circle notch
    Circle(CircleNotch),
    /// Line notch
    Line(LineNotch),
}

/// A classic circular [`Style`] of a [`Knob`]
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct CircleStyle {
    /// The color of the knob
    pub color: Color,
    /// The width of the border around the knob
    pub border_width: f32,
    /// The color of the border around the knob
    pub border_color: Color,
    /// The shape of the notch
    pub notch: NotchShape,
}

/// A modern arc [`Style`] of a [`Knob`]
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct ArcStyle {
    /// The width (thickness) of the arc
    pub width: StyleLength,
    /// The color of an empty portion of the arc
    pub empty_color: Color,
    /// The color of the filled portion of the arc
    pub filled_color: Color,
    /// The shape of the notch
    pub notch: NotchShape,
    /// The cap at the ends of the arc
    pub cap: LineCap,
}

/// A modern arc [`Style`] of a [`Knob`].
/// It can display different colors for left, right, and center positions. The filled arc
/// color draws from the center position.
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct ArcBipolarStyle {
    /// The width (thickness) of the arc
    pub width: StyleLength,
    /// The color of the empty background portion of the arc
    pub empty_color: Color,
    /// The color of the filled portion to the left of the center
    pub left_filled_color: Color,
    /// The color of the filled portion to the right of the center
    pub right_filled_color: Color,
    /// The shape of the notch when in the center position
    pub notch_center: NotchShape,
    /// The shape of the notch when it is to the left and right of the
    /// center. Set this to `None` to only use `notch_center`.
    pub notch_left_right: Option<(NotchShape, NotchShape)>,
    /// The cap at the ends of the arc
    pub cap: LineCap,
}

/// A style for a value arc around a [`Knob`]
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct ValueArcStyle {
    /// The width (thickness) of the arc
    pub width: f32,
    /// The offset from the edge of the `Knob` in pixels
    pub offset: f32,
    /// The color of the empty background portion in the arc. Set this to
    /// `None` for no background arc.
    pub empty_color: Option<Color>,
    /// The color of a filled portion of the ring. If `right_filled_color` is
    /// `Some`, then this will only apply to the left side of the ring.
    pub left_filled_color: Color,
    /// The color of a filled portion on the right side of the ring.
    /// Set this to `None` for unipolar mode.
    pub right_filled_color: Option<Color>,
    /// The cap at the ends of the arc
    pub cap: LineCap,
}

/// A style for a [`ModulationRange`] arc around a [`Knob`]
///
/// [`ModulationRange`]: ../../core/struct.ModulationRange.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct ModRangeArcStyle {
    /// The width (thickness) of the arc
    pub width: f32,
    /// The offset from the edge of the `Knob` in pixels
    pub offset: f32,
    /// The color of an empty background portion in the arc. Set this to
    /// `None` for no background arc.
    pub empty_color: Option<Color>,
    /// The color of a filled portion of the arc
    pub filled_color: Color,
    /// The color of a filled portion of the arc when `end` is less than
    /// `start`
    pub filled_inverse_color: Color,
    /// The cap at the ends of the arc
    pub cap: LineCap,
}

/// Style of tick marks for a [`Knob`].
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct TickMarksStyle {
    /// The style of the tick marks
    pub style: tick_marks::Style,
    /// The offset from the edge of the knob in pixels
    pub offset: f32,
}

/// Style of text marks for a [`Knob`].
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct TextMarksStyle {
    /// The style of the text marks
    pub style: text_marks::Style,
    /// The offset from the edge of the knob in pixels
    pub offset: f32,
    /// Extra horizontal offset in pixels for each additional character
    /// in the text label. This is used to keep longer labels on the sides
    /// of the knob from being too close to the knob.
    ///
    /// The default is `3.0`.
    pub h_char_offset: f32,
    /// The vertical offset in pixels.
    ///
    /// The default is `-0.75`.
    pub v_offset: f32,
}

impl std::default::Default for TextMarksStyle {
    fn default() -> Self {
        Self {
            style: text_marks::Style::default(),
            offset: 15.0,
            h_char_offset: 3.0,
            v_offset: -0.75,
        }
    }
}

/// A set of rules that dictate the style of a [`Knob`].
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
pub trait StyleSheet {
    /// Produces the style of an active [`Knob`].
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`Knob`].
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn hovered(&self) -> Style;

    /// Produces the style of a [`Knob`] that is being dragged.
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn dragging(&self) -> Style;

    /// a [`KnobAngleRange`] that defines the minimum and maximum angle that the
    /// knob rotates
    ///
    /// [`KnobAngleRange`]: struct.KnobAngleRange.html
    fn angle_range(&self) -> KnobAngleRange {
        KnobAngleRange::default()
    }

    /// The style of tick marks around a [`Knob`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        None
    }

    /// The style of a value arc around a [`Knob`]
    ///
    /// For no value arc, don't override this or set this to return `None`.
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn value_arc_style(&self) -> Option<ValueArcStyle> {
        None
    }

    /// The style of a [`ModulationRange`] arc around a [`Knob`]
    ///
    /// For no modulation range arc, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn mod_range_arc_style(&self) -> Option<ModRangeArcStyle> {
        None
    }

    /// The style of a second [`ModulationRange`] arc around a [`Knob`]
    ///
    /// For no second modulation range arc, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn mod_range_arc_style_2(&self) -> Option<ModRangeArcStyle> {
        None
    }

    /// The style of text marks around a [`Knob`]
    ///
    /// For no text marks, don't override this or set this to return `None`.
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn text_marks_style(&self) -> Option<TextMarksStyle> {
        None
    }
}

struct Default;
impl Default {
    const ACTIVE_CIRCLE_STYLE: CircleStyle = CircleStyle {
        color: default_colors::LIGHT_BACK,
        border_width: 1.0,
        border_color: default_colors::BORDER,
        notch: NotchShape::Circle(CircleNotch {
            color: default_colors::BORDER,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            diameter: StyleLength::Scaled(0.17),
            offset: StyleLength::Scaled(0.15),
        }),
    };
}
impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Circle(Self::ACTIVE_CIRCLE_STYLE)
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> Style {
        Style::Circle(CircleStyle {
            color: default_colors::KNOB_BACK_HOVER,
            ..Self::ACTIVE_CIRCLE_STYLE
        })
    }

    fn dragging(&self) -> Style {
        self.hovered()
    }

    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: tick_marks::Style {
                tier_1: tick_marks::Shape::Circle {
                    diameter: 4.0,
                    color: default_colors::TICK_TIER_1,
                },
                tier_2: tick_marks::Shape::Circle {
                    diameter: 2.0,
                    color: default_colors::TICK_TIER_2,
                },
                tier_3: tick_marks::Shape::Circle {
                    diameter: 2.0,
                    color: default_colors::TICK_TIER_3,
                },
            },
            offset: 3.5,
        })
    }

    fn text_marks_style(&self) -> Option<TextMarksStyle> {
        Some(TextMarksStyle {
            style: text_marks::Style::default(),
            offset: 14.0,
            h_char_offset: 3.0,
            v_offset: -0.75,
        })
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
