use iced::{Column, Element, Length, Row, Text};

use iced_audio::{ramp, FloatRange, Normal, Ramp};

use crate::{style, Step};

#[derive(Debug, Clone)]
pub enum Message {
    DefaultUp(Normal),
    DefaultDown(Normal),
    CustomUp(Normal),
    CustomDown(Normal),
}

pub struct RampStep {
    float_range: FloatRange,

    ramp_default_up_state: ramp::State,
    ramp_default_down_state: ramp::State,
    ramp_custom_up_state: ramp::State,
    ramp_custom_down_state: ramp::State,

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
                float_range.default_normal_param(),
            ),

            ramp_default_down_state: ramp::State::new(
                float_range.default_normal_param(),
            ),

            ramp_custom_up_state: ramp::State::new(
                float_range.default_normal_param(),
            ),

            ramp_custom_down_state: ramp::State::new(
                float_range.default_normal_param(),
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
            Message::DefaultUp(normal) => {
                self.output_text = crate::info_text_f32(
                    "DefaultUp",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::DefaultDown(normal) => {
                self.output_text = crate::info_text_f32(
                    "DefaultDown",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::CustomUp(normal) => {
                self.output_text = crate::info_text_f32(
                    "CutomUp",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::CustomDown(normal) => {
                self.output_text = crate::info_text_f32(
                    "CustomDown",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Ramp widgets, passing in the value of
        // the corresponding parameter

        let ramp_default_up = Ramp::new(
            &mut self.ramp_default_up_state,
            Message::DefaultUp,
            ramp::RampDirection::Up,
        );

        let ramp_default_down = Ramp::new(
            &mut self.ramp_default_down_state,
            Message::DefaultDown,
            ramp::RampDirection::Down,
        );

        let ramp_custom_up = Ramp::new(
            &mut self.ramp_custom_up_state,
            Message::CustomUp,
            ramp::RampDirection::Up,
        )
        .style(style::ramp::CustomStyle);

        let ramp_custom_down = Ramp::new(
            &mut self.ramp_custom_down_state,
            Message::CustomDown,
            ramp::RampDirection::Down,
        )
        .style(style::ramp::CustomStyle);

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
