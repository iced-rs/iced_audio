use iced::Color;
use iced_audio::xy_pad;

use super::colors;

// Custom style for the XYPad widget

pub struct CustomStyle;
impl CustomStyle {
    const ACTIVE_HANDLE: xy_pad::HandleSquare = xy_pad::HandleSquare {
        color: colors::FILLED,
        size: 10,
        border_width: 1.0,
        border_radius: 2.0,
        border_color: colors::HANDLE,
    };
    const ACTIVE_STYLE: xy_pad::Style = xy_pad::Style {
        rail_width: 1.0,
        h_rail_color: colors::HANDLE,
        v_rail_color: colors::HANDLE,
        handle: xy_pad::HandleShape::Square(Self::ACTIVE_HANDLE),
        back_color: colors::EMPTY,
        border_width: 2.0,
        border_color: Color::BLACK,
        center_line_width: 1.0,
        center_line_color: Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 0.7,
        },
    };
}
impl xy_pad::StyleSheet for CustomStyle {
    fn active(&self) -> xy_pad::Style {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self) -> xy_pad::Style {
        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: colors::FILLED_HOVER,
                size: 12,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
    }

    fn dragging(&self) -> xy_pad::Style {
        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: colors::FILLED_HOVER,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
    }
}
