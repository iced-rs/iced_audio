//! Various styles for the [`VSlider`] widget
//!
//! [`VSlider`]: ../native/v_slider/struct.VSlider.html

use iced_native::{image, Color, Rectangle};

use crate::core::Offset;
use crate::style::{default_colors, text_marks, tick_marks};

/// The appearance of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum Style {
    /// uses an image texture for the handle
    Texture(TextureStyle),
    /// modeled after hardware sliders
    Classic(ClassicStyle),
    /// a modern style with a line inside a filled rectangle
    Rect(RectStyle),
    /// same as `Rect` but can have different colors for left,
    /// right, and center positions
    RectBipolar(RectBipolarStyle),
}

/// A classic line rail style
#[derive(Debug, Clone)]
pub struct ClassicRail {
    /// Colors of the left and right of the rail
    pub rail_colors: (Color, Color),
    /// Width (thickness) of the left and right of the rail
    pub rail_widths: (f32, f32),
    /// The padding from the rail to the top and bottom edges of the widget
    pub rail_padding: f32,
}

/// A [`Style`] for a [`VSlider`] that uses an image texture for the handle
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
/// [`Handle`]: https://docs.rs/iced/0.1.1/iced/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    /// The rail style
    pub rail: ClassicRail,
    /// The [`Handle`] to the image texture
    pub image_handle: image::Handle,
    /// The effective height of the handle (not including any padding on the texture)
    pub handle_height: u16,
    /// The bounds of the image texture, where the origin is in the
    /// center of the handle.
    pub image_bounds: Rectangle,
}

/// A classic [`Style`] for a [`VSlider`], modeled after hardware sliders
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
/// [`ClassicHandle`]: struct.ClassicHandle.html
#[derive(Debug, Clone)]
pub struct ClassicStyle {
    /// The rail style
    pub rail: ClassicRail,
    /// a `ClassicHandle` defining the style of the handle
    pub handle: ClassicHandle,
}

/// The [`ClassicStyle`] appearance of the handle of a [`VSlider`]
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
/// [`ClassicStyle`]: struct.ClassicStyle.html
#[derive(Debug, Clone)]
pub struct ClassicHandle {
    /// background color
    pub color: Color,
    /// height of the handle
    pub height: u16,
    /// the width (thickness) of the middle notch
    pub notch_width: f32,
    /// color of the middle notch
    pub notch_color: Color,
    /// radius of the background rectangle
    pub border_radius: f32,
    /// width of the background rectangle
    pub border_width: f32,
    /// color of the background rectangle border
    pub border_color: Color,
}

/// A modern [`Style`] for a [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle.
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectStyle {
    /// color of the background rectangle
    pub back_color: Color,
    /// width of the background rectangle border
    pub back_border_width: f32,
    /// radius of the background rectangle
    pub back_border_radius: f32,
    /// color of the background rectangle border
    pub back_border_color: Color,
    /// color of a filled portion in the background rectangle
    pub filled_color: Color,
    /// color of the handle rectangle
    pub handle_color: Color,
    /// height of the handle rectangle
    pub handle_height: u16,
    /// height of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: f32,
}

/// A modern [`Style`] for a [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle. It has different colors for left, right,
/// and center values.
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectBipolarStyle {
    /// color of the background rectangle
    pub back_color: Color,
    /// width of the background rectangle border
    pub back_border_width: f32,
    /// radius of the background rectangle
    pub back_border_radius: f32,
    /// color of the background rectangle border
    pub back_border_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the top side of the center
    pub top_filled_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the bottom side of the center
    pub bottom_filled_color: Color,
    /// color of the handle rectangle when it is on the
    /// top side of the center
    pub handle_top_color: Color,
    /// color of the handle rectangle when it is on the
    /// bottom side of the center
    pub handle_bottom_color: Color,
    /// color of the handle rectangle when it is in the center
    pub handle_center_color: Color,
    /// height of the handle rectangle
    pub handle_height: u16,
    /// height of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: f32,
}

/// The position of a [`ModRangeStyle`] ring for a [`VSlider`]
///
/// [`ModRangeStyle`]: struct.ModRangeStyle.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub enum ModRangePlacement {
    /// In the center of the widget
    Center {
        /// The width of the mod range.
        width: f32,
        /// The offset from the center of the widget.
        offset: f32,
    },
    /// In the center of the widget while filling the width
    /// of the widget.
    CenterFilled {
        /// The padding from the left and right edges of the widget.
        edge_padding: f32,
    },
    /// To the left of the widget
    Left {
        /// The width of the mod range.
        width: f32,
        /// The offset from the left edge of the widget.
        offset: f32,
    },
    /// To the right of the widget
    Right {
        /// The width of the mod range.
        width: f32,
        /// The offset from the right edge of the widget.
        offset: f32,
    },
}

/// A style for a [`ModulationRange`] line for a [`VSlider`]
///
/// [`ModulationRange`]: ../../core/struct.ModulationRange.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct ModRangeStyle {
    /// The placement of the line relative to the widget
    pub placement: ModRangePlacement,
    /// The width of the background border.
    pub back_border_width: f32,
    /// The radius of the background border.
    pub back_border_radius: f32,
    /// The color of the background border.
    pub back_border_color: Color,
    /// The color of the background.
    /// Set to `None` for no background.
    pub back_color: Option<Color>,
    /// The color of a filled portion of the line.
    pub filled_color: Color,
    /// The color of a filled portion of the line when `end` is less than
    /// `start`.
    pub filled_inverse_color: Color,
}

/// Style of tick marks for a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct TickMarksStyle {
    /// The style of the tick marks
    pub style: tick_marks::Style,
    /// The placement of the tick marks
    pub placement: tick_marks::Placement,
}

/// Style of text marks for a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone)]
pub struct TextMarksStyle {
    /// The style of the text marks
    pub style: text_marks::Style,
    /// The placement of the text marks
    pub placement: text_marks::Placement,
}

/// A set of rules that dictate the style of a [`VSlider`].
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
pub trait StyleSheet {
    /// Produces the style of an active [`VSlider`].
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`VSlider`].
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn hovered(&self) -> Style;

    /// Produces the style of a [`VSlider`] that is being dragged.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn dragging(&self) -> Style;

    /// The style of tick marks for a [`VSlider`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        None
    }

    /// The style of an [`ModulationRange`] line for a [`VSlider`]
    ///
    /// For no modulation range line, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn mod_range_style(&self) -> Option<ModRangeStyle> {
        None
    }

    /// The style of a second [`ModulationRange`] line for a [`VSlider`]
    ///
    /// For no second modulation range line, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn mod_range_style_2(&self) -> Option<ModRangeStyle> {
        None
    }

    /// The style of text marks for a [`VSlider`]
    ///
    /// For no text marks, don't override this or set this to return `None`.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn text_marks_style(&self) -> Option<TextMarksStyle> {
        None
    }
}

struct Default;
impl Default {
    const ACTIVE_STYLE: ClassicStyle = ClassicStyle {
        rail: ClassicRail {
            rail_colors: default_colors::SLIDER_RAIL,
            rail_widths: (1.0, 1.0),
            rail_padding: 12.0,
        },
        handle: ClassicHandle {
            color: default_colors::LIGHT_BACK,
            height: 34,
            notch_width: 4.0,
            notch_color: default_colors::BORDER,
            border_radius: 2.0,
            border_color: default_colors::BORDER,
            border_width: 1.0,
        },
    };
}
impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Classic(Self::ACTIVE_STYLE)
    }

    fn hovered(&self) -> Style {
        Style::Classic(ClassicStyle {
            handle: ClassicHandle {
                color: default_colors::LIGHT_BACK_HOVER,
                ..Self::ACTIVE_STYLE.handle
            },
            ..Self::ACTIVE_STYLE
        })
    }

    fn dragging(&self) -> Style {
        Style::Classic(ClassicStyle {
            handle: ClassicHandle {
                color: default_colors::LIGHT_BACK_DRAG,
                ..Self::ACTIVE_STYLE.handle
            },
            ..Self::ACTIVE_STYLE
        })
    }

    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: tick_marks::Style {
                tier_1: tick_marks::Shape::Line {
                    length: 24.0,
                    width: 2.0,
                    color: default_colors::TICK_TIER_1,
                },
                tier_2: tick_marks::Shape::Line {
                    length: 22.0,
                    width: 1.0,
                    color: default_colors::TICK_TIER_2,
                },
                tier_3: tick_marks::Shape::Line {
                    length: 18.0,
                    width: 1.0,
                    color: default_colors::TICK_TIER_3,
                },
            },
            placement: tick_marks::Placement::Center {
                offset: Offset::ZERO,
                fill_length: false,
            },
        })
    }

    fn text_marks_style(&self) -> Option<TextMarksStyle> {
        Some(TextMarksStyle {
            style: text_marks::Style::default(),
            placement: text_marks::Placement::LeftOrTop {
                inside: false,
                offset: Offset { x: -7.0, y: 0.0 },
            },
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
