use iced::widget::image;
use iced::{Color, Rectangle};
use iced_audio::{h_slider, text_marks, tick_marks, Offset};

use super::colors;

// Custom style for the Rect HSlider

pub struct RectStyle;
impl RectStyle {
    const ACTIVE_RECT_STYLE: h_slider::RectAppearance =
        h_slider::RectAppearance {
            back_color: colors::EMPTY,
            back_border_width: 1.0,
            back_border_radius: 2.0,
            back_border_color: colors::BORDER,
            filled_color: colors::FILLED,
            handle_width: 4,
            handle_color: colors::HANDLE,
            handle_filled_gap: 1.0,
        };
}
impl h_slider::StyleSheet for RectStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> h_slider::Appearance {
        h_slider::Appearance::Rect(Self::ACTIVE_RECT_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> h_slider::Appearance {
        h_slider::Appearance::Rect(h_slider::RectAppearance {
            filled_color: colors::FILLED_HOVER,
            handle_width: 5,
            ..Self::ACTIVE_RECT_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> h_slider::Appearance {
        self.hovered(style)
    }

    fn mod_range_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<h_slider::ModRangeAppearance> {
        Some(h_slider::ModRangeAppearance {
            placement: h_slider::ModRangePlacement::Bottom {
                height: 3.0,
                offset: 2.0,
            },
            back_border_color: Color::TRANSPARENT,
            back_border_width: 0.0,
            back_border_radius: 2.0,
            back_color: Some(colors::KNOB_ARC_EMPTY),
            filled_color: colors::KNOB_ARC,
            filled_inverse_color: colors::KNOB_ARC_RIGHT,
        })
    }
}

// Custom style for the Rect Bipolar HSlider

pub struct RectBipolarStyle;
impl RectBipolarStyle {
    const ACTIVE_RECT_STYLE: h_slider::RectBipolarAppearance =
        h_slider::RectBipolarAppearance {
            back_color: colors::EMPTY,
            back_border_width: 1.0,
            back_border_radius: 2.0,
            back_border_color: colors::BORDER,
            left_filled_color: colors::FILLED,
            right_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            handle_width: 4,
            handle_left_color: colors::HANDLE,
            handle_right_color: Color::from_rgb(0.0, 0.9, 0.0),
            handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            handle_filled_gap: 1.0,
        };
}
impl h_slider::StyleSheet for RectBipolarStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> h_slider::Appearance {
        h_slider::Appearance::RectBipolar(Self::ACTIVE_RECT_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> h_slider::Appearance {
        h_slider::Appearance::RectBipolar(h_slider::RectBipolarAppearance {
            left_filled_color: colors::FILLED_HOVER,
            right_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
            handle_width: 5,
            ..Self::ACTIVE_RECT_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> h_slider::Appearance {
        self.hovered(style)
    }
}

// Custom style for the Texture HSlider

pub struct TextureStyle(pub image::Handle, pub Rectangle);
impl h_slider::StyleSheet for TextureStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> h_slider::Appearance {
        h_slider::Appearance::Texture(h_slider::TextureAppearance {
            rail: h_slider::ClassicRail {
                rail_colors: (
                    [0.0, 0.0, 0.0, 0.9].into(),
                    [0.36, 0.36, 0.36, 0.75].into(),
                ),
                rail_widths: (1.0, 2.0),
                rail_padding: 14.0,
            },
            handle_width: 38,
            image_handle: self.0.clone(),
            image_bounds: self.1,
        })
    }

    fn hovered(&self, style: &Self::Style) -> h_slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> h_slider::Appearance {
        self.active(style)
    }

    fn tick_marks_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<h_slider::TickMarksAppearance> {
        Some(h_slider::TickMarksAppearance {
            style: tick_marks::Appearance {
                tier_1: tick_marks::Shape::Line {
                    length: 12.0,
                    width: 2.0,
                    color: [0.56, 0.56, 0.56, 0.75].into(),
                },
                tier_2: tick_marks::Shape::Line {
                    length: 10.0,
                    width: 1.0,
                    color: [0.56, 0.56, 0.56, 0.75].into(),
                },
                tier_3: tick_marks::Shape::Line {
                    length: 8.0,
                    width: 1.0,
                    color: [0.56, 0.56, 0.56, 0.75].into(),
                },
            },
            placement: tick_marks::Placement::CenterSplit {
                offset: Offset::ZERO,
                fill_length: false,
                gap: 9.0,
            },
        })
    }

    fn text_marks_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<h_slider::TextMarksAppearance> {
        Some(h_slider::TextMarksAppearance {
            style: text_marks::Appearance {
                color: [0.16, 0.16, 0.16, 0.9].into(),
                text_size: 12,
                font: Default::default(),
                bounds_width: 30,
                bounds_height: 14,
            },
            placement: text_marks::Placement::Center {
                align: text_marks::Align::Start,
                offset: Offset { x: 0.0, y: 20.0 },
            },
        })
    }
}
