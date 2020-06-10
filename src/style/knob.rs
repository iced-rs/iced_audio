//! Various styles for the [`Knob`] widget
//!
//! [`Knob`]: ../native/knob/struct.Knob.html

use iced::Color;
//use iced_native::image;

use crate::style::default_colors;
use crate::{KnobAngleRange, Normal};

/// The appearance of a [`Knob`],
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub enum Style {
    //Texture(TextureStyle),
    /// a classic vector style with a circle as the notch
    VectorCircle(VectorCircleStyle),
    /// a classic vector style with a line as the notch
    VectorLine(VectorLineStyle),
    /// a modern arc style with an optional line as the notch
    Arc(ArcStyle),
    /// a modern arc style with an optional line as the notch. It can
    /// display different colors for left, right, and center positions.
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

/// a classic vector [`Style`] of a [`Knob`] witch a circle as the notch
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct VectorCircleStyle {
    /// the color of the knob
    pub knob_color: Color,
    /// the width of the border around the knob
    pub knob_border_width: u16,
    /// the color of the border around the knob
    pub knob_border_color: Color,
    /// the color of the notch line
    pub notch_color: Color,
    /// the width of the border around the notch
    pub notch_border_width: u16,
    /// the color of the border around the notch
    pub notch_border_color: Color,
    /// the scale of the notch from the size of the knob. For example, a scale
    /// of `0.5.into()` will have the notch's diameter be half of the knob's
    /// diameter.
    pub notch_scale: Normal,
    /// he offset of the notch from the edge of the knob to it's center. For
    /// example, `0.0.into()` will have the notch touching the edge of the knob,
    /// and `0.5.into()` will have the notch halfway between the edge and the
    /// center of the knob.
    pub notch_offset: Normal,
}

/// a classic vector [`Style`] of a [`Knob`] with a line as the notch
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`InnerCircle`]: struct.InnerCircle.html
#[derive(Debug, Clone)]
pub struct VectorLineStyle {
    /// the color of the knob
    pub knob_color: Color,
    /// the width of the border around the knob
    pub knob_border_width: u16,
    /// the color of the border around the knob
    pub knob_border_color: Color,
    /// the color of the notch line
    pub notch_color: Color,
    /// the width of the notch line
    pub notch_width: f32,
    /// the scale (height) of the notch line compared to the radius of the knob
    pub notch_scale: Normal,
    /// the offset of the notch line from the edge of the knob compared to the
    /// radius of the knob
    pub notch_offset: Normal,
}

/// a modern arc [`Style`] of a [`Knob`] with an optional [`ArcNotch`]
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`ArcNotch`]: struct.ArcNotch.html
#[derive(Debug, Clone)]
pub struct ArcStyle {
    /// the width (thickness) of the arc
    pub arc_width: f32,
    /// the color of an empty portion of the arc
    pub arc_empty_color: Color,
    /// the color of the filled portion of the arc
    pub arc_filled_color: Color,
    /// an option notch to display
    pub notch: Option<ArcNotch>,
}

/// The notch for the [`ArcStyle`] of a [`Knob`]
///
/// [`ArcStyle`]: struct.ArcStyle.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct ArcNotch {
    /// the width (thickness) of the notch
    pub width: f32,
    /// the length of the notch compared to the radius of the [`Knob`]
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    pub length_scale: Normal,
    /// the color of the notch
    pub color: Color,
}

/// a modern arc [`Style`] of a [`Knob`] with an optional [`ArcBipolarNotch`].
/// It can display different colors for left, right, and center positions. The filled arc
/// color draws from the center position.
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`ArcBipolarNotch`]: struct.ArcBipolarNotch.html
#[derive(Debug, Clone)]
pub struct ArcBipolarStyle {
    /// the width (thickness) of the arc
    pub arc_width: f32,
    /// the color of an empty portion of the arc
    pub arc_empty_color: Color,
    /// the color of the filled portion of the arc left of the center
    pub arc_left_color: Color,
    /// the color of the filled portion of the arc right of the center
    pub arc_right_color: Color,
    /// an optional notch to display
    pub notch: Option<ArcBipolarNotch>,
}

/// The notch for the [`ArcBipolarStyle`] of a [`Knob`]
///
/// [`ArcBipolarStyle`]: struct.ArcBipolarStyle.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Clone)]
pub struct ArcBipolarNotch {
    /// the width (thickness) of the notch
    pub width: f32,
    /// the length of the notch compared to the radius of the [`Knob`]
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    pub length_scale: Normal,
    /// the color of the notch when it is in the center
    pub color_center: Color,
    /// the color of the notch when it is left of the center
    pub color_left: Color,
    /// the color of the notch when it is right of the center
    pub color_right: Color,
}

/// The style of a [`TickMarkGroup`] for a [`Knob`]
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub enum TickMarkStyle {
    /// A style with circular tick marks.
    Circle(CircleTickMarks),
    /// A style with line tick marks.
    Line(LineTickMarks),
}

impl std::default::Default for TickMarkStyle {
    fn default() -> Self {
        TickMarkStyle::Circle(CircleTickMarks::default())
    }
}

/// A circular [`TickMarkStyle`] for a [`Knob`]
///
/// [`TickMarkStyle]: enum.TickMarkStyle.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct CircleTickMarks {
    /// The diameter of a tier 1 tick mark
    pub diameter_tier_1: u16,
    /// The diameter of a tier 2 tick mark
    pub diameter_tier_2: u16,
    /// The diameter of a tier 3 tick mark
    pub diameter_tier_3: u16,

    /// The color of a tier 1 tick mark
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark
    pub color_tier_3: Color,

    /// The distance from the tick mark to the outside edge of the knob
    pub offset: f32,
}

impl std::default::Default for CircleTickMarks {
    fn default() -> Self {
        Self {
            diameter_tier_1: 4,
            diameter_tier_2: 2,
            diameter_tier_3: 2,

            color_tier_1: default_colors::KNOB_TICK_TIER_1,
            color_tier_2: default_colors::KNOB_TICK_TIER_2,
            color_tier_3: default_colors::KNOB_TICK_TIER_3,

            offset: 4.47,
        }
    }
}

/// A line [`TickMarkStyle`] for a [`Knob`]
///
/// [`TickMarkStyle]: enum.TickMarkStyle.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct LineTickMarks {
    /// The width (thickness) of a tier 1 tick mark
    pub width_tier_1: f32,
    /// The width (thickness) of a tier 2 tick mark
    pub width_tier_2: f32,
    /// The width (thickness) of a tier 3 tick mark
    pub width_tier_3: f32,

    /// The length of a tier 1 tick mark
    pub length_tier_1: f32,
    /// The length of a tier 2 tick mark
    pub length_tier_2: f32,
    /// The length of a tier 3 tick mark
    pub length_tier_3: f32,

    /// The color of a tier 1 tick mark
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark
    pub color_tier_3: Color,

    /// The distance from the tick mark to the outside edge of the knob
    pub offset: f32,
}

impl std::default::Default for LineTickMarks {
    fn default() -> Self {
        Self {
            width_tier_1: 2.0,
            width_tier_2: 1.75,
            width_tier_3: 1.75,

            length_tier_1: 3.5,
            length_tier_2: 2.5,
            length_tier_3: 2.5,

            color_tier_1: default_colors::KNOB_TICK_TIER_1,
            color_tier_2: default_colors::KNOB_TICK_TIER_2,
            color_tier_3: default_colors::KNOB_TICK_TIER_3,

            offset: 2.0,
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

    /// The style of a [`TickMarkGroup`] for a [`Knob`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        None
    }
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::VectorCircle(VectorCircleStyle {
            knob_color: default_colors::LIGHT_BACK,
            knob_border_width: 1,
            knob_border_color: default_colors::BORDER,
            notch_color: default_colors::BORDER,
            notch_border_width: 0,
            notch_border_color: Color::TRANSPARENT,
            notch_scale: 0.17.into(),
            notch_offset: 0.15.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::VectorCircle(active) = self.active() {
            Style::VectorCircle(VectorCircleStyle {
                knob_color: default_colors::KNOB_BACK_HOVER,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> Style {
        self.hovered()
    }

    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        Some(TickMarkStyle::default())
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
