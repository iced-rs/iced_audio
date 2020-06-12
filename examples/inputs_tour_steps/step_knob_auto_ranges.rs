use iced::{Column, Element, Length, Row, Text};

use iced_audio::{
    knob, FloatRange, Knob, AutomationRange,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum KnobAutoRangesID {
    RangeStart,
    RangeEnd,
    Style1,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobMoved(KnobAutoRangesID),
}

pub struct KnobAutoRanges {
    float_range: FloatRange,

    knob_start_state: knob::State<KnobAutoRangesID>,
    knob_end_state: knob::State<KnobAutoRangesID>,

    knob_style1_state: knob::State<KnobAutoRangesID>,

    output_text: String,
}

impl Default for KnobAutoRanges {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default();

        let auto_range = AutomationRange::new(0.0.into(), 0.0.into());

        // create application

        Self {
            float_range,

            // initialize the state of the Knob widget
            knob_start_state: knob::State::new(
                float_range.create_param_default(KnobAutoRangesID::RangeStart),
            ),
            knob_end_state: knob::State::new(
                float_range.create_param_default(KnobAutoRangesID::RangeEnd),
            ),

            knob_style1_state: knob::State::new(
                float_range.create_param_default(KnobAutoRangesID::Style1)
            )
            .automation_range(auto_range),

            output_text: String::from("Move a widget"),
        }
    }
}

impl KnobAutoRanges {
    pub fn title(&self) -> &str {
        "Knob Automation Ranges"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    KnobAutoRangesID::RangeStart => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_start_state.param.normal),
                        );

                        if let Some(auto_range) = &mut self.knob_style1_state.automation_range {
                            auto_range.start = self.knob_start_state.param.normal;
                        }
                    }
                    KnobAutoRangesID::RangeEnd => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_end_state.param.normal),
                        );

                        if let Some(auto_range) = &mut self.knob_style1_state.automation_range {
                            auto_range.end = self.knob_end_state.param.normal;
                        }
                    }
                    KnobAutoRangesID::Style1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_style1_state.param.normal),
                        );
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

        let knob_end =
            Knob::new(&mut self.knob_end_state, Message::KnobMoved);

        let knob_style1 =
            Knob::new(&mut self.knob_style1_state, Message::KnobMoved)
                .style(style::KnobCustomArc);

        // push the widgets into rows
        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Range Start"))
                    .push(knob_start)
                    .push(Text::new("Range End"))
                    .push(knob_end)
                    .push(Text::new("Custom Style 1"))
                    .push(knob_style1)
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Knob Automation Ranges").push(content).into()
    }
}
