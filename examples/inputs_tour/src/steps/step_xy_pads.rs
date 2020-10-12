use iced::{Column, Element, Length, Row, Text};

use iced_audio::{xy_pad, FloatRange, Normal, XYPad};

use crate::{style, Step};

#[derive(Debug, Clone)]
pub enum Message {
    Default(Normal, Normal),
    Custom(Normal, Normal),
}

pub struct XYPadStep {
    float_range: FloatRange,

    xy_pad_default_state: xy_pad::State,
    xy_pad_custom_state: xy_pad::State,

    output_text_x: String,
    output_text_y: String,
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
                float_range.default_normal_param(),
                float_range.default_normal_param(),
            ),

            xy_pad_custom_state: xy_pad::State::new(
                float_range.default_normal_param(),
                float_range.default_normal_param(),
            ),

            output_text_x: String::from("Move a widget"),
            output_text_y: String::from(""),
        }
    }
}

impl XYPadStep {
    pub fn title(&self) -> &str {
        "XYPads"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Default(normal_x, normal_y) => {
                self.output_text_x = crate::info_text_f32(
                    "XYPadDefaultX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = crate::info_text_f32(
                    "XYPadDefaultY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
            Message::Custom(normal_x, normal_y) => {
                self.output_text_x = crate::info_text_f32(
                    "XYPadCustomX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = crate::info_text_f32(
                    "XYPadCustomY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default =
            XYPad::new(&mut self.xy_pad_default_state, Message::Default);

        let xy_pad_custom =
            XYPad::new(&mut self.xy_pad_custom_state, Message::Custom)
                .style(style::xy_pad::CustomStyle);

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
            .push(Text::new(&self.output_text_x).size(16))
            .push(Text::new(&self.output_text_y).size(16));

        Step::container("XYPads").push(content).into()
    }
}
