use iced::{
    advanced::{text::Renderer as _, Text},
    alignment::Vertical,
    widget::text::{Alignment, LineHeight, Shaping, Wrapping},
    Pixels, Point, Rectangle, Renderer, Size,
};

use super::Group;
use crate::style::text_marks::Appearance;
/// Draws text marks around an arc.
///
/// * `center` - The center point of the arc.
/// * `radius` - The radius of the arc where the text marks start
/// * `start_angle` - The starting angle of the arc in radians
/// * `angle_span` - The span of the angle in radians
/// * `text_marks` - The group of text marks.
/// * `style` - The text marks style.
/// * `h_char_offset` - Extra horizontal offset in pixels for each additional
///   character in the text label. This is used to keep longer labels on the sides
///   from being too close to the arc.
/// * `inverse` - Whether to inverse the positions of the text marks (true) or
///   not (false).
#[allow(clippy::too_many_arguments)]
pub fn draw_radial_text_marks(
    renderer: &mut Renderer,
    center: Point,
    radius: f32,
    start_angle: f32,
    angle_span: f32,
    text_marks: &Group,
    style: &Appearance,
    h_char_offset: f32,
    inverse: bool,
    //cache: &PrimitiveCache,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    let start_angle = start_angle + std::f32::consts::FRAC_PI_2;

    for (position, text) in text_marks.group.iter() {
        let angle = if inverse {
            start_angle + position.scale_inv(angle_span)
        } else {
            start_angle + position.scale(angle_span)
        };

        let (dx, dy) = {
            if !(-0.001..=0.001).contains(&angle) {
                angle.sin_cos()
            } else {
                (0.0, -1.0)
            }
        };

        let mut offset_x = dx * radius;
        if offset_x < -0.001 {
            offset_x -= (text.len() as f32 - 1.0) * h_char_offset;
        } else if offset_x > 0.001 {
            offset_x += (text.len() as f32 - 1.0) * h_char_offset;
        }

        renderer.fill_text(
            Text {
                content: text.clone(),
                size: Pixels(text_size),
                bounds: Size {
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                align_x: Alignment::Center,
                align_y: Vertical::Center,
                line_height: LineHeight::default(),
                wrapping: Wrapping::default(),
                shaping: Shaping::Basic,
                font,
            },
            Point {
                x: (center.x + offset_x).round(),
                y: (center.y - (dy * radius)).round(),
            },
            color,
            // TODO: What is this?
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1000.0,
                height: 1000.0,
            },
        );
    }
}
