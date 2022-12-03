use iced::widget::{checkbox, column, container, row, text};
use iced::{Alignment, Element, Length};

use iced_audio::{
    style::theme, FloatRange, HSlider, Knob, ModRangeInput, ModulationRange,
    Normal, NormalParam, VSlider,
};

use crate::{style, StepContainer};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum ModRangesID {}

#[derive(Debug, Clone)]
pub enum Message {
    RangeStart(Normal),
    RangeEnd(Normal),
    Knob1(Normal),
    HSlider1(Normal),
    VSlider1(Normal),
    ModKnob1(Normal),
    ModKnob2(Normal),
    ModRangeInput1(Normal),
    ModRangeInput2(Normal),
    ToggleModRange(bool),
}

pub struct ModRanges {
    float_range: FloatRange,
    float_range_bipolar: FloatRange,

    knob_start_param: NormalParam,
    knob_end_param: NormalParam,

    knob1_param: NormalParam,
    h_slider1_param: NormalParam,
    v_slider1_param: NormalParam,
    mod_range_1: ModulationRange,

    auto_input1_param: NormalParam,
    knob_auto1_param: NormalParam,
    knob_auto1_mod_range: ModulationRange,

    auto_input2_param: NormalParam,
    knob_auto2_param: NormalParam,
    knob_auto2_mod_range: ModulationRange,

    mod_range_toggle_value: bool,

    output_text: String,
}

impl Default for ModRanges {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default();
        let float_range_bipolar = FloatRange::default_bipolar();

        // create application

        Self {
            float_range,
            float_range_bipolar,

            // initialize the state of the Knob widget
            knob_start_param: float_range.default_normal_param(),
            knob_end_param: float_range.default_normal_param(),

            mod_range_1: ModulationRange::default(),

            knob1_param: float_range.default_normal_param(),

            h_slider1_param: float_range.default_normal_param(),

            v_slider1_param: float_range.default_normal_param(),

            auto_input1_param: float_range_bipolar.default_normal_param(),

            knob_auto1_param: float_range.default_normal_param(),
            auto_input2_param: float_range_bipolar.default_normal_param(),
            knob_auto1_mod_range: ModulationRange::default(),

            knob_auto2_param: float_range.default_normal_param(),
            output_text: String::from("Move a widget"),
            knob_auto2_mod_range: ModulationRange::default(),

            mod_range_toggle_value: true,
        }
    }
}

impl ModRanges {
    pub fn title(&self) -> &str {
        "Modulation Ranges"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RangeStart(normal) => {
                self.knob_start_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "RangeStart",
                    self.float_range.unmap_to_value(normal),
                );

                self.mod_range_1.start = normal;
            }
            Message::RangeEnd(normal) => {
                self.knob_end_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "RangeEnd",
                    self.float_range.unmap_to_value(normal),
                );

                self.mod_range_1.end = normal;
            }
            Message::Knob1(normal) => {
                self.knob1_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "Knob1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::HSlider1(normal) => {
                self.h_slider1_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "HSlider1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::VSlider1(normal) => {
                self.v_slider1_param.value = normal;

                self.output_text = crate::info_text_f32(
                    "VSlider1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::ModKnob1(normal) => {
                self.knob_auto1_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "ModKnob1",
                    self.float_range.unmap_to_value(normal),
                );

                let mod_value = self
                    .float_range_bipolar
                    .unmap_to_value(self.auto_input1_param.value);

                self.knob_auto1_mod_range.start = normal;
                self.knob_auto1_mod_range.end =
                    (self.knob_auto1_mod_range.start.as_f32() + mod_value)
                        .into();
            }
            Message::ModRangeInput1(normal) => {
                self.auto_input1_param.update(normal);

                let value = self.float_range_bipolar.unmap_to_value(normal);

                self.output_text =
                    crate::info_text_f32("ModRangeInput1", value);

                self.knob_auto1_mod_range.end =
                    (self.knob_auto1_param.value.as_f32() + value).into();
            }
            Message::ModKnob2(normal) => {
                self.knob_auto2_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "ModKnob2",
                    self.float_range.unmap_to_value(normal),
                );

                let mod_value = self
                    .float_range_bipolar
                    .unmap_to_value(self.auto_input2_param.value);

                self.knob_auto2_mod_range.start = normal;
                self.knob_auto2_mod_range.end =
                    (self.knob_auto2_mod_range.start.as_f32() + mod_value)
                        .into();
            }
            Message::ModRangeInput2(normal) => {
                self.auto_input2_param.update(normal);

                let value = self.float_range_bipolar.unmap_to_value(normal);

                self.output_text =
                    crate::info_text_f32("ModRangeInput1", value);

                self.knob_auto2_mod_range.end =
                    (self.knob_auto2_param.value.as_f32() + value).into();
            }
            Message::ToggleModRange(toggle) => {
                self.mod_range_toggle_value = toggle;

                self.mod_range_1.filled_visible = toggle;
                self.knob_auto1_mod_range.filled_visible = toggle;
                self.knob_auto2_mod_range.filled_visible = toggle;
            }
        }
    }

    pub fn view(&self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_start = Knob::new(self.knob_start_param, Message::RangeStart);

        let knob_end = Knob::new(self.knob_end_param, Message::RangeEnd);

        let knob1 = Knob::new(self.knob1_param, Message::Knob1)
            .mod_range(&self.mod_range_1)
            .style(style::knob::CustomArc);

        let h_slider1 = HSlider::new(self.h_slider1_param, Message::HSlider1)
            .mod_range(&self.mod_range_1)
            .style(style::h_slider::RectStyle);

        let v_slider1 = VSlider::new(self.v_slider1_param, Message::VSlider1)
            .width(Length::Units(30))
            .mod_range(&self.mod_range_1)
            .style(style::v_slider::RectStyle);

        let auto_input1 =
            ModRangeInput::new(self.auto_input1_param, Message::ModRangeInput1)
                .size(Length::from(10))
                .style(style::mod_range_input::CustomStyle);

        let knob_auto1 = Knob::new(self.knob_auto1_param, Message::ModKnob1)
            .mod_range(&self.knob_auto1_mod_range)
            .style(style::knob::CustomStyleCircle);

        let auto_input2 =
            ModRangeInput::new(self.auto_input2_param, Message::ModRangeInput2)
                .size(Length::from(15))
                .style(theme::ModRangeInput::Invisible);

        let knob_auto2 = Knob::new(self.knob_auto2_param, Message::ModKnob2)
            .mod_range(&self.knob_auto2_mod_range)
            .style(style::knob::CustomStyleCircle);

        // push the widgets into rows
        let knob_row = row![
            column![
                text("Range Start"),
                knob_start,
                text("Range End"),
                knob_end,
                checkbox(
                    "Show Modulation",
                    self.mod_range_toggle_value,
                    Message::ToggleModRange,
                ),
            ]
            .max_width(130)
            .spacing(10),
            container(
                column![knob1, h_slider1, v_slider1]
                    .max_width(130)
                    .align_items(Alignment::Center)
                    .spacing(20)
            )
            .max_height(250),
            column![
                column![
                    text("Custom Style with ModRangeInput"),
                    auto_input1,
                    knob_auto1,
                ]
                .width(Length::Fill)
                .spacing(14)
                .align_items(Alignment::Center),
                column![
                    text("Custom Style with invisible ModRangeInput",),
                    auto_input2,
                    knob_auto2,
                ]
                .width(Length::Fill)
                .spacing(0)
                .align_items(Alignment::Center)
            ]
            .width(Length::Fill)
            .spacing(10),
        ]
        .spacing(20);

        let content = column![knob_row, text(&self.output_text).size(16)]
            .spacing(20)
            .padding(20);

        StepContainer::<Self, _, _>::new("Modulation Ranges")
            .push(content)
            .into()
    }
}
