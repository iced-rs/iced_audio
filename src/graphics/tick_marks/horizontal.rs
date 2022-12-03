//! `iced_graphics` renderer for tick marks

use super::PrimitiveCache;
use crate::core::Normal;
use crate::native::tick_marks;
use crate::style::tick_marks::{Placement, Shape, Style};
use iced_graphics::{Background, Color, Primitive, Rectangle};

#[allow(clippy::too_many_arguments)]
fn draw_horizontal_lines(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_x: f32,
    bounds_width: f32,
    y: f32,
    width: f32,
    length: f32,
    color: Color,
    inverse: bool,
) {
    let start_x = bounds_x - (width / 2.0);
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale_inv(bounds_width)),
                    y,
                    width,
                    height: length,
                },
                background: back_color,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale(bounds_width)),
                    y,
                    width,
                    height: length,
                },
                background: back_color,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_horizontal_circles(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_x: f32,
    bounds_width: f32,
    y: f32,
    diameter: f32,
    color: Color,
    inverse: bool,
) {
    let radius = diameter / 2.0;
    let start_x = bounds_x - radius;
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale_inv(bounds_width)),
                    y,
                    width: diameter,
                    height: diameter,
                },
                background: back_color,
                border_radius: radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: (start_x + tick_mark.scale(bounds_width)),
                    y,
                    width: diameter,
                    height: diameter,
                },
                background: back_color,
                border_radius: radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

#[inline]
fn draw_horizontal_top_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_horizontal_lines(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y,
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_horizontal_circles(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y,
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_horizontal_top_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_horizontal_top_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_horizontal_bottom_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_horizontal_lines(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y - (*length),
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_horizontal_circles(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y - (*diameter),
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_horizontal_bottom_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_horizontal_bottom_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_horizontal_center_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (y, length) = if fill_length {
                    (bounds.y + (*length), bounds.height - ((*length) * 2.0))
                } else {
                    (y - (*length / 2.0), *length)
                };

                draw_horizontal_lines(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (y, diameter) = if fill_length {
                    (
                        bounds.y + (*diameter),
                        bounds.height - ((*diameter) * 2.0),
                    )
                } else {
                    (y - (diameter / 2.0), *diameter)
                };

                draw_horizontal_circles(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    y,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_horizontal_center_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    inverse: bool,
) {
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        inverse,
    );
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        inverse,
    );
    draw_horizontal_center_aligned_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        inverse,
    );
}

#[inline]
#[allow(clippy::too_many_arguments)]
fn draw_horizontal_center_aligned_split_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (left_y, length) = if fill_length {
                    let length = (*length) + (bounds.height + gap) / 2.0;
                    ((y - length - (gap / 2.0)), length)
                } else {
                    ((y - (*length) - (gap / 2.0)), *length)
                };

                let right_y = y + (gap / 2.0);

                draw_horizontal_lines(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    left_y,
                    *width,
                    length,
                    *color,
                    inverse,
                );
                draw_horizontal_lines(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    right_y,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (left_y, diameter) = if fill_length {
                    (
                        bounds.y - (*diameter),
                        (*diameter) + ((bounds.height + gap) / 2.0),
                    )
                } else {
                    (y - (*diameter) - (gap / 2.0), *diameter)
                };

                let right_y = y + (gap / 2.0);

                draw_horizontal_circles(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    left_y,
                    diameter,
                    *color,
                    inverse,
                );
                draw_horizontal_circles(
                    primitives,
                    tick_marks,
                    bounds.x,
                    bounds.width,
                    right_y,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_horizontal_center_aligned_split(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        gap,
        inverse,
    );
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        gap,
        inverse,
    );
    draw_horizontal_center_aligned_split_tier(
        primitives,
        bounds,
        y,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        gap,
        inverse,
    );
}

/// Draws tick marks on a horizontal axis.
///
/// * bounds - The bounds of the widget to place the tick marks in/outside of.
/// * tick_marks - The group of tick marks.
/// * style - The tick marks style.
/// * placement - The placement of the tick marks relative to the bounds.
/// * inverse - Whether to inverse the positions of the tick marks (true) or
/// not (false).
pub fn draw_horizontal_tick_marks(
    bounds: &Rectangle,
    tick_marks: &tick_marks::Group,
    style: &Style,
    placement: &Placement,
    inverse: bool,
    cache: &PrimitiveCache,
) -> Primitive {
    cache.cached_linear(
        *bounds,
        tick_marks,
        *style,
        *placement,
        inverse,
        || {
            let primitives = match placement {
                Placement::BothSides { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len() * 2);

                    if *inside {
                        draw_horizontal_top_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                        draw_horizontal_bottom_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_horizontal_bottom_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                        draw_horizontal_top_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::LeftOrTop { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    if *inside {
                        draw_horizontal_top_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_horizontal_bottom_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::RightOrBottom { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    if *inside {
                        draw_horizontal_bottom_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_horizontal_top_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::Center {
                    offset,
                    fill_length,
                } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    draw_horizontal_center_aligned(
                        &mut primitives,
                        &bounds,
                        bounds.center_y(),
                        tick_marks,
                        style,
                        *fill_length,
                        inverse,
                    );

                    primitives
                }
                Placement::CenterSplit {
                    offset,
                    fill_length,
                    gap,
                } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len() * 2);

                    draw_horizontal_center_aligned_split(
                        &mut primitives,
                        &bounds,
                        bounds.center_y(),
                        tick_marks,
                        style,
                        *fill_length,
                        *gap,
                        inverse,
                    );

                    primitives
                }
            };

            Primitive::Group { primitives }
        },
    )
}
