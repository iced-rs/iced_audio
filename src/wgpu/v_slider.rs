use crate::core::Normal;
use crate::native::v_slider;
use iced_native::{
    Background, Color, MouseCursor, Point, Rectangle
};
use iced_wgpu::{Primitive, Renderer};


pub use crate::native::v_slider::State;
pub use crate::style::v_slider::{Style, StyleSheet, ClassicStyle, ClassicHandle,
    RectStyle, RectBipolarStyle, TextureStyle
};

/// This is an alias of a `crate::native` HSlider with an `iced_wgpu::Renderer`.
pub type VSlider<'a, Message, ID> =
    v_slider::VSlider<'a, Message, Renderer, ID>;


impl v_slider::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn width(&self, style_sheet: &Self::Style) -> u16 {
        style_sheet.width()
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

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let rail_x = (bounds_x + (bounds_width / 2.0)).round();

        match style {



            Style::Classic(style) => {


            
            let (rail_top, rail_bottom) = (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: rail_x,
                        y: bounds_y,
                        width: 2.0,
                        height: bounds_height,
                    },
                    background: Background::Color(style.rail_colors.0),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: rail_x + 2.0,
                        y: bounds_y,
                        width: 2.0,
                        height: bounds_height,
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
            
            let handle_offset = ( (bounds_height - handle_height)
                                    * (1.0 - normal.value())
                                ).round();
            
            let notch_width = style.handle.notch_width as f32;
            let notch_height = style.handle.notch_height as f32;
            
            let handle = Primitive::Quad {
                bounds: Rectangle {
                    x: (rail_x - (handle_width / 2.0) + 1.0).round(),
                    y: bounds_y + handle_offset,
                    width: handle_width,
                    height: handle_height,
                },
                background: Background::Color(style.handle.color),
                border_radius: handle_border_radius,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            };

            if style.handle.notch_width != 0 {
                let handle_notch = Primitive::Quad {
                    bounds: Rectangle {
                        x: (rail_x - (notch_width / 2.0) + 1.0).round(),
                        y: (bounds_y + handle_offset + (handle_height / 2.0)
                            - (notch_height / 2.0)).round(),
                        width: notch_width,
                        height: notch_height,
                    },
                    background: Background::Color(style.handle.notch_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![rail_top, rail_bottom, handle,
                            handle_notch],
                    },
                    MouseCursor::OutOfBounds,
                )
            } else {
                (
                    Primitive::Group {
                        primitives: vec![rail_top, rail_bottom, handle],
                    },
                    MouseCursor::OutOfBounds,
                )
            }
        }


        
        Style::Rect(style) => {
            
            let rect_width = style_sheet.width() as f32;
            let rect_x = rail_x - (rect_width / 2.0).round();

            let empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: rect_x,
                    y: bounds_y,
                    width: rect_width,
                    height: bounds_height,
                },
                background: Background::Color(style.back_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: style.border_color,
            };

            let handle_height = style.handle_height as f32;
            let border_width = style.border_width as f32;
            
            let handle_offset = ( (
                                    ( (bounds_height - (border_width * 2.0))
                                    - handle_height
                                  ) * (1.0 - normal.value()) )
                                ).round();
            
            let filled_rect_offset = handle_offset + handle_height
                + style.handle_filled_gap as f32;

            let filled_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: rect_x,
                    y: bounds_y + filled_rect_offset,
                    width: rect_width,
                    height: bounds_height - filled_rect_offset + border_width,
                },
                background: Background::Color(style.back_filled_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: Color::TRANSPARENT,
            };
            
            let handle = Primitive::Quad {
                bounds: Rectangle {
                    x: rect_x,
                    y: bounds_y + handle_offset,
                    width: rect_width,
                    height: handle_height + (border_width * 2.0),
                },
                background: Background::Color(style.handle_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: Color::TRANSPARENT,
            };

            (
                Primitive::Group {
                    primitives: vec![empty_rect, filled_rect, handle]
                },
                MouseCursor::OutOfBounds,
            )
        }


        
        Style::RectBipolar(style) => {


            let rect_width = style_sheet.width() as f32;
            let rect_x = rail_x - (rect_width / 2.0).round();

            let handle_height = style.handle_height as f32;
            let border_width = style.border_width as f32;

            let bottom_empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: rect_x,
                    y: bounds_y,
                    width: rect_width,
                    height: bounds_height,
                },
                background: Background::Color(style.back_bottom_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: style.border_color,
            };

            let half_bounds_height = (bounds_height / 2.0).round();

            let top_empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: rect_x,
                    y: bounds_y,
                    width: rect_width,
                    height: half_bounds_height,
                },
                background: Background::Color(style.back_top_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: Color::TRANSPARENT,
            };
            
            let handle_offset = ( (
                                    ( (bounds_height - (border_width * 2.0))
                                    - handle_height
                                  ) * (1.0 - normal.value()) ) + border_width
                                ).round();
            
            if normal.value() > 0.499 && normal.value() < 0.501 {
                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + handle_offset - border_width,
                        width: rect_width,
                        height: handle_height + (border_width * 2.0),
                    },
                    background: Background::Color(style.handle_center_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![bottom_empty_rect, top_empty_rect,
                            handle]
                    },
                    MouseCursor::OutOfBounds,
                )
            } else if normal.value() > 0.5 {
                let filled_rect_offset = handle_offset
                            + handle_height
                            + style.handle_filled_gap as f32
                            - border_width;
                
                let filled_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + filled_rect_offset,
                        width: rect_width,
                        height: half_bounds_height - filled_rect_offset,
                    },
                    background: Background::Color(style.back_top_filled_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + handle_offset - border_width,
                        width: rect_width,
                        height: handle_height + (border_width * 2.0),
                    },
                    background: Background::Color(style.handle_top_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![bottom_empty_rect, top_empty_rect,
                            filled_rect, handle]
                    },
                    MouseCursor::OutOfBounds,
                )
            } else {
                let filled_rect_offset = half_bounds_height;
                let filled_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + filled_rect_offset - (border_width * 2.0),
                        width: rect_width,
                        height: handle_offset - filled_rect_offset
                                + (border_width * 3.0)
                                - style.handle_filled_gap as f32,
                    },
                    background: Background::Color(
                        style.back_bottom_filled_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + handle_offset - border_width,
                        width: rect_width,
                        height: handle_height + (border_width * 2.0),
                    },
                    background: Background::Color(style.handle_bottom_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![bottom_empty_rect, top_empty_rect,
                            filled_rect, handle]
                    },
                    MouseCursor::OutOfBounds,
                )
            }
            }



            Style::Texture(style) => {

            

            let (rail_top, rail_bottom) = (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: rail_x,
                        y: bounds_y,
                        width: 2.0,
                        height: bounds_height,
                    },
                    background: Background::Color(style.rail_colors.0),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: rail_x + 2.0,
                        y: bounds.y,
                        width: 2.0,
                        height: bounds_height,
                    },
                    background: Background::Color(style.rail_colors.1),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            );
            
            let handle_width = style.handle_width as f32;
            let handle_height = style.handle_height as f32;

            let handle_offset = ( (bounds_height - handle_height)
                                    * (1.0 - normal.value()) ).round();

            let handle = {
                if let Some(pad) = style.texture_padding {
                    Primitive::Image {
                        handle: style.texture,
                        bounds: Rectangle {
                            x: (rail_x - (handle_width / 2.0)).round()
                                - pad.bottom as f32,
                            y: bounds.y + handle_offset - pad.top as f32,
                            width: handle_width +
                                (pad.bottom + pad.top) as f32,
                            height: handle_height +
                                (pad.top + pad.bottom) as f32,
                        },
                    }
                } else {
                    Primitive::Image {
                        handle: style.texture,
                        bounds: Rectangle {
                            x: (rail_x - (handle_width / 2.0) + 1.0).round(),
                            y: bounds.y + handle_offset,
                            width: handle_width,
                            height: handle_height,
                        },
                    }
                }
            };

            (
                Primitive::Group {
                    primitives: vec![rail_top, rail_bottom, handle],
                },
                MouseCursor::OutOfBounds,
            )
        }
        }
    }
}
