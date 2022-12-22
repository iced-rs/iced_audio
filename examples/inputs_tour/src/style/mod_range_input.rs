use iced::Color;
use iced_audio::mod_range_input;

use super::colors;

// Custom style for the ModRangeInput

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_STYLE: mod_range_input::CircleAppearance =
        mod_range_input::CircleAppearance {
            color: colors::KNOB_ARC_RIGHT,
            border_width: 2.0,
            border_color: Color::from_rgb(0.0, 0.6, 0.0),
        };
}
impl mod_range_input::StyleSheet for CustomStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> mod_range_input::Appearance {
        mod_range_input::Appearance::Circle(Self::ACTIVE_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> mod_range_input::Appearance {
        mod_range_input::Appearance::Circle(mod_range_input::CircleAppearance {
            border_width: 1.0,
            ..Self::ACTIVE_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> mod_range_input::Appearance {
        self.hovered(style)
    }
}
