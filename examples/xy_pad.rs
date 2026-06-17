mod style;
mod util;

use iced::{
    application,
    widget::{column, row, text},
    Element, Length, Result, Size,
};
use iced_audio::{FloatRange, Normal, NormalParam, XYPad};
use util::info_text;

fn main() -> Result {
    application(
        XYPadExample::default,
        XYPadExample::update,
        XYPadExample::view,
    )
    .window_size(Size::new(600.0, 400.0))
    .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    Default(Normal, Normal),
    Custom(Normal, Normal),
    Knob(Normal),
}

pub struct XYPadExample {
    float_range: FloatRange,

    xy_pad_default_x_param: NormalParam,
    xy_pad_default_y_param: NormalParam,
    xy_pad_custom_x_param: NormalParam,
    xy_pad_custom_y_param: NormalParam,

    output_text_x: String,
    output_text_y: String,
}

impl Default for XYPadExample {
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

            output_text_x: String::new(),
            output_text_y: String::new(),
        }
    }
}

impl XYPadExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::Default(normal_x, normal_y) => {
                self.xy_pad_default_x_param.update(normal_x);
                self.xy_pad_default_y_param.update(normal_y);

                self.output_text_x = info_text::info_text_f32(
                    "XYPadDefaultX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = info_text::info_text_f32(
                    "XYPadDefaultY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
            Message::Custom(normal_x, normal_y) => {
                self.xy_pad_custom_x_param.update(normal_x);
                self.xy_pad_custom_y_param.update(normal_y);

                self.output_text_x = info_text::info_text_f32(
                    "XYPadCustomX",
                    self.float_range.unmap_to_value(normal_x),
                );
                self.output_text_y = info_text::info_text_f32(
                    "XYPadCustomY",
                    self.float_range.unmap_to_value(normal_y),
                );
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
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

        column![
            xy_pad_row,
            text(&self.output_text_x).size(16),
            text(&self.output_text_y).size(16),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
