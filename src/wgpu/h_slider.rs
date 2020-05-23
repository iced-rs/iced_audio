//! wgpu renderer for the [`HSlider`] widget
//!
//! [`HSlider`]: ../native/h_slider/struct.HSlider.html

use crate::core::Normal;
use crate::native::h_slider;
use iced_native::{
    Background, Color, MouseCursor, Point, Rectangle
};
use iced_wgpu::{Primitive, Renderer};


pub use crate::native::h_slider::State;
pub use crate::style::h_slider::{Style, StyleSheet, ClassicStyle, ClassicHandle,
    RectStyle, RectBipolarStyle, TextureStyle
};

/// This is an alias of a `crate::native` [`HSlider`] with an
/// `iced_wgpu::Renderer`.
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
pub type HSlider<'a, Message, ID> =
    h_slider::HSlider<'a, Message, Renderer, ID>;


impl h_slider::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

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

        let rail_y = (bounds_y + (bounds_height / 2.0)).round();

        match style {



            Style::Texture(style) => {

            

            let (rail_top, rail_bottom) = (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: rail_y,
                        width: bounds_width,
                        height: 2.0,
                    },
                    background: Background::Color(style.rail_colors.0),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: rail_y + 2.0,
                        width: bounds_width,
                        height: 2.0,
                    },
                    background: Background::Color(style.rail_colors.1),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            );
            
            let handle_width = style.handle_width as f32;

            let handle_offset = ( (bounds_width - handle_width)
                                    * normal.value() ).round();

            let handle = {
                if let Some(pad) = style.texture_padding {
                    Primitive::Image {
                        handle: style.texture,
                        bounds: Rectangle {
                            x: bounds.x + handle_offset - pad.left as f32,
                            y: (rail_y - (bounds_height / 2.0)).round()
                                - pad.top as f32,
                            width: handle_width +
                                (pad.left + pad.right) as f32,
                            height: bounds_height +
                                (pad.top + pad.bottom) as f32,
                        },
                    }
                } else {
                    Primitive::Image {
                        handle: style.texture,
                        bounds: Rectangle {
                            x: bounds.x + handle_offset,
                            y: (rail_y - (bounds_height / 2.0)).round(),
                            width: handle_width,
                            height: bounds_height,
                        },
                    }
                }
            };

            (
                Primitive::Group {
                    primitives: vec![rail_top, rail_bottom, handle],
                },
                MouseCursor::default(),
            )
        }



            Style::Classic(style) => {


            
            let (rail_top, rail_bottom) = (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: rail_y,
                        width: bounds_width,
                        height: 2.0,
                    },
                    background: Background::Color(style.rail_colors.0),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: rail_y + 2.0,
                        width: bounds_width,
                        height: 2.0,
                    },
                    background: Background::Color(style.rail_colors.1),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            );

            let (handle_width, handle_border_radius) =
                (f32::from(style.handle.width),
                style.handle.border_radius);
            
            let handle_offset = ( (bounds_width - handle_width)
                                    * normal.value() ).round();
            
            let notch_width = style.handle.notch_width as f32;
            
            let handle = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x + handle_offset,
                    y: (rail_y - (bounds_height / 2.0)).round(),
                    width: handle_width,
                    height: bounds_height,
                },
                background: Background::Color(style.handle.color),
                border_radius: handle_border_radius,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            };

            if style.handle.notch_width != 0 {
                let handle_notch = Primitive::Quad {
                    bounds: Rectangle {
                        x: (bounds_x + handle_offset + (handle_width / 2.0)
                            - (notch_width / 2.0)).round(),
                        y: (rail_y - (bounds_height / 2.0)).round(),
                        width: notch_width,
                        height: bounds_height,
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
                    MouseCursor::default(),
                )
            } else {
                (
                    Primitive::Group {
                        primitives: vec![rail_top, rail_bottom, handle],
                    },
                    MouseCursor::default(),
                )
            }
        }


        
        Style::Rect(style) => {



            let empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y: bounds_y,
                    width: bounds_width,
                    height: bounds_height,
                },
                background: Background::Color(style.back_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: style.border_color,
            };

            let handle_width = style.handle_width as f32;
            let border_width = style.border_width as f32;
            
            let handle_offset = ( (
                                    (bounds_width - (border_width * 2.0))
                                    - handle_width
                                  ) * normal.value() + border_width
                                ).round();

            let filled_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y: bounds_y,
                    width: handle_offset + border_width
                        - style.handle_filled_gap as f32,
                    height: bounds_height,
                },
                background: Background::Color(style.back_filled_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: Color::TRANSPARENT,
            };
            
            let handle = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x + handle_offset - border_width,
                    y: bounds_y,
                    width: handle_width + (border_width * 2.0),
                    height: bounds_height,
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
                MouseCursor::default(),
            )
        }


        
        Style::RectBipolar(style) => {



            let handle_width = style.handle_width as f32;
            let border_width = style.border_width as f32;

            let left_empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y: bounds_y,
                    width: bounds_width,
                    height: bounds_height,
                },
                background: Background::Color(style.back_left_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: style.border_color,
            };

            let half_bounds_width = (bounds_width / 2.0).round();

            let right_empty_rect = Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x + half_bounds_width - border_width,
                    y: bounds_y,
                    width: half_bounds_width + border_width,
                    height: bounds_height,
                },
                background: Background::Color(style.back_right_empty_color),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: Color::TRANSPARENT,
            };
            
            let handle_offset = ( (
                                    ((bounds_width - (border_width * 2.0))
                                    - handle_width
                                  ) * normal.value()) + border_width
                                ).round();
            
            if normal.value() > 0.499 && normal.value() < 0.501 {
                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + handle_offset - border_width,
                        y: bounds_y,
                        width: handle_width + (border_width * 2.0),
                        height: bounds_height,
                    },
                    background: Background::Color(style.handle_center_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![left_empty_rect, right_empty_rect,
                            handle]
                    },
                    MouseCursor::default(),
                )
            } else if normal.value() < 0.5 {
                let filled_rect_offset = handle_offset
                            + handle_width
                            + style.handle_filled_gap as f32
                            - border_width;
                
                let filled_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + filled_rect_offset,
                        y: bounds_y,
                        width: half_bounds_width - filled_rect_offset
                            + border_width,
                        height: bounds_height,
                    },
                    background: Background::Color(style.back_left_filled_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + handle_offset - border_width,
                        y: bounds_y,
                        width: handle_width + (border_width * 2.0),
                        height: bounds_height,
                    },
                    background: Background::Color(style.handle_left_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![left_empty_rect, right_empty_rect,
                            filled_rect, handle]
                    },
                    MouseCursor::default(),
                )
            } else {
                let filled_rect_offset = half_bounds_width;
                let filled_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + filled_rect_offset - border_width,
                        y: bounds_y,
                        width: handle_offset - filled_rect_offset
                                + (border_width * 2.0)
                                - style.handle_filled_gap as f32,
                        height: bounds_height,
                    },
                    background: Background::Color(
                        style.back_right_filled_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + handle_offset - border_width,
                        y: bounds_y,
                        width: handle_width + (border_width * 2.0),
                        height: bounds_height,
                    },
                    background: Background::Color(style.handle_right_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![left_empty_rect, right_empty_rect,
                            filled_rect, handle]
                    },
                    MouseCursor::default(),
                )
            }
            }
        }
    }
}
