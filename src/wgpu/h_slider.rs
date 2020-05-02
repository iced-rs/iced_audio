use crate::{
    Normal,
    native::h_slider,
    style::h_slider::StyleSheet,
};
use iced_native::{
    Background, Color, MouseCursor, Point, Rectangle
};
use iced_wgpu::{Primitive, Renderer};

impl h_slider::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn height(&self, style_sheet: &Self::Style) -> u16 {
        style_sheet.height()
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let rail_y = bounds.y + (bounds.height / 2.0).round();

        let (rail_top, rail_bottom) = (
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y,
                    width: bounds.width,
                    height: 2.0,
                },
                background: Background::Color(style.rail_colors.0),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y + 2.0,
                    width: bounds.width,
                    height: 2.0,
                },
                background: Background::Color(style.rail_colors.1),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        );

        let (handle_width, handle_height, handle_border_radius) =
            (f32::from(style.handle.width),
            f32::from(style.handle.height),
            style.handle.border_radius);
        
        let handle_offset = (bounds.width - handle_width) * normal.value();
        
        let handle = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: rail_y - handle_height / 2.0,
                width: handle_width,
                height: handle_height,
            },
            background: Background::Color(style.handle.color),
            border_radius: handle_border_radius,
            border_width: style.handle.border_width,
            border_color: style.handle.border_color,
        };

        let handle_notch = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset + (handle_width / 2.0)
                    - 1.0,
                y: rail_y - handle_height / 2.0,
                width: 2.0,
                height: handle_height,
            },
            background: Background::Color(style.handle.border_color),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        (
            Primitive::Group {
                primitives: vec![rail_top, rail_bottom, handle, handle_notch],
            },
            MouseCursor::OutOfBounds,
        )
    }
}
