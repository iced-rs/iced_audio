use iced::{Align, Column, Element, Length, Row, Text};

use iced_audio::{
    h_slider, knob, mod_range_input, v_slider, FloatRange, HSlider, Knob,
    ModRangeInput, ModulationRange, VSlider,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum ModRangesID {
    RangeStart,
    RangeEnd,
    Knob1,
    HSlider1,
    VSlider1,
    ModKnob1,
    ModKnob2,
    ModRangeInput1,
    ModRangeInput2,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobMoved(ModRangesID),
}

pub struct ModRanges {
    float_range: FloatRange,
    float_range_bipolar: FloatRange,

    knob_start_state: knob::State<ModRangesID>,
    knob_end_state: knob::State<ModRangesID>,

    knob1_state: knob::State<ModRangesID>,
    h_slider1_state: h_slider::State<ModRangesID>,
    v_slider1_state: v_slider::State<ModRangesID>,

    auto_input1_state: mod_range_input::State<ModRangesID>,
    knob_auto1_state: knob::State<ModRangesID>,

    auto_input2_state: mod_range_input::State<ModRangesID>,
    knob_auto2_state: knob::State<ModRangesID>,

    output_text: String,
}

impl Default for ModRanges {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default();
        let float_range_bipolar = FloatRange::default_bipolar();

        let mod_range = ModulationRange::new(0.0.into(), 0.0.into());

        // create application

        Self {
            float_range,
            float_range_bipolar,

            // initialize the state of the Knob widget
            knob_start_state: knob::State::new(
                float_range.create_param_default(ModRangesID::RangeStart),
            ),
            knob_end_state: knob::State::new(
                float_range.create_param_default(ModRangesID::RangeEnd),
            ),

            knob1_state: knob::State::new(
                float_range.create_param_default(ModRangesID::Knob1),
            )
            .modulation_range(mod_range),

            h_slider1_state: h_slider::State::new(
                float_range.create_param_default(ModRangesID::HSlider1),
            )
            .modulation_range(mod_range),

            v_slider1_state: v_slider::State::new(
                float_range.create_param_default(ModRangesID::VSlider1),
            )
            .modulation_range(mod_range),

            auto_input1_state: mod_range_input::State::new(
                float_range_bipolar
                    .create_param_default(ModRangesID::ModRangeInput1),
            ),

            knob_auto1_state: knob::State::new(
                float_range.create_param_default(ModRangesID::ModKnob1),
            )
            .modulation_range(ModulationRange::default()),

            auto_input2_state: mod_range_input::State::new(
                float_range_bipolar
                    .create_param_default(ModRangesID::ModRangeInput2),
            ),

            knob_auto2_state: knob::State::new(
                float_range.create_param_default(ModRangesID::ModKnob2),
            )
            .modulation_range(ModulationRange::default()),

            output_text: String::from("Move a widget"),
        }
    }
}

impl ModRanges {
    pub fn title(&self) -> &str {
        "Modulation Ranges"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    ModRangesID::RangeStart => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_start_state.param.normal),
                        );

                        if let Some(mod_range) =
                            &mut self.knob1_state.modulation_range
                        {
                            mod_range.start =
                                self.knob_start_state.param.normal;
                        }
                        if let Some(mod_range) =
                            &mut self.h_slider1_state.modulation_range
                        {
                            mod_range.start =
                                self.knob_start_state.param.normal;
                        }
                        if let Some(mod_range) =
                            &mut self.v_slider1_state.modulation_range
                        {
                            mod_range.start =
                                self.knob_start_state.param.normal;
                        }
                    }
                    ModRangesID::RangeEnd => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_end_state.param.normal),
                        );

                        if let Some(mod_range) =
                            &mut self.knob1_state.modulation_range
                        {
                            mod_range.end = self.knob_end_state.param.normal;
                        }
                        if let Some(mod_range) =
                            &mut self.h_slider1_state.modulation_range
                        {
                            mod_range.end = self.knob_end_state.param.normal;
                        }
                        if let Some(mod_range) =
                            &mut self.v_slider1_state.modulation_range
                        {
                            mod_range.end = self.knob_end_state.param.normal;
                        }
                    }
                    ModRangesID::Knob1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob1_state.param.normal),
                        );
                    }
                    ModRangesID::HSlider1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.h_slider1_state.param.normal),
                        );
                    }
                    ModRangesID::VSlider1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.v_slider1_state.param.normal),
                        );
                    }

                    ModRangesID::ModKnob1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_auto1_state.param.normal),
                        );

                        if let Some(mod_range) =
                            &mut self.knob_auto1_state.modulation_range
                        {
                            let auto_value = self
                                .float_range_bipolar
                                .to_value(self.auto_input1_state.param.normal);

                            mod_range.start =
                                self.knob_auto1_state.param.normal;
                            mod_range.end =
                                (mod_range.start.value() + auto_value).into();
                        }
                    }
                    ModRangesID::ModRangeInput1 => {
                        let auto_value = self
                            .float_range_bipolar
                            .to_value(self.auto_input1_state.param.normal);

                        self.output_text = crate::info_text_f32(id, auto_value);

                        if let Some(mod_range) =
                            &mut self.knob_auto1_state.modulation_range
                        {
                            mod_range.end =
                                (self.knob_auto1_state.param.normal.value()
                                    + auto_value)
                                    .into();
                        }
                    }

                    ModRangesID::ModKnob2 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_auto2_state.param.normal),
                        );

                        if let Some(mod_range) =
                            &mut self.knob_auto2_state.modulation_range
                        {
                            let auto_value = self
                                .float_range_bipolar
                                .to_value(self.auto_input2_state.param.normal);

                            mod_range.start =
                                self.knob_auto2_state.param.normal;
                            mod_range.end =
                                (mod_range.start.value() + auto_value).into();
                        }
                    }
                    ModRangesID::ModRangeInput2 => {
                        let auto_value = self
                            .float_range_bipolar
                            .to_value(self.auto_input2_state.param.normal);

                        self.output_text = crate::info_text_f32(id, auto_value);

                        if let Some(mod_range) =
                            &mut self.knob_auto2_state.modulation_range
                        {
                            mod_range.end =
                                (self.knob_auto2_state.param.normal.value()
                                    + auto_value)
                                    .into();
                        }
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_start =
            Knob::new(&mut self.knob_start_state, Message::KnobMoved);

        let knob_end = Knob::new(&mut self.knob_end_state, Message::KnobMoved);

        let knob1 = Knob::new(&mut self.knob1_state, Message::KnobMoved)
            .style(style::KnobCustomArc);

        let auto_input1 =
            ModRangeInput::new(&mut self.auto_input1_state, Message::KnobMoved)
                .size(Length::from(10))
                .style(style::ModRangeInputCustom);

        let knob_auto1 =
            Knob::new(&mut self.knob_auto1_state, Message::KnobMoved)
                .style(style::KnobCustomStyleCircle);

        let h_slider1 =
            HSlider::new(&mut self.h_slider1_state, Message::KnobMoved)
                .style(style::HSliderRectStyle);

        let v_slider1 =
            VSlider::new(&mut self.v_slider1_state, Message::KnobMoved)
                .width(Length::from(Length::Units(30)))
                .style(style::VSliderRectStyle);

        let auto_input2 =
            ModRangeInput::new(&mut self.auto_input2_state, Message::KnobMoved)
                .size(Length::from(15))
                .style(mod_range_input::DefaultInvisible);

        let knob_auto2 =
            Knob::new(&mut self.knob_auto2_state, Message::KnobMoved)
                .style(style::KnobCustomStyleCircle);

        // push the widgets into rows
        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .max_width(125)
                    .spacing(10)
                    .push(Text::new("Range Start"))
                    .push(knob_start)
                    .push(Text::new("Range End"))
                    .push(knob_end),
            )
            .push(
                Column::new()
                    .max_width(125)
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
                            .spacing(10)
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
