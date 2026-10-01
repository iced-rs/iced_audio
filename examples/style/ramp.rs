#![allow(unused)]

use iced::{Background, Border, Color, Theme};
use iced_audio::{ramp, virtual_slider::Status};

use super::colors;

// Custom style for the Ramp widget
pub fn custom_style(theme: &Theme, status: Status) -> ramp::Style {
    let base = ramp::Style {
        background: Some(Background::Color(colors::KNOB)),
        border: Border::default().color(colors::KNOB_BORDER).width(2.0),
        line_width: 2.0,
        line_color: Color::from_rgb(0.7, 0.7, 0.7),
        line_up_color: Some(Color::from_rgb(0.0, 0.9, 0.0)),
        line_down_color: Some(colors::HANDLE),
    };

    match status {
        Status::Hovered => ramp::Style {
            line_color: Color::from_rgb(0.8, 0.8, 0.8),
            line_up_color: Some(Color::from_rgb(0.0, 1.0, 0.0)),
            line_down_color: Some(Color::from_rgb(
                0x8A as f32 / 255.0,
                0xD7 as f32 / 255.0,
                0xFF as f32 / 255.0,
            )),
            ..base
        },
        _ => base,
    }
}
