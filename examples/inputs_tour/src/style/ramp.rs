use iced::Color;
use iced_audio::ramp;

use super::colors;

// Custom style for the Ramp widget

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_STYLE: ramp::Appearance = ramp::Appearance {
        back_color: colors::KNOB,
        back_border_width: 2.0,
        back_border_color: colors::KNOB_BORDER,
        line_width: 2.0,
        line_center_color: Color::from_rgb(0.7, 0.7, 0.7),
        line_up_color: Color::from_rgb(0.0, 0.9, 0.0),
        line_down_color: colors::HANDLE,
    };
}
impl ramp::StyleSheet for CustomStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> ramp::Appearance {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self, _style: &Self::Style) -> ramp::Appearance {
        ramp::Appearance {
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

    fn dragging(&self, style: &Self::Style) -> ramp::Appearance {
        self.hovered(style)
    }
}
