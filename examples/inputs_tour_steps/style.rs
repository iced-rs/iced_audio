use iced::{button, image, Background, Color, Font, Vector};
use iced_audio::{
    bar_text_marks, h_slider, knob, mod_range_input, ramp, v_slider, xy_pad,
};

pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => BUTTON_PRIMARY_COLOR,
                Button::Secondary => BUTTON_SECONDARY_COLOR,
            })),
            border_radius: 12,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}

pub const BUTTON_PRIMARY_COLOR: Color = Color::from_rgb(
    0x32 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0xC8 as f32 / 255.0,
);

pub const BUTTON_SECONDARY_COLOR: Color = Color::from_rgb(
    0x62 as f32 / 255.0,
    0x69 as f32 / 255.0,
    0x73 as f32 / 255.0,
);

pub const EMPTY_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
pub const BORDER_COLOR: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0x33 as f32 / 255.0,
    0x3C as f32 / 255.0,
);
pub const FILLED_COLOR: Color = Color::from_rgb(
    0x29 as f32 / 255.0,
    0x66 as f32 / 255.0,
    0xA3 as f32 / 255.0,
);
pub const FILLED_HOVER_COLOR: Color = Color::from_rgb(
    0x33 as f32 / 255.0,
    0x70 as f32 / 255.0,
    0xAD as f32 / 255.0,
);
pub const HANDLE_COLOR: Color = Color::from_rgb(
    0x75 as f32 / 255.0,
    0xC2 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const HANDLE_HOVER_COLOR: Color = Color::from_rgb(
    0x7A as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const KNOB_COLOR: Color = Color::from_rgb(
    0x56 as f32 / 255.0,
    0x59 as f32 / 255.0,
    0x62 as f32 / 255.0,
);
pub const KNOB_BORDER_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
pub const KNOB_ARC_COLOR: Color = Color::from_rgb(
    0x3D as f32 / 255.0,
    0x9E as f32 / 255.0,
    0xE9 as f32 / 255.0,
);
pub const KNOB_ARC_RIGHT_COLOR: Color = Color::from_rgb(0.0, 0.77, 0.0);
pub const KNOB_ARC_EMPTY_COLOR: Color = Color::from_rgb(0.85, 0.85, 0.85);

// Custom style for the Rect HSlider

pub struct HSliderRectStyle;
impl h_slider::StyleSheet for HSliderRectStyle {
    fn active(&self) -> h_slider::Style {
        h_slider::Style::Rect(h_slider::RectStyle {
            back_color: EMPTY_COLOR,
            back_border_width: 1,
            back_border_radius: 2,
            back_border_color: BORDER_COLOR,
            filled_color: FILLED_COLOR,
            handle_width: 4,
            handle_color: HANDLE_COLOR,
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> h_slider::Style {
        let active = self.active();
        if let h_slider::Style::Rect(active) = active {
            h_slider::Style::Rect(h_slider::RectStyle {
                filled_color: FILLED_HOVER_COLOR,
                handle_width: 5,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> h_slider::Style {
        self.hovered()
    }

    fn mod_range_style(&self) -> Option<h_slider::ModRangeStyle> {
        Some(h_slider::ModRangeStyle {
            width: 3,
            offset: 2,
            placement: h_slider::ModRangePlacement::Bottom,
            empty_color: Some(KNOB_ARC_EMPTY_COLOR),
            filled_color: KNOB_ARC_COLOR,
            filled_inverse_color: KNOB_ARC_RIGHT_COLOR,
        })
    }
}

// Custom style for the Rect VSlider

pub struct VSliderRectStyle;
impl v_slider::StyleSheet for VSliderRectStyle {
    fn active(&self) -> v_slider::Style {
        v_slider::Style::Rect(v_slider::RectStyle {
            back_color: EMPTY_COLOR,
            back_border_width: 1,
            back_border_radius: 2,
            back_border_color: BORDER_COLOR,
            filled_color: FILLED_COLOR,
            handle_color: HANDLE_COLOR,
            handle_height: 4,
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> v_slider::Style {
        let active = self.active();
        if let v_slider::Style::Rect(active) = active {
            v_slider::Style::Rect(v_slider::RectStyle {
                filled_color: FILLED_HOVER_COLOR,
                handle_height: 5,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> v_slider::Style {
        self.hovered()
    }

    fn mod_range_style(&self) -> Option<v_slider::ModRangeStyle> {
        Some(v_slider::ModRangeStyle {
            width: 0,
            offset: 10,
            placement: v_slider::ModRangePlacement::Center,
            empty_color: None,
            filled_color: Color {
                r: 0.0,
                g: 0.7,
                b: 0.0,
                a: 0.3,
            },
            filled_inverse_color: Color {
                r: 0.0,
                g: 0.7,
                b: 0.0,
                a: 0.5,
            },
        })
    }
}

// Custom style for the Rect Bipolar HSlider

pub struct HSliderRectBipolarStyle;
impl h_slider::StyleSheet for HSliderRectBipolarStyle {
    fn active(&self) -> h_slider::Style {
        h_slider::Style::RectBipolar(h_slider::RectBipolarStyle {
            back_color: EMPTY_COLOR,
            back_border_width: 1,
            back_border_radius: 2,
            back_border_color: BORDER_COLOR,
            left_filled_color: FILLED_COLOR,
            right_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            handle_width: 4,
            handle_left_color: HANDLE_COLOR,
            handle_right_color: Color::from_rgb(0.0, 0.9, 0.0),
            handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> h_slider::Style {
        let active = self.active();
        if let h_slider::Style::RectBipolar(active) = active {
            h_slider::Style::RectBipolar(h_slider::RectBipolarStyle {
                left_filled_color: FILLED_HOVER_COLOR,
                right_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
                handle_width: 5,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> h_slider::Style {
        self.hovered()
    }
}

// Custom style for the Rect Bipolar VSlider

pub struct VSliderRectBipolarStyle;
impl v_slider::StyleSheet for VSliderRectBipolarStyle {
    fn active(&self) -> v_slider::Style {
        v_slider::Style::RectBipolar(v_slider::RectBipolarStyle {
            back_color: EMPTY_COLOR,
            back_border_width: 1,
            back_border_radius: 2,
            back_border_color: BORDER_COLOR,
            bottom_filled_color: FILLED_COLOR,
            top_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            handle_bottom_color: HANDLE_COLOR,
            handle_top_color: Color::from_rgb(0.0, 0.9, 0.0),
            handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            handle_height: 4,
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> v_slider::Style {
        let active = self.active();
        if let v_slider::Style::RectBipolar(active) = active {
            v_slider::Style::RectBipolar(v_slider::RectBipolarStyle {
                bottom_filled_color: FILLED_HOVER_COLOR,
                top_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
                handle_height: 5,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> v_slider::Style {
        self.hovered()
    }
}

// Custom style for the Texture HSlider

pub struct HSliderTextureStyle(pub image::Handle);
impl h_slider::StyleSheet for HSliderTextureStyle {
    fn active(&self) -> h_slider::Style {
        h_slider::Style::Texture(h_slider::TextureStyle {
            rail_colors: (
                [0.0, 0.0, 0.0, 0.9].into(),
                [0.36, 0.36, 0.36, 0.75].into(),
            ),
            rail_widths: (1, 2),
            texture: self.0.clone(),
            handle_width: 38,
            texture_padding: None,
        })
    }

    fn hovered(&self) -> h_slider::Style {
        self.active()
    }

    fn dragging(&self) -> h_slider::Style {
        self.active()
    }

    fn tick_mark_style(&self) -> Option<h_slider::TickMarkStyle> {
        Some(h_slider::TickMarkStyle {
            length_scale_tier_1: 0.85,
            length_scale_tier_2: 0.8,
            length_scale_tier_3: 0.75,

            width_tier_1: 2,
            width_tier_2: 1,
            width_tier_3: 1,

            color_tier_1: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_2: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_3: [0.56, 0.56, 0.56, 0.75].into(),

            center_offset: 5,
        })
    }

    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            offset: 5,
            text_size: 12,
            font: Font::Default,
            bounds_width: 30,
            bounds_height: 14,
            placement: bar_text_marks::Placement::RightOrBottom,
        })
    }
}

// Custom style for the Texture VSlider

pub struct VSliderTextureStyle(pub image::Handle);
impl v_slider::StyleSheet for VSliderTextureStyle {
    fn active(&self) -> v_slider::Style {
        v_slider::Style::Texture(v_slider::TextureStyle {
            rail_colors: (
                [0.0, 0.0, 0.0, 0.9].into(),
                [0.36, 0.36, 0.36, 0.75].into(),
            ),
            rail_widths: (1, 2),
            texture: self.0.clone(),
            handle_height: 38,
            texture_padding: None,
        })
    }

    fn hovered(&self) -> v_slider::Style {
        self.active()
    }

    fn dragging(&self) -> v_slider::Style {
        self.active()
    }

    fn tick_mark_style(&self) -> Option<v_slider::TickMarkStyle> {
        Some(v_slider::TickMarkStyle {
            length_scale_tier_1: 0.85,
            length_scale_tier_2: 0.8,
            length_scale_tier_3: 0.75,

            width_tier_1: 2,
            width_tier_2: 1,
            width_tier_3: 1,

            color_tier_1: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_2: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_3: [0.56, 0.56, 0.56, 0.75].into(),

            center_offset: 5,
        })
    }

    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            offset: 5,
            text_size: 12,
            font: Font::Default,
            bounds_width: 30,
            bounds_height: 14,
            placement: bar_text_marks::Placement::LeftOrTop,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleCircle;
impl knob::StyleSheet for KnobCustomStyleCircle {
    fn active(&self) -> knob::Style {
        knob::Style::ClassicCircle(knob::ClassicCircleStyle {
            color: KNOB_COLOR,
            border_width: 3,
            border_color: KNOB_BORDER_COLOR,
            notch_color: HANDLE_COLOR,
            notch_border_width: 1,
            notch_border_color: FILLED_COLOR,
            notch_scale: 0.21.into(),
            notch_offset: 0.21.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        let active = self.active();
        if let knob::Style::ClassicCircle(active) = self.active() {
            knob::Style::ClassicCircle(knob::ClassicCircleStyle {
                notch_color: HANDLE_HOVER_COLOR,
                notch_border_color: FILLED_HOVER_COLOR,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn value_ring_style(&self) -> Option<knob::ValueRingStyle> {
        Some(knob::ValueRingStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: None,
        })
    }

    fn mod_range_ring_style(&self) -> Option<knob::ModRangeRingStyle> {
        Some(knob::ModRangeRingStyle {
            width: 3.0,
            offset: 6.0,
            empty_color: Some(KNOB_ARC_EMPTY_COLOR),
            filled_color: KNOB_ARC_RIGHT_COLOR,
            filled_inverse_color: KNOB_ARC_RIGHT_COLOR,
        })
    }

    fn text_mark_style(&self) -> Option<knob::TextMarkStyle> {
        Some(knob::TextMarkStyle {
            color: [0.16, 0.16, 0.16, 0.9].into(),
            offset: 15.0,
            text_size: 11,
            font: Font::Default,
            bounds_width: 20,
            bounds_height: 20,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleLine;
impl knob::StyleSheet for KnobCustomStyleLine {
    fn active(&self) -> knob::Style {
        knob::Style::ClassicLine(knob::ClassicLineStyle {
            color: KNOB_COLOR,
            border_width: 0,
            border_color: KNOB_BORDER_COLOR,
            notch_color: Color::from_rgb(0.0, 0.82, 0.0),
            notch_width: 3.5,
            notch_scale: 0.35.into(),
            notch_offset: 0.21.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn value_ring_style(&self) -> Option<knob::ValueRingStyle> {
        Some(knob::ValueRingStyle {
            width: 2.5,
            offset: 2.0,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: Some(KNOB_ARC_RIGHT_COLOR),
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomArc;
impl knob::StyleSheet for KnobCustomArc {
    fn active(&self) -> knob::Style {
        knob::Style::Arc(knob::ArcStyle {
            width: 3.15,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            filled_color: KNOB_ARC_COLOR,
            notch: Some(knob::ArcNotch {
                width: 3.15,
                length_scale: 0.55.into(),
                color: KNOB_ARC_COLOR,
            }),
        })
    }

    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn angle_range(&self) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }

    fn mod_range_ring_style(&self) -> Option<knob::ModRangeRingStyle> {
        Some(knob::ModRangeRingStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: Some(KNOB_ARC_EMPTY_COLOR),
            filled_color: KNOB_ARC_COLOR,
            filled_inverse_color: KNOB_ARC_RIGHT_COLOR,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomArcBipolar;
impl knob::StyleSheet for KnobCustomArcBipolar {
    fn active(&self) -> knob::Style {
        knob::Style::ArcBipolar(knob::ArcBipolarStyle {
            width: 3.15,
            empty_color: KNOB_ARC_EMPTY_COLOR,
            left_filled_color: KNOB_ARC_COLOR,
            right_filled_color: KNOB_ARC_RIGHT_COLOR,
            notch: Some(knob::ArcBipolarNotch {
                width: 3.15,
                length_scale: 0.55.into(),
                center_color: EMPTY_COLOR,
                left_color: KNOB_ARC_COLOR,
                right_color: KNOB_ARC_RIGHT_COLOR,
            }),
        })
    }

    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn angle_range(&self) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }
}

// Custom style for the ModRangeInput

pub struct ModRangeInputCustom;

impl mod_range_input::StyleSheet for ModRangeInputCustom {
    fn active(&self) -> mod_range_input::Style {
        mod_range_input::Style::Circle(mod_range_input::CircleStyle {
            color: KNOB_ARC_RIGHT_COLOR,
            border_width: 2,
            border_color: Color::from_rgb(0.0, 0.6, 0.0),
        })
    }

    fn hovered(&self) -> mod_range_input::Style {
        let active = self.active();
        if let mod_range_input::Style::Circle(active) = self.active() {
            mod_range_input::Style::Circle(mod_range_input::CircleStyle {
                border_width: 1,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> mod_range_input::Style {
        self.hovered()
    }
}

// Custom style for the Texture VSlider

pub struct XYPadCustomStyle;
impl xy_pad::StyleSheet for XYPadCustomStyle {
    fn active(&self) -> xy_pad::Style {
        xy_pad::Style {
            rail_width: 1,
            h_rail_color: HANDLE_COLOR,
            v_rail_color: HANDLE_COLOR,
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_COLOR,
                size: 10,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            back_color: EMPTY_COLOR,
            border_width: 1,
            border_color: Color::BLACK,
            center_line_width: 1,
            center_line_color: [0.0, 0.0, 0.0, 0.4].into(),
        }
    }

    fn hovered(&self) -> xy_pad::Style {
        let active = self.active();

        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_HOVER_COLOR,
                size: 12,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            ..active
        }
    }

    fn dragging(&self) -> xy_pad::Style {
        let active = self.active();

        xy_pad::Style {
            handle: xy_pad::HandleShape::Square(xy_pad::HandleSquare {
                color: FILLED_HOVER_COLOR,
                size: 10,
                border_width: 1,
                border_radius: 2,
                border_color: HANDLE_COLOR,
            }),
            ..active
        }
    }
}

// Custom style for the Texture VSlider

pub struct RampCustomStyle;
impl ramp::StyleSheet for RampCustomStyle {
    fn active(&self) -> ramp::Style {
        ramp::Style {
            back_color: KNOB_COLOR,
            back_border_width: 2,
            back_border_color: KNOB_BORDER_COLOR,
            line_width: 2.0,
            line_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            line_up_color: Color::from_rgb(0.0, 0.9, 0.0),
            line_down_color: HANDLE_COLOR,
        }
    }

    fn hovered(&self) -> ramp::Style {
        let active = self.active();

        ramp::Style {
            line_center_color: Color::from_rgb(0.8, 0.8, 0.8),
            line_up_color: Color::from_rgb(0.0, 1.0, 0.0),
            line_down_color: Color::from_rgb(
                0x8A as f32 / 255.0,
                0xD7 as f32 / 255.0,
                0xFF as f32 / 255.0,
            ),
            ..active
        }
    }

    fn dragging(&self) -> ramp::Style {
        self.hovered()
    }
}
