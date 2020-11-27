use iced::Color;
use iced_audio::{knob, text_marks};

use super::colors;

// Custom style for the Knob

pub struct CustomStyleCircle;
impl CustomStyleCircle {
    const ACTIVE_CIRCLE_NOTCH: knob::CircleNotch = knob::CircleNotch {
        color: colors::HANDLE,
        border_width: 1.0,
        border_color: colors::FILLED,
        diameter: knob::StyleLength::Scaled(0.21),
        offset: knob::StyleLength::Scaled(0.21),
    };
    const ACTIVE_CIRCLE_STYLE: knob::CircleStyle = knob::CircleStyle {
        color: colors::KNOB,
        border_width: 3.0,
        border_color: colors::KNOB_BORDER,
        notch: knob::NotchShape::Circle(Self::ACTIVE_CIRCLE_NOTCH),
    };
}
impl knob::StyleSheet for CustomStyleCircle {
    fn active(&self) -> knob::Style {
        knob::Style::Circle(Self::ACTIVE_CIRCLE_STYLE)
    }

    fn hovered(&self) -> knob::Style {
        knob::Style::Circle(knob::CircleStyle {
            notch: knob::NotchShape::Circle(knob::CircleNotch {
                color: colors::HANDLE_HOVER,
                border_color: colors::FILLED_HOVER,
                ..Self::ACTIVE_CIRCLE_NOTCH
            }),
            ..Self::ACTIVE_CIRCLE_STYLE
        })
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn value_arc_style(&self) -> Option<knob::ValueArcStyle> {
        Some(knob::ValueArcStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            left_filled_color: colors::KNOB_ARC,
            right_filled_color: None,
            cap: knob::LineCap::Butt,
        })
    }

    fn mod_range_arc_style(&self) -> Option<knob::ModRangeArcStyle> {
        Some(knob::ModRangeArcStyle {
            width: 3.0,
            offset: 6.0,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            filled_color: colors::KNOB_ARC_RIGHT,
            filled_inverse_color: colors::KNOB_ARC_RIGHT,
            cap: knob::LineCap::Butt,
        })
    }

    fn text_marks_style(&self) -> Option<knob::TextMarksStyle> {
        Some(knob::TextMarksStyle {
            style: text_marks::Style {
                color: [0.16, 0.16, 0.16, 0.9].into(),
                text_size: 11,
                font: Default::default(),
                bounds_width: 20,
                bounds_height: 20,
            },
            offset: 15.0,
            h_char_offset: 3.0,
            v_offset: -0.75,
        })
    }
}

// Custom style for the Knob

pub struct CustomStyleLine;
impl CustomStyleLine {
    const ACTIVE_CIRCLE_NOTCH: knob::LineNotch = knob::LineNotch {
        color: Color::from_rgb(0.0, 0.82, 0.0),
        width: knob::StyleLength::Units(3.5),
        length: knob::StyleLength::Scaled(0.12),
        offset: knob::StyleLength::Units(5.0),
        cap: knob::LineCap::Round,
    };
    const ACTIVE_CIRCLE_STYLE: knob::CircleStyle = knob::CircleStyle {
        color: colors::KNOB,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        notch: knob::NotchShape::Line(Self::ACTIVE_CIRCLE_NOTCH),
    };
}
impl knob::StyleSheet for CustomStyleLine {
    fn active(&self) -> knob::Style {
        knob::Style::Circle(Self::ACTIVE_CIRCLE_STYLE)
    }

    #[allow(irrefutable_let_patterns)]
    fn hovered(&self) -> knob::Style {
        self.active()
    }

    fn dragging(&self) -> knob::Style {
        self.active()
    }

    fn value_arc_style(&self) -> Option<knob::ValueArcStyle> {
        Some(knob::ValueArcStyle {
            width: 2.5,
            offset: 2.0,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            left_filled_color: colors::KNOB_ARC,
            right_filled_color: Some(colors::KNOB_ARC_RIGHT),
            cap: knob::LineCap::Round,
        })
    }
}

// Custom style for the Knob

pub struct CustomArc;
impl knob::StyleSheet for CustomArc {
    fn active(&self) -> knob::Style {
        knob::Style::Arc(knob::ArcStyle {
            width: knob::StyleLength::Units(3.15),
            empty_color: colors::KNOB_ARC_EMPTY,
            filled_color: colors::KNOB_ARC,
            notch: knob::NotchShape::Line(knob::LineNotch {
                color: colors::KNOB_ARC,
                width: knob::StyleLength::Units(3.15),
                length: knob::StyleLength::Scaled(0.25),
                cap: knob::LineCap::Round,
                offset: knob::StyleLength::Units(2.5),
            }),
            cap: knob::LineCap::Round,
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

    fn mod_range_arc_style(&self) -> Option<knob::ModRangeArcStyle> {
        Some(knob::ModRangeArcStyle {
            width: 3.0,
            offset: 1.5,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            filled_color: colors::KNOB_ARC,
            filled_inverse_color: colors::KNOB_ARC_RIGHT,
            cap: knob::LineCap::Round,
        })
    }
}

// Custom style for the Knob

pub struct CustomArcBipolar;
impl CustomArcBipolar {
    const NOTCH_CENTER: knob::LineNotch = knob::LineNotch {
        color: colors::KNOB_ARC_EMPTY,
        width: knob::StyleLength::Units(3.15),
        length: knob::StyleLength::Scaled(0.39),
        cap: knob::LineCap::Butt,
        offset: knob::StyleLength::Units(0.0),
    };
}
impl knob::StyleSheet for CustomArcBipolar {
    fn active(&self) -> knob::Style {
        knob::Style::ArcBipolar(knob::ArcBipolarStyle {
            width: knob::StyleLength::Units(3.15),
            empty_color: colors::KNOB_ARC_EMPTY,
            left_filled_color: colors::KNOB_ARC,
            right_filled_color: colors::KNOB_ARC_RIGHT,
            notch_center: knob::NotchShape::Line(Self::NOTCH_CENTER),
            notch_left_right: Some((
                knob::NotchShape::Line(knob::LineNotch {
                    color: colors::KNOB_ARC,
                    ..Self::NOTCH_CENTER
                }),
                knob::NotchShape::Line(knob::LineNotch {
                    color: colors::KNOB_ARC_RIGHT,
                    ..Self::NOTCH_CENTER
                }),
            )),
            cap: knob::LineCap::Butt,
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
