use iced::{Column, Element, Length, Row, Text};

use iced_audio::{ramp, FloatParam, Normal, Ramp};

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
    RampsChanged((RampsID, Normal)),
}

pub struct RampStep {
    ramp_default_up_param: FloatParam<RampsID>,
    ramp_default_up_state: ramp::State,
    ramp_default_up_label: String,

    ramp_default_down_param: FloatParam<RampsID>,
    ramp_default_down_state: ramp::State,
    ramp_default_down_label: String,

    ramp_custom_up_param: FloatParam<RampsID>,
    ramp_custom_up_state: ramp::State,
    ramp_custom_up_label: String,

    ramp_custom_down_param: FloatParam<RampsID>,
    ramp_custom_down_state: ramp::State,
    ramp_custom_down_label: String,

    output_text: String,
}

impl Default for RampStep {
    fn default() -> Self {
        // initalize parameters

        let ramp_default_up_param =
            FloatParam::<RampsID>::new(RampsID::DefaultUp, -1.0, 1.0, 0.0, 0.0);

        let ramp_default_down_param = FloatParam::<RampsID>::new(
            RampsID::DefaultDown,
            -1.0,
            1.0,
            0.0,
            0.0,
        );

        let ramp_custom_up_param =
            FloatParam::<RampsID>::new(RampsID::CustomUp, -1.0, 1.0, 0.0, 0.0);

        let ramp_custom_down_param = FloatParam::<RampsID>::new(
            RampsID::CustomDown,
            -1.0,
            1.0,
            0.0,
            0.0,
        );

        // create application

        Self {
            // add the parameter
            ramp_default_up_param,
            // initialize the state of the Ramp widget
            ramp_default_up_state: ramp::State::new(&ramp_default_up_param),
            // initialize the label above the Ramp widget
            ramp_default_up_label: String::from("Default Style Up"),

            ramp_default_down_param,
            ramp_default_down_state: ramp::State::new(&ramp_default_down_param),
            ramp_default_down_label: String::from("Default Style Down"),

            ramp_custom_up_param,
            ramp_custom_up_state: ramp::State::new(&ramp_custom_up_param),
            ramp_custom_up_label: String::from("Custom Style Up"),

            ramp_custom_down_param,
            ramp_custom_down_state: ramp::State::new(&ramp_custom_down_param),
            ramp_custom_down_label: String::from("Custom Style Down"),

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
            Message::RampsChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // Ramp widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    RampsID::DefaultUp => {
                        self.ramp_default_up_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.ramp_default_up_param.value(),
                        );
                    }
                    RampsID::DefaultDown => {
                        self.ramp_default_down_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.ramp_default_down_param.value(),
                        );
                    }
                    RampsID::CustomUp => {
                        self.ramp_custom_up_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.ramp_custom_up_param.value(),
                        );
                    }
                    RampsID::CustomDown => {
                        self.ramp_custom_down_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.ramp_custom_down_param.value(),
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
            &self.ramp_default_up_param,
            Message::RampsChanged,
            ramp::RampDirection::Up,
        );

        let ramp_default_down = Ramp::new(
            &mut self.ramp_default_down_state,
            &self.ramp_default_down_param,
            Message::RampsChanged,
            ramp::RampDirection::Down,
        );

        let ramp_custom_up = Ramp::new(
            &mut self.ramp_custom_up_state,
            &self.ramp_custom_up_param,
            Message::RampsChanged,
            ramp::RampDirection::Up,
        )
        .style(style::RampCustomStyle);

        let ramp_custom_down = Ramp::new(
            &mut self.ramp_custom_down_state,
            &self.ramp_custom_down_param,
            Message::RampsChanged,
            ramp::RampDirection::Down,
        )
        .style(style::RampCustomStyle);

        // push the ramps into rows

        let ramp_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new(&self.ramp_default_up_label))
                    .push(ramp_default_up)
                    .push(Text::new(&self.ramp_default_down_label))
                    .push(ramp_default_down),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new(&self.ramp_custom_up_label))
                    .push(ramp_custom_up)
                    .push(Text::new(&self.ramp_custom_down_label))
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
