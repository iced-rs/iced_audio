use iced::{button, image, Background, Color, Vector};
use iced_audio::{h_slider, knob, ramp, v_slider, xy_pad};

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

// Custom style for the Rect HSlider

pub struct HSliderRectStyle;
impl h_slider::StyleSheet for HSliderRectStyle {
    fn active(&self) -> h_slider::Style {
        h_slider::Style::Rect(h_slider::RectStyle {
            back_empty_color: EMPTY_COLOR,
            back_filled_color: FILLED_COLOR,
            border_color: BORDER_COLOR,
            border_radius: 2,
            border_width: 1,
            handle_width: 4,
            handle_color: HANDLE_COLOR,
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> h_slider::Style {
        let active = self.active();
        if let h_slider::Style::Rect(active) = active {
            h_slider::Style::Rect(h_slider::RectStyle {
                back_filled_color: FILLED_HOVER_COLOR,
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

// Custom style for the Rect VSlider

pub struct VSliderRectStyle;
impl v_slider::StyleSheet for VSliderRectStyle {
    fn active(&self) -> v_slider::Style {
        v_slider::Style::Rect(v_slider::RectStyle {
            back_empty_color: EMPTY_COLOR,
            back_filled_color: FILLED_COLOR,
            border_color: BORDER_COLOR,
            border_radius: 2,
            border_width: 1,
            handle_height: 4,
            handle_color: HANDLE_COLOR,
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> v_slider::Style {
        let active = self.active();
        if let v_slider::Style::Rect(active) = active {
            v_slider::Style::Rect(v_slider::RectStyle {
                back_filled_color: FILLED_HOVER_COLOR,
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

// Custom style for the Rect Bipolar HSlider

pub struct HSliderRectBipolarStyle;
impl h_slider::StyleSheet for HSliderRectBipolarStyle {
    fn active(&self) -> h_slider::Style {
        h_slider::Style::RectBipolar(h_slider::RectBipolarStyle {
            back_left_empty_color: EMPTY_COLOR,
            back_left_filled_color: FILLED_COLOR,
            back_right_empty_color: EMPTY_COLOR,
            back_right_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            border_color: BORDER_COLOR,
            border_radius: 2,
            border_width: 1,
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
                back_left_filled_color: FILLED_HOVER_COLOR,
                back_right_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
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
            back_bottom_empty_color: EMPTY_COLOR,
            back_bottom_filled_color: FILLED_COLOR,
            back_top_empty_color: EMPTY_COLOR,
            back_top_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
            border_color: BORDER_COLOR,
            border_radius: 2,
            border_width: 1,
            handle_height: 4,
            handle_bottom_color: HANDLE_COLOR,
            handle_top_color: Color::from_rgb(0.0, 0.9, 0.0),
            handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
            handle_filled_gap: 1,
        })
    }

    fn hovered(&self) -> v_slider::Style {
        let active = self.active();
        if let v_slider::Style::RectBipolar(active) = active {
            v_slider::Style::RectBipolar(v_slider::RectBipolarStyle {
                back_bottom_filled_color: FILLED_HOVER_COLOR,
                back_top_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
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
            rail_heights: (1, 2),
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
            scale_tier_1: 0.85,
            scale_tier_2: 0.8,
            scale_tier_3: 0.75,

            width_tier_1: 2,
            width_tier_2: 1,
            width_tier_3: 1,

            color_tier_1: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_2: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_3: [0.56, 0.56, 0.56, 0.75].into(),

            center_offset: 5,
            handle_offset: 19,
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
            scale_tier_1: 0.85,
            scale_tier_2: 0.8,
            scale_tier_3: 0.75,

            height_tier_1: 2,
            height_tier_2: 1,
            height_tier_3: 1,

            color_tier_1: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_2: [0.56, 0.56, 0.56, 0.75].into(),
            color_tier_3: [0.56, 0.56, 0.56, 0.75].into(),

            center_offset: 5,
            handle_offset: 19,
        })
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleCircle;
impl knob::StyleSheet for KnobCustomStyleCircle {
    fn active(&self) -> knob::Style {
        knob::Style::VectorCircle(knob::VectorCircleStyle {
            knob_color: KNOB_COLOR,
            knob_border_width: 3,
            knob_border_color: KNOB_BORDER_COLOR,
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
        if let knob::Style::VectorCircle(active) = self.active() {
            knob::Style::VectorCircle(knob::VectorCircleStyle {
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

    fn tick_mark_style(&self) -> Option<knob::TickMarkStyle> {
        Some(knob::TickMarkStyle::Circle(knob::CircleTickMarks {
            diameter_tier_1: 2,
            diameter_tier_2: 2,
            diameter_tier_3: 2,

            color_tier_1: Color::from_rgb(0.45, 0.45, 0.45),
            color_tier_2: Color::from_rgb(0.45, 0.45, 0.45),
            color_tier_3: Color::from_rgb(0.45, 0.45, 0.45),

            offset: 3.2,
        }))
    }
}

// Custom style for the Knob

pub struct KnobCustomStyleLine;
impl knob::StyleSheet for KnobCustomStyleLine {
    fn active(&self) -> knob::Style {
        knob::Style::VectorLine(knob::VectorLineStyle {
            knob_color: KNOB_COLOR,
            knob_border_width: 0,
            knob_border_color: KNOB_BORDER_COLOR,
            notch_color: Color::from_rgb(0.0, 0.8, 0.0),
            notch_width: 3.5,
            notch_scale: 0.35.into(),
            notch_offset: 0.21.into(),
        })
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        let active = self.active();
        if let knob::Style::VectorLine(active) = self.active() {
            knob::Style::VectorLine(knob::VectorLineStyle {
                knob_border_width: 2,
                notch_width: 3.0,
                notch_color: Color::from_rgb(0.0, 0.85, 0.0),
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn tick_mark_style(&self) -> Option<knob::TickMarkStyle> {
        Some(knob::TickMarkStyle::Line(knob::LineTickMarks {
            width_tier_1: 2.0,
            width_tier_2: 1.0,
            width_tier_3: 1.0,

            length_tier_1: 3.5,
            length_tier_2: 2.5,
            length_tier_3: 2.5,

            color_tier_1: Color::from_rgb(0.56, 0.56, 0.56),
            color_tier_2: Color::from_rgb(0.45, 0.45, 0.45),
            color_tier_3: Color::from_rgb(0.45, 0.45, 0.45),

            offset: 2.1,
        }))
    }
}

// Custom style for the Knob

pub struct KnobCustomArc;
impl knob::StyleSheet for KnobCustomArc {
    fn active(&self) -> knob::Style {
        knob::Style::Arc(knob::ArcStyle {
            arc_width: 3.15,
            arc_empty_color: EMPTY_COLOR,
            arc_filled_color: KNOB_ARC_COLOR,
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
}

// Custom style for the Knob

pub struct KnobCustomArcBipolar;
impl knob::StyleSheet for KnobCustomArcBipolar {
    fn active(&self) -> knob::Style {
        knob::Style::ArcBipolar(knob::ArcBipolarStyle {
            arc_width: 3.15,
            arc_empty_color: EMPTY_COLOR,
            arc_left_color: KNOB_ARC_COLOR,
            arc_right_color: KNOB_ARC_RIGHT_COLOR,
            notch: Some(knob::ArcBipolarNotch {
                width: 3.15,
                length_scale: 0.55.into(),
                color_center: EMPTY_COLOR,
                color_left: KNOB_ARC_COLOR,
                color_right: KNOB_ARC_RIGHT_COLOR,
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
            background_color: KNOB_COLOR,
            border_width: 2,
            border_color: KNOB_BORDER_COLOR,
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
