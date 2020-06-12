use iced::{Column, Element, Length, Row, Text};

use iced_audio::{ramp, FloatRange, Ramp};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum RampsID {
    DefaultUp,
    DefaultDown,
    CustomUp,
    CustomDown,
}

#[derive(Debug, Clone)]
pub enum Message {
    RampMoved(RampsID),
}

pub struct RampStep {
    float_range: FloatRange,

    ramp_default_up_state: ramp::State<RampsID>,
    ramp_default_down_state: ramp::State<RampsID>,
    ramp_custom_up_state: ramp::State<RampsID>,
    ramp_custom_down_state: ramp::State<RampsID>,

    output_text: String,
}

impl Default for RampStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();

        // create application

        Self {
            float_range,

            // initialize the state of the ramp widget
            ramp_default_up_state: ramp::State::new(
                float_range.create_param_default(RampsID::DefaultUp),
            ),

            ramp_default_down_state: ramp::State::new(
                float_range.create_param_default(RampsID::DefaultDown),
            ),

            ramp_custom_up_state: ramp::State::new(
                float_range.create_param_default(RampsID::CustomUp),
            ),

            ramp_custom_down_state: ramp::State::new(
                float_range.create_param_default(RampsID::CustomDown),
            ),

            output_text: String::from("Move a widget"),
        }
    }
}

impl RampStep {
    pub fn title(&self) -> &str {
        "Ramps"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RampMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    RampsID::DefaultUp => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.ramp_default_up_state.param.normal,
                            ),
                        );
                    }
                    RampsID::DefaultDown => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.ramp_default_down_state.param.normal,
                            ),
                        );
                    }
                    RampsID::CustomUp => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.ramp_custom_up_state.param.normal,
                            ),
                        );
                    }
                    RampsID::CustomDown => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.ramp_custom_down_state.param.normal,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Ramp widgets, passing in the value of
        // the corresponding parameter

        let ramp_default_up = Ramp::new(
            &mut self.ramp_default_up_state,
            Message::RampMoved,
            ramp::RampDirection::Up,
        );

        let ramp_default_down = Ramp::new(
            &mut self.ramp_default_down_state,
            Message::RampMoved,
            ramp::RampDirection::Down,
        );

        let ramp_custom_up = Ramp::new(
            &mut self.ramp_custom_up_state,
            Message::RampMoved,
            ramp::RampDirection::Up,
        )
        .style(style::RampCustomStyle);

        let ramp_custom_down = Ramp::new(
            &mut self.ramp_custom_down_state,
            Message::RampMoved,
            ramp::RampDirection::Down,
        )
        .style(style::RampCustomStyle);

        // push the widgets into rows
        let ramp_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Default Style Up"))
                    .push(ramp_default_up)
                    .push(Text::new("Default Style Down"))
                    .push(ramp_default_down),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Custom Style Up"))
                    .push(ramp_custom_up)
                    .push(Text::new("Custom Style Down"))
                    .push(ramp_custom_down),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(ramp_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Ramps").push(content).into()
    }
}
