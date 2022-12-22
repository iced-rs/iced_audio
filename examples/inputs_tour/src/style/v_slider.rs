use iced::widget::image;
use iced::{Color, Rectangle};
use iced_audio::{text_marks, tick_marks, v_slider, Offset};

use super::colors;

// Custom style for the Rect VSlider

pub struct RectStyle;
impl RectStyle {
    const ACTIVE_RECT_STYLE: v_slider::RectAppearance =
        v_slider::RectAppearance {
            back_color: colors::EMPTY,
            back_border_width: 1.0,
            back_border_radius: 2.0,
            back_border_color: colors::BORDER,
            filled_color: colors::FILLED,
            handle_height: 4,
            handle_color: colors::HANDLE,
            handle_filled_gap: 1.0,
        };
}
impl v_slider::StyleSheet for RectStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> v_slider::Appearance {
        v_slider::Appearance::Rect(Self::ACTIVE_RECT_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> v_slider::Appearance {
        v_slider::Appearance::Rect(v_slider::RectAppearance {
            filled_color: colors::FILLED_HOVER,
            handle_height: 5,
            ..Self::ACTIVE_RECT_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> v_slider::Appearance {
        self.hovered(style)
    }

    fn mod_range_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<v_slider::ModRangeAppearance> {
        Some(v_slider::ModRangeAppearance {
            placement: v_slider::ModRangePlacement::CenterFilled {
                edge_padding: 0.0,
            },
            back_border_width: 1.0,
            back_border_radius: 2.0,
            back_border_color: Color::TRANSPARENT,
            back_color: None,
            filled_color: Color {
                r: 0.0,
                g: 0.77,
                b: 0.0,
                a: 0.2,
            },
            filled_inverse_color: Color {
                r: 0.0,
                g: 0.77,
                b: 0.0,
                a: 0.2,
            },
        })
    }
}

// Custom style for the Rect Bipolar VSlider

pub struct RectBipolarStyle;
impl RectBipolarStyle {
    const ACTIVE_RECT_STYLE: v_slider::RectBipolarAppearance =
        v_slider::RectBipolarAppearance {
            back_color: colors::EMPTY,
            back_border_width: 1.0,
            back_border_radius: 2.0,
            back_border_color: colors::BORDER,
            top_filled_color: colors::FILLED,
            bottom_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            handle_height: 4,
            handle_top_color: colors::HANDLE,
            handle_bottom_color: Color::from_rgb(0.0, 0.9, 0.0),
            handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            handle_filled_gap: 1.0,
        };
}
impl v_slider::StyleSheet for RectBipolarStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> v_slider::Appearance {
        v_slider::Appearance::RectBipolar(Self::ACTIVE_RECT_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> v_slider::Appearance {
        v_slider::Appearance::RectBipolar(v_slider::RectBipolarAppearance {
            top_filled_color: colors::FILLED_HOVER,
            bottom_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
            handle_height: 5,
            ..Self::ACTIVE_RECT_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> v_slider::Appearance {
        self.hovered(style)
    }
}

// Custom style for the Texture VSlider

pub struct TextureStyle(pub image::Handle, pub Rectangle);
impl v_slider::StyleSheet for TextureStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> v_slider::Appearance {
        v_slider::Appearance::Texture(v_slider::TextureAppearance {
            rail: v_slider::ClassicRail {
                rail_colors: (
                    [0.0, 0.0, 0.0, 0.9].into(),
                    [0.36, 0.36, 0.36, 0.75].into(),
                ),
                rail_widths: (1.0, 2.0),
                rail_padding: 14.0,
            },
            handle_height: 38,
            image_handle: self.0.clone(),
            image_bounds: self.1,
        })
    }

    fn hovered(&self, style: &Self::Style) -> v_slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> v_slider::Appearance {
        self.active(style)
    }

    fn tick_marks_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<v_slider::TickMarksAppearance> {
        Some(v_slider::TickMarksAppearance {
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
    ) -> Option<v_slider::TextMarksAppearance> {
        Some(v_slider::TextMarksAppearance {
            style: text_marks::Appearance {
                color: [0.16, 0.16, 0.16, 0.9].into(),
                text_size: 12,
                font: Default::default(),
                bounds_width: 30,
                bounds_height: 14,
            },
            placement: text_marks::Placement::Center {
                align: text_marks::Align::End,
                offset: Offset { x: -20.0, y: 0.0 },
            },
        })
    }
}
