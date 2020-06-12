use iced::{Column, Element, Length, Row, Text};

use iced_audio::{
    xy_pad, FloatRange, XYPad,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum XYPadsID {
    DefaultX,
    DefaultY,
    CustomX,
    CustomY,
}

#[derive(Debug, Clone)]
pub enum Message {
    XYPadMoved(XYPadsID),
}

pub struct XYPadStep {
    float_range: FloatRange,

    xy_pad_default_state: xy_pad::State<XYPadsID>,
    xy_pad_custom_state: xy_pad::State<XYPadsID>,

    output_text: String,
}

impl Default for XYPadStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();

        // create application

        Self {
            float_range,

            // initialize the state of the xy_pad widget
            xy_pad_default_state: xy_pad::State::new(
                float_range.create_param_default(XYPadsID::DefaultX),
                float_range.create_param_default(XYPadsID::DefaultY),
            ),

            xy_pad_custom_state: xy_pad::State::new(
                float_range.create_param_default(XYPadsID::CustomX),
                float_range.create_param_default(XYPadsID::CustomY),
            ),

            output_text: String::from("Move a widget"),
        }
    }
}

impl XYPadStep {
    pub fn title(&self) -> &str {
        "XYPads"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::XYPadMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    XYPadsID::DefaultX => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.xy_pad_default_state.param_x.normal,
                            ),
                        );
                    }
                    XYPadsID::DefaultY => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.xy_pad_default_state.param_y.normal,
                            ),
                        );
                    }
                    XYPadsID::CustomX => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.xy_pad_custom_state.param_x.normal,
                            ),
                        );
                    }
                    XYPadsID::CustomY => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.xy_pad_custom_state.param_y.normal,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default =
            XYPad::new(&mut self.xy_pad_default_state, Message::XYPadMoved);

        let xy_pad_custom =
            XYPad::new(&mut self.xy_pad_custom_state, Message::XYPadMoved)
                .style(style::XYPadCustomStyle);

        // push the widgets into rows
        let xy_pad_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Default Style"))
                    .push(xy_pad_default),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Custom Style"))
                    .push(xy_pad_custom),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(xy_pad_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("XYPads").push(content).into()
    }
}
