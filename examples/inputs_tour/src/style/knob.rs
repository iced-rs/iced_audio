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
    const ACTIVE_CIRCLE_STYLE: knob::CircleAppearance =
        knob::CircleAppearance {
            color: colors::KNOB,
            border_width: 3.0,
            border_color: colors::KNOB_BORDER,
            notch: knob::NotchShape::Circle(Self::ACTIVE_CIRCLE_NOTCH),
        };
}
impl knob::StyleSheet for CustomStyleCircle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> knob::Appearance {
        knob::Appearance::Circle(Self::ACTIVE_CIRCLE_STYLE)
    }

    fn hovered(&self, _style: &Self::Style) -> knob::Appearance {
        knob::Appearance::Circle(knob::CircleAppearance {
            notch: knob::NotchShape::Circle(knob::CircleNotch {
                color: colors::HANDLE_HOVER,
                border_color: colors::FILLED_HOVER,
                ..Self::ACTIVE_CIRCLE_NOTCH
            }),
            ..Self::ACTIVE_CIRCLE_STYLE
        })
    }

    fn dragging(&self, style: &Self::Style) -> knob::Appearance {
        self.hovered(style)
    }

    fn value_arc_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<knob::ValueArcAppearance> {
        Some(knob::ValueArcAppearance {
            width: 3.0,
            offset: 1.5,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            left_filled_color: colors::KNOB_ARC,
            right_filled_color: None,
            cap: knob::LineCap::Butt,
        })
    }

    fn mod_range_arc_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<knob::ModRangeArcAppearance> {
        Some(knob::ModRangeArcAppearance {
            width: 3.0,
            offset: 6.0,
            empty_color: Some(colors::KNOB_ARC_EMPTY),
            filled_color: colors::KNOB_ARC_RIGHT,
            filled_inverse_color: colors::KNOB_ARC_RIGHT,
            cap: knob::LineCap::Butt,
        })
    }

    fn text_marks_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<knob::TextMarksAppearance> {
        Some(knob::TextMarksAppearance {
            style: text_marks::Appearance {
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
        width: knob::StyleLength::Fixed(3.5),
        length: knob::StyleLength::Scaled(0.12),
        offset: knob::StyleLength::Fixed(5.0),
        cap: knob::LineCap::Round,
    };
    const ACTIVE_CIRCLE_STYLE: knob::CircleAppearance =
        knob::CircleAppearance {
            color: colors::KNOB,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            notch: knob::NotchShape::Line(Self::ACTIVE_CIRCLE_NOTCH),
        };
}
impl knob::StyleSheet for CustomStyleLine {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> knob::Appearance {
        knob::Appearance::Circle(Self::ACTIVE_CIRCLE_STYLE)
    }

    fn hovered(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn value_arc_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<knob::ValueArcAppearance> {
        Some(knob::ValueArcAppearance {
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
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> knob::Appearance {
        knob::Appearance::Arc(knob::ArcAppearance {
            width: knob::StyleLength::Fixed(3.15),
            empty_color: colors::KNOB_ARC_EMPTY,
            filled_color: colors::KNOB_ARC,
            notch: knob::NotchShape::Line(knob::LineNotch {
                color: colors::KNOB_ARC,
                width: knob::StyleLength::Fixed(3.15),
                length: knob::StyleLength::Scaled(0.25),
                cap: knob::LineCap::Round,
                offset: knob::StyleLength::Fixed(2.5),
            }),
            cap: knob::LineCap::Round,
        })
    }

    fn hovered(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn angle_range(&self, _style: &Self::Style) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }

    fn mod_range_arc_appearance(
        &self,
        _style: &Self::Style,
    ) -> Option<knob::ModRangeArcAppearance> {
        Some(knob::ModRangeArcAppearance {
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
        width: knob::StyleLength::Fixed(3.15),
        length: knob::StyleLength::Scaled(0.39),
        cap: knob::LineCap::Butt,
        offset: knob::StyleLength::Fixed(0.0),
    };
}
impl knob::StyleSheet for CustomArcBipolar {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> knob::Appearance {
        knob::Appearance::ArcBipolar(knob::ArcBipolarAppearance {
            width: knob::StyleLength::Fixed(3.15),
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

    fn hovered(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> knob::Appearance {
        self.active(style)
    }

    fn angle_range(&self, _style: &Self::Style) -> iced_audio::KnobAngleRange {
        iced_audio::KnobAngleRange::from_deg(40.0, 320.0)
    }
}
