//! wgpu renderer for tick marks for bar meters

use crate::core::TickMarkGroup;
use crate::style::bar_tick_marks::{Placement, Style};
use iced_native::{Background, Color, Rectangle};
use iced_wgpu::Primitive;

pub fn draw_vertical_tick_marks(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: &TickMarkGroup,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let offset = style.offset as f32;

    match style.placement {
        Placement::BothSides => {
            primitives.reserve_exact(tick_marks.len() * 2);

            let left_start_x = bounds_x - offset;
            let right_x = bounds_x + bounds_width + offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);
                let left_x = left_start_x - mark_length;

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: left_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: right_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);
                let left_x = left_start_x - mark_length;

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: left_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: right_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);
                let left_x = left_start_x - mark_length;

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: left_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: right_x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
        Placement::LeftOrTop => {
            primitives.reserve_exact(tick_marks.len());

            let start_x = bounds_x - offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);
                let x = start_x - mark_length;

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);
                let x = start_x - mark_length;

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);
                let x = start_x - mark_length;

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
        Placement::RightOrBottom => {
            primitives.reserve_exact(tick_marks.len());

            let x = bounds_x + bounds_width + offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_y = bounds_y + bounds_height - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let y = if inverse {
                        (start_y - tick_mark_position.scale_inv(bounds_height))
                            .floor()
                    } else {
                        (start_y - tick_mark_position.scale(bounds_height))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_length,
                            height: mark_width,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
    }

    Primitive::Group { primitives }
}

pub fn draw_horizontal_tick_marks(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: &TickMarkGroup,
    style: &Style,
    inverse: bool,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let offset = style.offset as f32;

    match style.placement {
        Placement::BothSides => {
            primitives.reserve_exact(tick_marks.len() * 2);

            let top_start_y = bounds_y - offset;
            let bottom_y = bounds_y + bounds_height + offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);
                let top_y = top_start_y - mark_length;

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: top_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: bottom_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);
                let top_y = top_start_y - mark_length;

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: top_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: bottom_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);
                let top_y = top_start_y - mark_length;

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: top_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: bottom_y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
        Placement::LeftOrTop => {
            primitives.reserve_exact(tick_marks.len());

            let start_y = bounds_y - offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);
                let y = start_y - mark_length;

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);
                let y = start_y - mark_length;

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);
                let y = start_y - mark_length;

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
        Placement::RightOrBottom => {
            primitives.reserve_exact(tick_marks.len() * 2);

            let y = bounds_y + bounds_height + offset;

            if tick_marks.has_tier_1() {
                let mark_width = style.width_tier_1 as f32;
                let mark_length = style.length_tier_1 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_1);

                for tick_mark_position in tick_marks.tier_1_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_2() {
                let mark_width = style.width_tier_2 as f32;
                let mark_length = style.length_tier_2 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_2);

                for tick_mark_position in tick_marks.tier_2_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
            if tick_marks.has_tier_3() {
                let mark_width = style.width_tier_3 as f32;
                let mark_length = style.length_tier_3 as f32;
                let start_x = bounds_x - (mark_width / 2.0);
                let color = Background::Color(style.color_tier_3);

                for tick_mark_position in tick_marks.tier_3_positions().iter() {
                    let x = if inverse {
                        (start_x + tick_mark_position.scale_inv(bounds_width))
                            .floor()
                    } else {
                        (start_x + tick_mark_position.scale(bounds_width))
                            .floor()
                    };

                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: mark_width,
                            height: mark_length,
                        },
                        background: color,
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }
    }

    Primitive::Group { primitives }
}
