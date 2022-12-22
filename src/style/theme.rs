//! Implement `iced_audio` styles for `iced`'s theme.

use crate::core::{KnobAngleRange, Offset};
use crate::style::{
    default_colors, h_slider, knob, mod_range_input, ramp, text_marks,
    tick_marks, v_slider, xy_pad,
};

/// The style of a HSlider.
#[derive(Default)]
pub enum HSlider {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn h_slider::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for HSlider
where
    S: 'static + h_slider::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        HSlider::Custom(Box::new(val))
    }
}

impl h_slider::StyleSheet for iced_native::Theme {
    type Style = HSlider;

    fn active(&self, style: &Self::Style) -> h_slider::Appearance {
        match style {
            HSlider::Default => {
                h_slider::Appearance::Classic(Default::default())
            }
            HSlider::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> h_slider::Appearance {
        match style {
            HSlider::Default => {
                h_slider::Appearance::Classic(h_slider::ClassicAppearance {
                    handle: h_slider::ClassicHandle {
                        color: default_colors::LIGHT_BACK_HOVER,
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            HSlider::Custom(custom) => custom.hovered(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> h_slider::Appearance {
        match style {
            HSlider::Default => {
                h_slider::Appearance::Classic(h_slider::ClassicAppearance {
                    handle: h_slider::ClassicHandle {
                        color: default_colors::LIGHT_BACK_DRAG,
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            HSlider::Custom(custom) => custom.dragging(self),
        }
    }

    fn tick_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<h_slider::TickMarksAppearance> {
        match style {
            HSlider::Default => Some(h_slider::TickMarksAppearance {
                style: tick_marks::Appearance {
                    tier_1: tick_marks::Shape::Line {
                        length: 24.0,
                        width: 2.0,
                        color: default_colors::TICK_TIER_1,
                    },
                    tier_2: tick_marks::Shape::Line {
                        length: 22.0,
                        width: 1.0,
                        color: default_colors::TICK_TIER_2,
                    },
                    tier_3: tick_marks::Shape::Line {
                        length: 18.0,
                        width: 1.0,
                        color: default_colors::TICK_TIER_3,
                    },
                },
                placement: tick_marks::Placement::Center {
                    offset: Offset::ZERO,
                    fill_length: false,
                },
            }),
            HSlider::Custom(custom) => custom.tick_marks_appearance(self),
        }
    }

    fn mod_range_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<h_slider::ModRangeAppearance> {
        match style {
            HSlider::Default => None,
            HSlider::Custom(custom) => custom.mod_range_appearance(self),
        }
    }

    fn mod_range_appearance_2(
        &self,
        style: &Self::Style,
    ) -> Option<h_slider::ModRangeAppearance> {
        match style {
            HSlider::Default => None,
            HSlider::Custom(custom) => custom.mod_range_appearance_2(self),
        }
    }

    fn text_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<h_slider::TextMarksAppearance> {
        match style {
            HSlider::Default => Some(h_slider::TextMarksAppearance {
                style: Default::default(),
                placement: text_marks::Placement::RightOrBottom {
                    inside: false,
                    offset: Offset { x: 0.0, y: 7.0 },
                },
            }),
            HSlider::Custom(custom) => custom.text_marks_appearance(self),
        }
    }
}

/// The style of a Knob.
#[derive(Default)]
pub enum Knob {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn knob::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for Knob
where
    S: 'static + knob::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        Knob::Custom(Box::new(val))
    }
}

impl knob::StyleSheet for iced_native::Theme {
    type Style = Knob;

    fn active(&self, style: &Self::Style) -> knob::Appearance {
        match style {
            Knob::Default => knob::Appearance::Circle(Default::default()),
            Knob::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> knob::Appearance {
        match style {
            Knob::Default => knob::Appearance::Circle(knob::CircleAppearance {
                color: default_colors::KNOB_BACK_HOVER,
                ..Default::default()
            }),
            Knob::Custom(custom) => custom.hovered(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> knob::Appearance {
        match style {
            Knob::Default => self.hovered(style),
            Knob::Custom(custom) => custom.dragging(self),
        }
    }

    fn angle_range(&self, style: &Self::Style) -> KnobAngleRange {
        match style {
            Knob::Default => KnobAngleRange::default(),
            Knob::Custom(custom) => custom.angle_range(self),
        }
    }

    fn tick_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<knob::TickMarksAppearance> {
        match style {
            Knob::Default => Some(knob::TickMarksAppearance {
                style: tick_marks::Appearance {
                    tier_1: tick_marks::Shape::Circle {
                        diameter: 4.0,
                        color: default_colors::TICK_TIER_1,
                    },
                    tier_2: tick_marks::Shape::Circle {
                        diameter: 2.0,
                        color: default_colors::TICK_TIER_2,
                    },
                    tier_3: tick_marks::Shape::Circle {
                        diameter: 2.0,
                        color: default_colors::TICK_TIER_3,
                    },
                },
                offset: 3.5,
            }),
            Knob::Custom(custom) => custom.tick_marks_appearance(self),
        }
    }

    fn value_arc_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<knob::ValueArcAppearance> {
        match style {
            Knob::Default => None,
            Knob::Custom(custom) => custom.value_arc_appearance(self),
        }
    }

    fn mod_range_arc_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<knob::ModRangeArcAppearance> {
        match style {
            Knob::Default => None,
            Knob::Custom(custom) => custom.mod_range_arc_appearance(self),
        }
    }

    fn mod_range_arc_appearance_2(
        &self,
        style: &Self::Style,
    ) -> Option<knob::ModRangeArcAppearance> {
        match style {
            Knob::Default => None,
            Knob::Custom(custom) => custom.mod_range_arc_appearance_2(self),
        }
    }

    fn text_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<knob::TextMarksAppearance> {
        match style {
            Knob::Default => Some(knob::TextMarksAppearance {
                style: Default::default(),
                offset: 14.0,
                h_char_offset: 3.0,
                v_offset: -0.75,
            }),
            Knob::Custom(custom) => custom.text_marks_appearance(self),
        }
    }
}

/// The style of a [`ModRangeInput`].
#[derive(Default)]
pub enum ModRangeInput {
    /// The default style.
    #[default]
    Default,
    /// The invisible style.
    Invisible,
    /// A custom style.
    Custom(Box<dyn mod_range_input::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for ModRangeInput
where
    S: 'static + mod_range_input::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        ModRangeInput::Custom(Box::new(val))
    }
}

impl mod_range_input::StyleSheet for iced_native::Theme {
    type Style = ModRangeInput;

    fn active(&self, style: &Self::Style) -> mod_range_input::Appearance {
        match style {
            ModRangeInput::Default => {
                mod_range_input::Appearance::Circle(Default::default())
            }
            ModRangeInput::Invisible => mod_range_input::Appearance::Invisible,
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> mod_range_input::Appearance {
        match style {
            ModRangeInput::Default => mod_range_input::Appearance::Circle(
                mod_range_input::CircleAppearance {
                    color: default_colors::KNOB_BACK_HOVER,
                    ..Default::default()
                },
            ),
            ModRangeInput::Invisible => self.active(style),
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> mod_range_input::Appearance {
        match style {
            ModRangeInput::Default => self.hovered(style),
            ModRangeInput::Invisible => self.active(style),
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }
}

/// The style of a Ramp.
#[derive(Default)]
pub enum Ramp {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn ramp::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for Ramp
where
    S: 'static + ramp::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        Ramp::Custom(Box::new(val))
    }
}

impl ramp::StyleSheet for iced_native::Theme {
    type Style = Ramp;

    fn active(&self, style: &Self::Style) -> ramp::Appearance {
        match style {
            Ramp::Default => Default::default(),
            Ramp::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> ramp::Appearance {
        match style {
            Ramp::Default => ramp::Appearance {
                back_color: default_colors::RAMP_BACK_HOVER,
                ..Default::default()
            },
            Ramp::Custom(custom) => custom.active(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> ramp::Appearance {
        self.hovered(style)
    }
}

/// The style of a VSlider.
#[derive(Default)]
pub enum VSlider {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn v_slider::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for VSlider
where
    S: 'static + v_slider::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        VSlider::Custom(Box::new(val))
    }
}

impl v_slider::StyleSheet for iced_native::Theme {
    type Style = VSlider;

    fn active(&self, style: &Self::Style) -> v_slider::Appearance {
        match style {
            VSlider::Default => {
                v_slider::Appearance::Classic(Default::default())
            }
            VSlider::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> v_slider::Appearance {
        match style {
            VSlider::Default => {
                v_slider::Appearance::Classic(v_slider::ClassicAppearance {
                    handle: v_slider::ClassicHandle {
                        color: default_colors::LIGHT_BACK_HOVER,
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            VSlider::Custom(custom) => custom.hovered(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> v_slider::Appearance {
        match style {
            VSlider::Default => {
                v_slider::Appearance::Classic(v_slider::ClassicAppearance {
                    handle: v_slider::ClassicHandle {
                        color: default_colors::LIGHT_BACK_DRAG,
                        ..Default::default()
                    },
                    ..Default::default()
                })
            }
            VSlider::Custom(custom) => custom.dragging(self),
        }
    }

    fn tick_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<v_slider::TickMarksAppearance> {
        match style {
            VSlider::Default => Some(v_slider::TickMarksAppearance {
                style: tick_marks::Appearance {
                    tier_1: tick_marks::Shape::Line {
                        length: 24.0,
                        width: 2.0,
                        color: default_colors::TICK_TIER_1,
                    },
                    tier_2: tick_marks::Shape::Line {
                        length: 22.0,
                        width: 1.0,
                        color: default_colors::TICK_TIER_2,
                    },
                    tier_3: tick_marks::Shape::Line {
                        length: 18.0,
                        width: 1.0,
                        color: default_colors::TICK_TIER_3,
                    },
                },
                placement: tick_marks::Placement::Center {
                    offset: Offset::ZERO,
                    fill_length: false,
                },
            }),
            VSlider::Custom(custom) => custom.tick_marks_appearance(self),
        }
    }

    fn mod_range_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<v_slider::ModRangeAppearance> {
        match style {
            VSlider::Default => None,
            VSlider::Custom(custom) => custom.mod_range_appearance(self),
        }
    }

    fn mod_range_appearance_2(
        &self,
        style: &Self::Style,
    ) -> Option<v_slider::ModRangeAppearance> {
        match style {
            VSlider::Default => None,
            VSlider::Custom(custom) => custom.mod_range_appearance_2(self),
        }
    }

    fn text_marks_appearance(
        &self,
        style: &Self::Style,
    ) -> Option<v_slider::TextMarksAppearance> {
        match style {
            VSlider::Default => Some(v_slider::TextMarksAppearance {
                style: Default::default(),
                placement: text_marks::Placement::LeftOrTop {
                    inside: false,
                    offset: Offset { x: -7.0, y: 0.0 },
                },
            }),
            VSlider::Custom(custom) => custom.text_marks_appearance(self),
        }
    }
}

/// The style of a XYPad.
#[derive(Default)]
pub enum XYPad {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn xy_pad::StyleSheet<Style = iced_native::Theme>>),
}

impl<S> From<S> for XYPad
where
    S: 'static + xy_pad::StyleSheet<Style = iced_native::Theme>,
{
    fn from(val: S) -> Self {
        XYPad::Custom(Box::new(val))
    }
}

impl xy_pad::StyleSheet for iced_native::Theme {
    type Style = XYPad;

    fn active(&self, style: &Self::Style) -> xy_pad::Appearance {
        match style {
            XYPad::Default => Default::default(),
            XYPad::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> xy_pad::Appearance {
        match style {
            XYPad::Default => xy_pad::Appearance {
                handle: xy_pad::HandleShape::Circle(xy_pad::HandleCircle {
                    color: default_colors::LIGHT_BACK_HOVER,
                    ..Default::default()
                }),
                ..Default::default()
            },
            XYPad::Custom(custom) => custom.hovered(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> xy_pad::Appearance {
        match style {
            XYPad::Default => xy_pad::Appearance {
                handle: xy_pad::HandleShape::Circle(xy_pad::HandleCircle {
                    color: default_colors::LIGHT_BACK_DRAG,
                    diameter: 9.0,
                    ..Default::default()
                }),
                ..Default::default()
            },
            XYPad::Custom(custom) => custom.dragging(self),
        }
    }
}
