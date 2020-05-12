use iced::Color;
use iced_native::image;

use crate::TexturePadding;

/// The appearance of an [`VSlider`].
///
/// * `Classic` - modeled after hardware sliders
/// * `Rect` - a modern style with a line inside a filled rectangle
/// * `RectBipolar` - same as `Rect` but can have different colors for bottom,
/// top, and center positions
/// * `Texture` - uses an image texture for the handle
///
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone)]
pub enum Style {
    Classic(ClassicStyle),
    Rect(RectStyle),
    RectBipolar(RectBipolarStyle),
    Texture(TextureStyle),
}

/// A classic [`Style`] for an [`VSlider`], modeled after hardware sliders 
///
/// * `rail_colors` - colors of the top and bottom of the rail
/// * `handle` - a [`ClassicHandle`] defining the style of the handle
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: struct.VSlider.html
/// [`ClassicHandle`]: struct.ClassicHandle.html
#[derive(Debug, Clone)]
pub struct ClassicStyle {
    pub rail_colors: (Color, Color),
    pub handle: ClassicHandle,
}

/// The [`ClassicStyle`] appearance of the handle of an [`VSlider`]
///
/// * `color` - background color
/// * `width` - width of the handle
/// * `height` - height of the handle
/// * `notch_width` - width of the middle notch
/// * `notch_height` - height of the middle notch
/// * `notch_color` - color of the middle notch
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle
/// * `border_color` - color of the background rectangle border
///
/// [`VSlider`]: struct.VSlider.html
/// [`ClassicStyle`]: struct.ClassicStyle.html
#[derive(Debug, Clone, Copy)]
pub struct ClassicHandle {
    pub color: Color,
    pub width: u16,
    pub height: u16,
    pub notch_width: u16,
    pub notch_height: u16,
    pub notch_color: Color,
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: Color,
}

/// A modern [`Style`] for an [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle.
///
/// * `back_empty_color` - color of an unfilled portion in the background
/// rectangle
/// * `back_filled_color` - color of a filled portion in the background
/// rectangle
/// * `border_color` - color of the background rectangle border
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle border
/// * `handle_color` - color of the handle rectangle
/// * `handle_height` - height of the handle rectangle
/// * `handle_filled_gap` - width of the gap between the handle and the filled
/// portion of the background rectangle
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectStyle {
    pub back_empty_color: Color,
    pub back_filled_color: Color,
    pub border_color: Color,
    pub border_radius: u16,
    pub border_width: u16,
    pub handle_color: Color,
    pub handle_height: u16,
    pub handle_filled_gap: u16,
}

/// A modern [`Style`] for an [`VSlider`]. It is composed of a background
/// rectangle and a rectangular handle. It has different colors for bottom, top,
/// and center values.
///
/// * `back_bottom_empty_color` - color of an unfilled portion in the background
/// rectangle on the bottom side of the center
/// * `back_bottom_filled_color` - color of a filled portion in the background
/// rectangle on the bottom side of the center
/// * `back_top_empty_color` - color of an unfilled portion in the background
/// rectangle on the top side of the center
/// * `back_top_filled_color` - color of a filled portion in the background
/// rectangle on the top side of the center
/// * `border_color` - color of the background rectangle border
/// * `border_radius` - radius of the background rectangle
/// * `border_width` - width of the background rectangle border
/// * `handle_bottom_color` - color of the handle rectangle when it is on the
/// bottom side of the center
/// * `handle_top_color` - color of the handle rectangle when it is on the
/// top side of the center
/// * `handle_center_color` - color of the handle rectangle when it is in
/// the center
/// * `handle_height` - height of the handle rectangle
/// * `handle_filled_gap` - height of the gap between the handle and the filled
/// portion of the background rectangle
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone, Copy)]
pub struct RectBipolarStyle {
    pub back_bottom_empty_color: Color,
    pub back_bottom_filled_color: Color,
    pub back_top_empty_color: Color,
    pub back_top_filled_color: Color,
    pub border_color: Color,
    pub border_radius: u16,
    pub border_width: u16,
    pub handle_bottom_color: Color,
    pub handle_top_color: Color,
    pub handle_center_color: Color,
    pub handle_height: u16,
    pub handle_filled_gap: u16,
}

/// A [`Style`] for an [`VSlider`] that uses an image texture for the handle
///
/// * `rail_colors` - colors of the top and bottom of the rail
/// * `texture` - the [`Handle`] to the image texture
/// * `handle_width` - the width of the handle, not including padding
/// * `handle_height` - the height of the handle, not including padding
/// * `texture_padding` - the texture padding around the handle bounding
/// rectangle. This is useful when the texture is of a glowing handle or has
/// a drop shadow, etc.
///
/// [`Style`]: enum.Style.html
/// [`VSlider`]: struct.VSlider.html
/// [`Handle`]: https://docs.rs/iced/0.1.1/iced/widget/image/struct.Handle.html
#[derive(Debug, Clone)]
pub struct TextureStyle {
    pub rail_colors: (Color, Color),
    pub texture: image::Handle,
    pub handle_width: u16,
    pub handle_height: u16,
    pub texture_padding: Option<TexturePadding>,
}

/// A set of rules that dictate the style of an [`VSlider`].
///
/// [`VSlider`]: struct.VSlider.html
pub trait StyleSheet {
    /// Produces the style of an active [`VSlider`].
    ///
    /// [`VSlider`]: struct.VSlider.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`VSlider`].
    ///
    /// [`VSlider`]: struct.VSlider.html
    fn hovered(&self) -> Style;

    /// Produces the style of an [`VSlider`] that is being dragged.
    ///
    /// [`VSlider`]: struct.VSlider.html
    fn dragging(&self) -> Style;

    /// The width of the active selection area / background rectangle.
    /// With [`ClassicStyle`], this is usually the same as `handle.width`
    ///
    /// [`ClassicStyle`]: struct.ClassicStyle.html
    fn width(&self) -> u16;
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Classic(
        ClassicStyle {
            rail_colors: ([0.56, 0.56, 0.56, 0.75].into(), Color::WHITE),
            handle: ClassicHandle {
                color: Color::from_rgb(0.97, 0.97, 0.97),
                width: 16,
                height: 33,
                notch_width: 16,
                notch_height: 4,
                notch_color: Color::from_rgb(0.475, 0.475, 0.475),
                border_radius: 2,
                border_color: Color::from_rgb(0.51, 0.51, 0.51),
                border_width: 1,
            },
        }
        )
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {

        Style::Classic(
        ClassicStyle {
            handle: ClassicHandle {
                color: Color::from_rgb(0.93, 0.93, 0.93),
                ..active.handle
            },
            ..active
        })

        } else { active }
    }

    fn dragging(&self) -> Style {
        let active = self.active();
        if let Style::Classic(active) = self.active() {

        Style::Classic(
        ClassicStyle {
            handle: ClassicHandle {
                color: Color::from_rgb(0.92, 0.92, 0.92),
                ..active.handle
            },
            ..active
        })

        } else { active }
    }

    fn width(&self) -> u16 {
        16
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