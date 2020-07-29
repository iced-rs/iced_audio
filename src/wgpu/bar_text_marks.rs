//! `iced_wgpu` renderer for text marks for bar meters

use crate::core::TextMarkGroup;
use crate::style::bar_text_marks::{Placement, Style};

use iced_native::{HorizontalAlignment, Rectangle, VerticalAlignment};
use iced_wgpu::Primitive;

pub fn draw_vertical_text_marks(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    text_marks: &TextMarkGroup,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let offset = style.offset as f32;
    let color = style.color;
    let font = style.font;
    let text_size = style.text_size as f32;
    let text_bounds_width = style.bounds_width as f32;
    let text_bounds_height = style.bounds_height as f32;

    let start_y = bounds_y + bounds_height;

    match style.placement {
        Placement::LeftOrTop => {
            primitives.reserve_exact(text_marks.group.len());

            let start_x = bounds_x - offset;

            for text_mark in text_marks.group.iter() {
                let y = if inverse {
                    (start_y - text_mark.position.scale_inv(bounds_height))
                        .floor()
                } else {
                    (start_y - text_mark.position.scale(bounds_height)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x: start_x,
                        y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Right,
                    vertical_alignment: VerticalAlignment::Center,
                });
            }
        }
        Placement::RightOrBottom => {
            primitives.reserve_exact(text_marks.group.len());

            let start_x = bounds_x + bounds_width + offset;

            for text_mark in text_marks.group.iter() {
                let y = if inverse {
                    (start_y - text_mark.position.scale_inv(bounds_height))
                        .floor()
                } else {
                    (start_y - text_mark.position.scale(bounds_height)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x: start_x,
                        y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Left,
                    vertical_alignment: VerticalAlignment::Center,
                });
            }
        }
        Placement::BothSides => {
            primitives.reserve_exact(text_marks.group.len() * 2);

            let left_start_x = bounds_x - offset;
            let right_start_x = bounds_x + bounds_width + offset;

            for text_mark in text_marks.group.iter() {
                let y = if inverse {
                    (start_y - text_mark.position.scale_inv(bounds_height))
                        .floor()
                } else {
                    (start_y - text_mark.position.scale(bounds_height)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x: left_start_x,
                        y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Right,
                    vertical_alignment: VerticalAlignment::Center,
                });

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x: right_start_x,
                        y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Left,
                    vertical_alignment: VerticalAlignment::Center,
                });
            }
        }
    }

    Primitive::Group { primitives }
}

pub fn draw_horizontal_text_marks(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    text_marks: &TextMarkGroup,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let offset = style.offset as f32;
    let color = style.color;
    let font = style.font;
    let text_size = style.text_size as f32;
    let text_bounds_width = style.bounds_width as f32;
    let text_bounds_height = style.bounds_height as f32;

    let start_x = bounds_x;

    match style.placement {
        Placement::LeftOrTop => {
            primitives.reserve_exact(text_marks.group.len());

            let start_y = bounds_y - offset;

            for text_mark in text_marks.group.iter() {
                let x = if inverse {
                    (start_x + text_mark.position.scale_inv(bounds_width))
                        .floor()
                } else {
                    (start_x + text_mark.position.scale(bounds_width)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x,
                        y: start_y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Bottom,
                });
            }
        }
        Placement::RightOrBottom => {
            primitives.reserve_exact(text_marks.group.len());

            let start_y = bounds_y + bounds_height + offset;

            for text_mark in text_marks.group.iter() {
                let x = if inverse {
                    (start_x + text_mark.position.scale_inv(bounds_width))
                        .floor()
                } else {
                    (start_x + text_mark.position.scale(bounds_width)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x,
                        y: start_y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Top,
                });
            }
        }
        Placement::BothSides => {
            primitives.reserve_exact(text_marks.group.len() * 2);

            let top_start_y = bounds_y - offset;
            let bottom_start_y = bounds_y + bounds_height + offset;

            for text_mark in text_marks.group.iter() {
                let x = if inverse {
                    (start_x + text_mark.position.scale_inv(bounds_width))
                        .floor()
                } else {
                    (start_x + text_mark.position.scale(bounds_width)).floor()
                };

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x,
                        y: top_start_y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Bottom,
                });

                primitives.push(Primitive::Text {
                    content: text_mark.text.clone(),
                    size: text_size,
                    bounds: Rectangle {
                        x,
                        y: bottom_start_y,
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    color,
                    font,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Top,
                });
            }
        }
    }

    Primitive::Group { primitives }
}
