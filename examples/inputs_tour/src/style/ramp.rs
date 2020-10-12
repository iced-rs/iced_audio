use iced::Color;
use iced_audio::ramp;

use super::colors;

// Custom style for the Ramp widget

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_STYLE: ramp::Style = ramp::Style {
        back_color: colors::KNOB,
        back_border_width: 2,
        back_border_color: colors::KNOB_BORDER,
        line_width: 2.0,
        line_center_color: Color::from_rgb(0.7, 0.7, 0.7),
        line_up_color: Color::from_rgb(0.0, 0.9, 0.0),
        line_down_color: colors::HANDLE,
    };
}
impl ramp::StyleSheet for CustomStyle {
    fn active(&self) -> ramp::Style {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self) -> ramp::Style {
        ramp::Style {
            line_center_color: Color::from_rgb(0.8, 0.8, 0.8),
            line_up_color: Color::from_rgb(0.0, 1.0, 0.0),
            line_down_color: Color::from_rgb(
                0x8A as f32 / 255.0,
                0xD7 as f32 / 255.0,
                0xFF as f32 / 255.0,
            ),
            ..Self::ACTIVE_STYLE
        }
    }

    fn dragging(&self) -> ramp::Style {
        self.hovered()
    }
}
