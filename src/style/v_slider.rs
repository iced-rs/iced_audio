//! Various styles for the [`VSlider`] widget
//!
//! [`VSlider`]: ../../native/v_slider/struct.VSlider.html

use iced::Color;
use iced_native::image;

use crate::style::{bar_text_marks, default_colors};
use crate::TexturePadding;

/// The appearance of an [`VSlider`].
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
    /// same as `Rect` but can have different colors for bottom,
    /// top, and center positions
    RectBipolar(RectBipolarStyle),
}

/// A [`Style`] for an [`VSlider`] that uses an image texture for the handle
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
/// [`Handle`]: https://docs.rs/iced/0.1.1/iced/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    /// colors of the left and right of the rail
    pub rail_colors: (Color, Color),
    /// width (thickness) of the left and right of the rail
    pub rail_widths: (u16, u16),
    /// the [`Handle`] to the image texture
    pub texture: image::Handle,
    /// the height of the handle, not including padding
    pub handle_height: u16,
    /// the texture padding around the handle bounding
    /// rectangle. This is useful when the texture is of a glowing handle or has
    /// a drop shadow, etc.
    pub texture_padding: Option<TexturePadding>,
}

/// A classic [`Style`] for an [`VSlider`], modeled after hardware sliders
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
/// [`ClassicHandle`]: struct.ClassicHandle.html
#[derive(Debug, Clone)]
pub struct ClassicStyle {
    /// colors of the left and right of the rail
    pub rail_colors: (Color, Color),
    /// width (thickness) of the left and right of the rail
    pub rail_widths: (u16, u16),
    /// a `ClassicHandle` defining the style of the handle
    pub handle: ClassicHandle,
}

/// The [`ClassicStyle`] appearance of the handle of an [`VSlider`]
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
    pub notch_width: u16,
    /// color of the middle notch
    pub notch_color: Color,
    /// radius of the background rectangle
    pub border_radius: u16,
    /// width of the background rectangle
    pub border_width: u16,
    /// color of the background rectangle border
    pub border_color: Color,
}

/// A modern [`Style`] for an [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle.
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectStyle {
    /// color of the background rectangle
    pub back_color: Color,
    /// width of the background rectangle border
    pub back_border_width: u16,
    /// radius of the background rectangle
    pub back_border_radius: u16,
    /// color of the background rectangle border
    pub back_border_color: Color,
    /// color of a filled portion in the background rectangle
    pub filled_color: Color,
    /// color of the handle rectangle
    pub handle_color: Color,
    /// height of the handle rectangle
    pub handle_height: u16,
    /// width of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: u16,
}

/// A modern [`Style`] for an [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle. It has different colors for bottom, top,
/// and center values.
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectBipolarStyle {
    /// color of the background rectangle
    pub back_color: Color,
    /// width of the background rectangle border
    pub back_border_width: u16,
    /// radius of the background rectangle
    pub back_border_radius: u16,
    /// color of the background rectangle border
    pub back_border_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the bottom side of the center
    pub bottom_filled_color: Color,
    /// color of a filled portion in the background
    /// rectangle on the top side of the center
    pub top_filled_color: Color,
    /// color of the handle rectangle when it is on the
    /// bottom side of the center
    pub handle_bottom_color: Color,
    /// color of the handle rectangle when it is on the
    /// top side of the center
    pub handle_top_color: Color,
    /// color of the handle rectangle when it is in the center
    pub handle_center_color: Color,
    /// height of the handle rectangle
    pub handle_height: u16,
    /// height of the gap between the handle and the filled
    /// portion of the background rectangle
    pub handle_filled_gap: u16,
}

/// The style of a [`TickMarkGroup`] for a [`VSlider`]
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Copy, Clone)]
pub struct TickMarkStyle {
    /// The length of a tier 1 tick mark relative to the length of the `VSlider`
    pub length_scale_tier_1: f32,
    /// The length of a tier 2 tick mark relative to the length of the `VSlider`
    pub length_scale_tier_2: f32,
    /// The length of a tier 3 tick mark relative to the length of the `VSlider`
    pub length_scale_tier_3: f32,

    /// The width (thickness) of a tier 1 tick mark
    pub width_tier_1: u16,
    /// The width (thickness) of a tier 2 tick mark
    pub width_tier_2: u16,
    /// The width (thickness) of a tier 3 tick mark
    pub width_tier_3: u16,

    /// The color of a tier 1 tick mark
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark
    pub color_tier_3: Color,

    /// The vertical distance from the center rail to a tick mark. Setting this
    /// to `0` will cause each tick mark to be a single continous line going
    /// through the the rail, as apposed to a line above and a line below the
    /// rail.
    pub center_offset: u16,
}

impl std::default::Default for TickMarkStyle {
    fn default() -> Self {
        Self {
            length_scale_tier_1: 1.65,
            length_scale_tier_2: 1.55,
            length_scale_tier_3: 1.4,

            width_tier_1: 2,
            width_tier_2: 1,
            width_tier_3: 1,

            color_tier_1: default_colors::TICK_TIER_1,
            color_tier_2: default_colors::TICK_TIER_2,
            color_tier_3: default_colors::TICK_TIER_3,

            center_offset: 0,
        }
    }
}

/// The position of a [`ModRangeStyle`] ring for a [`VSlider`]
///
/// [`ModRangeStyle`]: struct.ModRangeStyle.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Copy, Clone)]
pub enum ModRangePlacement {
    /// In the center of the widget
    Center,
    /// To the left of the widget
    Left,
    /// To the right of the widget
    Right,
}

/// A style for a [`ModulationRange`] ring for a [`VSlider`]
///
/// [`ModulationRange`]: ../../core/struct.ModulationRange.html
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
#[derive(Debug, Copy, Clone)]
pub struct ModRangeStyle {
    /// The width of the line
    pub width: u16,
    /// The offset of the line from the edge of the widget.
    /// If `placement` is `ModRangePlacement::center`, then
    /// this will be the padding from the edge of the widget.
    pub offset: i32,
    /// The placement of the line relative to the widget
    pub placement: ModRangePlacement,
    /// The color of an empty portion of the line.
    /// Set to `None` for no empty portion.
    pub empty_color: Option<Color>,
    /// The color of a filled portion of the line.
    pub filled_color: Color,
    /// The color of a filled portion of the ring when `end` is less than
    /// `start`.
    pub filled_inverse_color: Color,
}

/// A set of rules that dictate the style of an [`VSlider`].
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

    /// Produces the style of an [`VSlider`] that is being dragged.
    ///
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn dragging(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`VSlider`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
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

    /// The style of a [`TextMarkGroup`] for an [`VSlider`]
    ///
    /// For no text marks, don't override this or set this to return `None`.
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        None
    }
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Classic(ClassicStyle {
            rail_colors: default_colors::SLIDER_RAIL,
            rail_widths: (1, 1),
            handle: ClassicHandle {
                color: default_colors::LIGHT_BACK,
                height: 34,
                notch_width: 4,
                notch_color: default_colors::BORDER,
                border_radius: 2,
                border_color: default_colors::BORDER,
                border_width: 1,
            },
        })
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {
            Style::Classic(ClassicStyle {
                handle: ClassicHandle {
                    color: default_colors::LIGHT_BACK_HOVER,
                    ..active.handle
                },
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {
            Style::Classic(ClassicStyle {
                handle: ClassicHandle {
                    color: default_colors::LIGHT_BACK_DRAG,
                    ..active.handle
                },
                ..active
            })
        } else {
            active
        }
    }

    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        Some(TickMarkStyle::default())
    }

    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style::default())
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
