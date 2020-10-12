use iced::{Align, Checkbox, Column, Element, Length, Row, Text};

use iced_audio::{
    h_slider, knob, mod_range_input, v_slider, FloatRange, HSlider, Knob,
    ModRangeInput, ModulationRange, Normal, VSlider,
};

use crate::{style, Step};

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

    knob_start_state: knob::State,
    knob_end_state: knob::State,

    knob1_state: knob::State,
    h_slider1_state: h_slider::State,
    v_slider1_state: v_slider::State,
    mod_range_1: ModulationRange,

    auto_input1_state: mod_range_input::State,
    knob_auto1_state: knob::State,
    knob_auto1_mod_range: ModulationRange,

    auto_input2_state: mod_range_input::State,
    knob_auto2_state: knob::State,
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
            knob_start_state: knob::State::new(
                float_range.default_normal_param(),
            ),
            knob_end_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            mod_range_1: ModulationRange::default(),

            knob1_state: knob::State::new(float_range.default_normal_param()),

            h_slider1_state: h_slider::State::new(
                float_range.default_normal_param(),
            ),

            v_slider1_state: v_slider::State::new(
                float_range.default_normal_param(),
            ),

            auto_input1_state: mod_range_input::State::new(
                float_range_bipolar.default_normal_param(),
            ),

            knob_auto1_state: knob::State::new(
                float_range.default_normal_param(),
            ),
            auto_input2_state: mod_range_input::State::new(
                float_range_bipolar.default_normal_param(),
            ),
            knob_auto1_mod_range: ModulationRange::default(),

            knob_auto2_state: knob::State::new(
                float_range.default_normal_param(),
            ),
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
                self.output_text = crate::info_text_f32(
                    "RangeStart",
                    self.float_range.unmap_to_value(normal),
                );

                self.mod_range_1.start = normal;
            }
            Message::RangeEnd(normal) => {
                self.output_text = crate::info_text_f32(
                    "RangeEnd",
                    self.float_range.unmap_to_value(normal),
                );

                self.mod_range_1.end = normal;
            }
            Message::Knob1(normal) => {
                self.output_text = crate::info_text_f32(
                    "Knob1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::HSlider1(normal) => {
                self.output_text = crate::info_text_f32(
                    "HSlider1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::VSlider1(normal) => {
                self.output_text = crate::info_text_f32(
                    "VSlider1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::ModKnob1(normal) => {
                self.output_text = crate::info_text_f32(
                    "ModKnob1",
                    self.float_range.unmap_to_value(normal),
                );

                let mod_value = self
                    .float_range_bipolar
                    .unmap_to_value(self.auto_input1_state.normal_param.value);

                self.knob_auto1_mod_range.start = normal;
                self.knob_auto1_mod_range.end =
                    (self.knob_auto1_mod_range.start.as_f32() + mod_value)
                        .into();
            }
            Message::ModRangeInput1(normal) => {
                let value = self.float_range_bipolar.unmap_to_value(normal);

                self.output_text =
                    crate::info_text_f32("ModRangeInput1", value);

                self.knob_auto1_mod_range.end =
                    (self.knob_auto1_state.normal_param.value.as_f32() + value)
                        .into();
            }
            Message::ModKnob2(normal) => {
                self.output_text = crate::info_text_f32(
                    "ModKnob2",
                    self.float_range.unmap_to_value(normal),
                );

                let mod_value = self
                    .float_range_bipolar
                    .unmap_to_value(self.auto_input2_state.normal_param.value);

                self.knob_auto2_mod_range.start = normal;
                self.knob_auto2_mod_range.end =
                    (self.knob_auto2_mod_range.start.as_f32() + mod_value)
                        .into();
            }
            Message::ModRangeInput2(normal) => {
                let value = self.float_range_bipolar.unmap_to_value(normal);

                self.output_text =
                    crate::info_text_f32("ModRangeInput1", value);

                self.knob_auto2_mod_range.end =
                    (self.knob_auto2_state.normal_param.value.as_f32() + value)
                        .into();
            }
            Message::ToggleModRange(toggle) => {
                self.mod_range_toggle_value = toggle;

                self.mod_range_1.filled_visible = toggle;
                self.knob_auto1_mod_range.filled_visible = toggle;
                self.knob_auto2_mod_range.filled_visible = toggle;
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_start =
            Knob::new(&mut self.knob_start_state, Message::RangeStart);

        let knob_end = Knob::new(&mut self.knob_end_state, Message::RangeEnd);

        let knob1 = Knob::new(&mut self.knob1_state, Message::Knob1)
            .mod_range(&self.mod_range_1)
            .style(style::knob::CustomArc);

        let h_slider1 =
            HSlider::new(&mut self.h_slider1_state, Message::HSlider1)
                .mod_range(&self.mod_range_1)
                .style(style::h_slider::RectStyle);

        let v_slider1 =
            VSlider::new(&mut self.v_slider1_state, Message::VSlider1)
                .width(Length::from(Length::Units(30)))
                .mod_range(&self.mod_range_1)
                .style(style::v_slider::RectStyle);

        let auto_input1 = ModRangeInput::new(
            &mut self.auto_input1_state,
            Message::ModRangeInput1,
        )
        .size(Length::from(10))
        .style(style::mod_range_input::CustomStyle);

        let knob_auto1 =
            Knob::new(&mut self.knob_auto1_state, Message::ModKnob1)
                .mod_range(&self.knob_auto1_mod_range)
                .style(style::knob::CustomStyleCircle);

        let auto_input2 = ModRangeInput::new(
            &mut self.auto_input2_state,
            Message::ModRangeInput2,
        )
        .size(Length::from(15))
        .style(mod_range_input::DefaultInvisible);

        let knob_auto2 =
            Knob::new(&mut self.knob_auto2_state, Message::ModKnob2)
                .mod_range(&self.knob_auto2_mod_range)
                .style(style::knob::CustomStyleCircle);

        // push the widgets into rows
        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .max_width(130)
                    .spacing(10)
                    .push(Text::new("Range Start"))
                    .push(knob_start)
                    .push(Text::new("Range End"))
                    .push(knob_end)
                    .push(Checkbox::new(
                        self.mod_range_toggle_value,
                        "Show Modulation",
                        Message::ToggleModRange,
                    )),
            )
            .push(
                Column::new()
                    .max_width(130)
                    .max_height(250)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(knob1)
                    .push(h_slider1)
                    .push(v_slider1),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .spacing(14)
                            .align_items(Align::Center)
                            .push(Text::new("Custom Style with ModRangeInput"))
                            .push(auto_input1)
                            .push(knob_auto1),
                    )
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .spacing(0)
                            .align_items(Align::Center)
                            .push(Text::new(
                                "Custom Style with invisible ModRangeInput",
                            ))
                            .push(auto_input2)
                            .push(knob_auto2),
                    ),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Modulation Ranges").push(content).into()
    }
}
