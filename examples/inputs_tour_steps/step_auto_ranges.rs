use iced::{Column, Element, Length, Row, Text, Align};

use iced_audio::{knob, AutomationRange, FloatRange, Knob, 
    auto_range_input, AutoRangeInput, h_slider, HSlider, v_slider, VSlider};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum AutoRangesID {
    RangeStart,
    RangeEnd,
    Knob1,
    HSlider1,
    VSlider1,
    AutoKnob1,
    AutoKnob2,
    AutoRangeInput1,
    AutoRangeInput2,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobMoved(AutoRangesID),
}

pub struct AutoRanges {
    float_range: FloatRange,
    float_range_bipolar: FloatRange,

    knob_start_state: knob::State<AutoRangesID>,
    knob_end_state: knob::State<AutoRangesID>,

    knob1_state: knob::State<AutoRangesID>,
    h_slider1_state: h_slider::State<AutoRangesID>,
    v_slider1_state: v_slider::State<AutoRangesID>,

    auto_input1_state: auto_range_input::State<AutoRangesID>,
    knob_auto1_state: knob::State<AutoRangesID>,

    auto_input2_state: auto_range_input::State<AutoRangesID>,
    knob_auto2_state: knob::State<AutoRangesID>,

    output_text: String,
}

impl Default for AutoRanges {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default();
        let float_range_bipolar = FloatRange::default_bipolar();

        let auto_range = AutomationRange::new(0.0.into(), 0.0.into());

        // create application

        Self {
            float_range,
            float_range_bipolar,

            // initialize the state of the Knob widget
            knob_start_state: knob::State::new(
                float_range.create_param_default(AutoRangesID::RangeStart),
            ),
            knob_end_state: knob::State::new(
                float_range.create_param_default(AutoRangesID::RangeEnd),
            ),

            knob1_state: knob::State::new(
                float_range.create_param_default(AutoRangesID::Knob1),
            )
            .automation_range(auto_range),

            h_slider1_state: h_slider::State::new(
                float_range.create_param_default(AutoRangesID::HSlider1),
            )
            .automation_range(auto_range),

            v_slider1_state: v_slider::State::new(
                float_range.create_param_default(AutoRangesID::VSlider1),
            )
            .automation_range(auto_range),

            auto_input1_state: auto_range_input::State::new(
                float_range_bipolar.create_param_default(AutoRangesID::AutoRangeInput1),
            ),
            
            knob_auto1_state: knob::State::new(
                float_range.create_param_default(AutoRangesID::AutoKnob1),
            )
            .automation_range(AutomationRange::default()),

            auto_input2_state: auto_range_input::State::new(
                float_range_bipolar.create_param_default(AutoRangesID::AutoRangeInput2),
            ),
            
            knob_auto2_state: knob::State::new(
                float_range.create_param_default(AutoRangesID::AutoKnob2),
            )
            .automation_range(AutomationRange::default()),

            output_text: String::from("Move a widget"),
        }
    }
}

impl AutoRanges {
    pub fn title(&self) -> &str {
        "Automation Ranges"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    AutoRangesID::RangeStart => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_start_state.param.normal),
                        );

                        if let Some(auto_range) =
                            &mut self.knob1_state.automation_range
                        {
                            auto_range.start =
                                self.knob_start_state.param.normal;
                        }
                        if let Some(auto_range) =
                            &mut self.h_slider1_state.automation_range
                        {
                            auto_range.start =
                                self.knob_start_state.param.normal;
                        }
                        if let Some(auto_range) =
                            &mut self.v_slider1_state.automation_range
                        {
                            auto_range.start =
                                self.knob_start_state.param.normal;
                        }
                    }
                    AutoRangesID::RangeEnd => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_end_state.param.normal),
                        );

                        if let Some(auto_range) =
                            &mut self.knob1_state.automation_range
                        {
                            auto_range.end = self.knob_end_state.param.normal;
                        }
                        if let Some(auto_range) =
                            &mut self.h_slider1_state.automation_range
                        {
                            auto_range.end = self.knob_end_state.param.normal;
                        }
                        if let Some(auto_range) =
                            &mut self.v_slider1_state.automation_range
                        {
                            auto_range.end = self.knob_end_state.param.normal;
                        }
                    }
                    AutoRangesID::Knob1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob1_state.param.normal),
                        );
                    }
                    AutoRangesID::HSlider1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.h_slider1_state.param.normal),
                        );
                    }
                    AutoRangesID::VSlider1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.v_slider1_state.param.normal),
                        );
                    }

                    AutoRangesID::AutoKnob1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_auto1_state.param.normal),
                        );

                        if let Some(auto_range) =
                            &mut self.knob_auto1_state.automation_range
                        {
                            let auto_value = self.float_range_bipolar
                                .to_value(self.auto_input1_state.param.normal);

                            auto_range.start = self.knob_auto1_state.param.normal;
                            auto_range.end = (auto_range.start.value() +
                                auto_value).into();
                        }
                    }
                    AutoRangesID::AutoRangeInput1 => {
                        let auto_value = self.float_range_bipolar
                            .to_value(self.auto_input1_state.param.normal);

                        self.output_text = crate::info_text_f32(
                            id,
                            auto_value,
                        );

                        if let Some(auto_range) =
                            &mut self.knob_auto1_state.automation_range
                        {
                            auto_range.end = (self.knob_auto1_state.param.normal.value() +
                                auto_value).into();
                        }
                    }

                    AutoRangesID::AutoKnob2 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_auto2_state.param.normal),
                        );

                        if let Some(auto_range) =
                            &mut self.knob_auto2_state.automation_range
                        {
                            let auto_value = self.float_range_bipolar
                                .to_value(self.auto_input2_state.param.normal);

                            auto_range.start = self.knob_auto2_state.param.normal;
                            auto_range.end = (auto_range.start.value() +
                                auto_value).into();
                        }
                    }
                    AutoRangesID::AutoRangeInput2 => {
                        let auto_value = self.float_range_bipolar
                            .to_value(self.auto_input2_state.param.normal);

                        self.output_text = crate::info_text_f32(
                            id,
                            auto_value,
                        );

                        if let Some(auto_range) =
                            &mut self.knob_auto2_state.automation_range
                        {
                            auto_range.end = (self.knob_auto2_state.param.normal.value() +
                                auto_value).into();
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

        let knob1 =
            Knob::new(&mut self.knob1_state, Message::KnobMoved)
                .style(style::KnobCustomArc);
        
        let auto_input1 =
            AutoRangeInput::new(&mut self.auto_input1_state, Message::KnobMoved)
                .size(Length::from(10))
                .style(style::AutoRangeInputCustom);
        
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
            AutoRangeInput::new(&mut self.auto_input2_state, Message::KnobMoved)
                .size(Length::from(15))
                .style(auto_range_input::DefaultInvisible);
        
        let knob_auto2 =
            Knob::new(&mut self.knob_auto2_state, Message::KnobMoved)
                .style(style::KnobCustomStyleCircle);

        // push the widgets into rows
        let knob_row = Row::new().spacing(20).push(
            Column::new()
                .max_width(125)
                .spacing(10)
                .push(Text::new("Range Start"))
                .push(knob_start)
                .push(Text::new("Range End"))
                .push(knob_end)
        )
        .push(Column::new()
            .max_width(125)
            .max_height(250)
            .align_items(Align::Center)
            .spacing(20)
            .push(knob1)
            .push(h_slider1)
            .push(v_slider1)
        )
        .push(Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(Column::new()
                .width(Length::Fill)
                .spacing(10)
                .align_items(Align::Center)
                .push(Text::new("Custom Style with AutoRangeInput"))
                .push(auto_input1)
                .push(knob_auto1),
            )
            .push(Column::new()
                .width(Length::Fill)
                .spacing(0)
                .align_items(Align::Center)
                .push(Text::new("Custom Style with invisible AutoRangeInput"))
                .push(auto_input2)
                .push(knob_auto2),
            )
        );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Automation Ranges").push(content).into()
    }
}
