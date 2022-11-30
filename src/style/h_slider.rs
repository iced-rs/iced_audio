//! Various styles for the [`HSlider`] widget
//!
//! [`HSlider`]: ../native/h_slider/struct.HSlider.html

use iced_native::{image, Color, Rectangle};

use crate::style::{default_colors, text_marks, tick_marks};

/// The appearance of an [`HSlider`].
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone)]
pub enum Appearance {
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
    /// Colors of the top and bottom of the rail
    pub rail_colors: (Color, Color),
    /// Width (thickness) of the top and bottom of the rail
    pub rail_widths: (f32, f32),
    /// The padding from the rail to the left and right edges of the widget
    pub rail_padding: f32,
}

/// An [`Appearance`] for an [`HSlider`] that uses an image texture for the handle
///
/// [`Appearance`]: enum.Appearance.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`Handle`]: https://docs.rs/iced/latest/iced/pure/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    /// The rail style
    pub rail: ClassicRail,
    /// The [`Handle`] to the image texture
    ///
    /// [`Handle`]: https://docs.rs/iced/latest/iced/pure/widget/image/struct.Handle.html
    pub image_handle: image::Handle,
    /// The effective width of the handle (not including any padding on the texture)
    pub handle_width: u16,
    /// The bounds of the image texture, where the origin is in the
    /// center of the handle.
    pub image_bounds: Rectangle,
}

/// A classic [`Appearance`] for an [`HSlider`], modeled after hardware sliders
///
/// [`Appearance`]: enum.Appearance.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`ClassicHandle`]: struct.ClassicHandle.html
#[derive(Debug, Clone)]
pub struct ClassicStyle {
    /// The rail style
    pub rail: ClassicRail,
    /// a `ClassicHandle` defining the style of the handle
    pub handle: ClassicHandle,
}

impl Default for ClassicStyle {
    fn default() -> Self {
        ClassicStyle {
            rail: ClassicRail {
                rail_colors: default_colors::SLIDER_RAIL,
                rail_widths: (1.0, 1.0),
                rail_padding: 12.0,
            },
            handle: ClassicHandle::default(),
        }
    }
}

/// The [`ClassicStyle`] appearance of the handle of an [`HSlider`]
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
/// [`ClassicStyle`]: struct.ClassicStyle.html
#[derive(Debug, Clone)]
pub struct ClassicHandle {
    /// background color
    pub color: Color,
    /// width of the handle
    pub width: u16,
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

impl Default for ClassicHandle {
    fn default() -> Self {
        ClassicHandle {
            color: default_colors::LIGHT_BACK,
            width: 34,
            notch_width: 4.0,
            notch_color: default_colors::BORDER,
            border_radius: 2.0,
            border_color: default_colors::BORDER,
            border_width: 1.0,
        }
    }
}

/// A modern [`Appearance`] for an [`HSlider`]. It is composed of a background
/// rectangle and a rectangular handle.
///
/// [`Appearance`]: enum.Appearance.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
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
    /// width of the handle rectangle
    pub handle_width: u16,
    /// width of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: f32,
}

/// A modern [`Appearance`] for an [`HSlider`]. It is composed of a background
/// rectangle and a rectangular handle. It has different colors for left, right,
/// and center values.
///
/// [`Appearance`]: enum.Appearance.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
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
    /// rectangle on the left side of the center
    pub left_filled_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the right side of the center
    pub right_filled_color: Color,
    /// color of the handle rectangle when it is on the
    /// left side of the center
    pub handle_left_color: Color,
    /// color of the handle rectangle when it is on the
    /// right side of the center
    pub handle_right_color: Color,
    /// color of the handle rectangle when it is in the center
    pub handle_center_color: Color,
    /// width of the handle rectangle
    pub handle_width: u16,
    /// width of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: f32,
}

/// The position of a [`ModRangeStyle`] ring for an [`HSlider`]
///
/// [`ModRangeStyle`]: struct.ModRangeStyle.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone)]
pub enum ModRangePlacement {
    /// In the center of the widget
    Center {
        /// The height of the mod range.
        height: f32,
        /// The offset from the center of the widget.
        offset: f32,
    },
    /// In the center of the widget while filling the height
    /// of the widget.
    CenterFilled {
        /// The padding from the top and bottom edges of the widget.
        edge_padding: f32,
    },
    /// Above the widget
    Top {
        /// The height of the mod range.
        height: f32,
        /// The offset from the top of the widget.
        offset: f32,
    },
    /// Below the widget
    Bottom {
        /// The height of the mod range.
        height: f32,
        /// The offset from the bottom of the widget.
        offset: f32,
    },
}

/// A style for a [`ModulationRange`] line for an [`HSlider`]
///
/// [`ModulationRange`]: ../../core/struct.ModulationRange.html
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
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

/// Style of tick marks for an [`HSlider`].
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone)]
pub struct TickMarksStyle {
    /// The style of the tick marks
    pub style: tick_marks::Style,
    /// The placement of the tick marks
    pub placement: tick_marks::Placement,
}

/// Style of text marks for an [`HSlider`].
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
#[derive(Debug, Clone)]
pub struct TextMarksStyle {
    /// The style of the text marks
    pub style: text_marks::Style,
    /// The placement of the text marks
    pub placement: text_marks::Placement,
}

/// A set of rules that dictate the style of an [`HSlider`].
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active [`HSlider`].
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a hovered [`HSlider`].
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of an [`HSlider`] that is being dragged.
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn dragging(&self, style: &Self::Style) -> Appearance;

    /// The style of tick marks for an [`HSlider`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn tick_marks_style(&self, _style: &Self::Style) -> Option<TickMarksStyle> {
        None
    }

    /// The style of an [`ModulationRange`] line for an [`HSlider`]
    ///
    /// For no modulation range line, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn mod_range_style(&self, _style: &Self::Style) -> Option<ModRangeStyle> {
        None
    }

    /// The style of a second [`ModulationRange`] line for an [`HSlider`]
    ///
    /// For no second modulation range line, don't override this or set this to return `None`.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn mod_range_style_2(&self, _style: &Self::Style) -> Option<ModRangeStyle> {
        None
    }

    /// The style of text marks for an [`HSlider`]
    ///
    /// For no text marks, don't override this or set this to return `None`.
    ///
    /// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
    fn text_marks_style(&self, _style: &Self::Style) -> Option<TextMarksStyle> {
        None
    }
}
