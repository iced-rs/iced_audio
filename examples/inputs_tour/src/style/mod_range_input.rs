use iced::Color;
use iced_audio::mod_range_input;

use super::colors;

// Custom style for the ModRangeInput

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_STYLE: mod_range_input::CircleStyle =
        mod_range_input::CircleStyle {
            color: colors::KNOB_ARC_RIGHT,
            border_width: 2,
            border_color: Color::from_rgb(0.0, 0.6, 0.0),
        };
}
impl mod_range_input::StyleSheet for CustomStyle {
    fn active(&self) -> mod_range_input::Style {
        mod_range_input::Style::Circle(Self::ACTIVE_STYLE)
    }

    fn hovered(&self) -> mod_range_input::Style {
        mod_range_input::Style::Circle(mod_range_input::CircleStyle {
            border_width: 1,
            ..Self::ACTIVE_STYLE
        })
    }

    fn dragging(&self) -> mod_range_input::Style {
        self.hovered()
    }
}
