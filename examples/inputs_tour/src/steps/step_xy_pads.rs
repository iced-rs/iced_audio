use iced::widget::{column, row, text};
use iced::{Element, Length};

use iced_audio::{FloatRange, Normal, NormalParam, XYPad};

use crate::{style, StepContainer};

#[derive(Debug, Clone)]
pub enum Message {
    Default(Normal, Normal),
    Custom(Normal, Normal),
}

pub struct XYPadStep {
    float_range: FloatRange,

    xy_pad_default_x_param: NormalParam,
    xy_pad_default_y_param: NormalParam,
    xy_pad_custom_x_param: NormalParam,
    xy_pad_custom_y_param: NormalParam,

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
            xy_pad_default_x_param: float_range.default_normal_param(),
            xy_pad_default_y_param: float_range.default_normal_param(),

            xy_pad_custom_x_param: float_range.default_normal_param(),
            xy_pad_custom_y_param: float_range.default_normal_param(),

            output_text_x: String::from("Move a widget"),
            output_text_y: String::from(" "),
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
                self.xy_pad_default_x_param.update(normal_x);
                self.xy_pad_default_y_param.update(normal_y);

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
                self.xy_pad_custom_x_param.update(normal_x);
                self.xy_pad_custom_y_param.update(normal_y);

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

    pub fn view(&self, _debug: bool) -> Element<Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default = XYPad::new(
            self.xy_pad_default_x_param,
            self.xy_pad_default_y_param,
            Message::Default,
        );

        let xy_pad_custom = XYPad::new(
            self.xy_pad_custom_x_param,
            self.xy_pad_custom_y_param,
            Message::Custom,
        )
        .style(style::xy_pad::CustomStyle);

        // push the widgets into rows
        let xy_pad_row = row![
            column![text("Default Style"), xy_pad_default,]
                .width(Length::Fill)
                .spacing(10),
            column![text("Custom Style"), xy_pad_custom,]
                .width(Length::Fill)
                .spacing(10),
        ]
        .spacing(20);

        let content = column![
            xy_pad_row,
            text(&self.output_text_x).size(16),
            text(&self.output_text_y).size(16),
        ]
        .spacing(20)
        .padding(20);

        StepContainer::<Self, _, _>::new("XYPads")
            .push(content)
            .into()
    }
}
