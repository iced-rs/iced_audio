mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, row, text},
};
use iced_audio::{FloatRange, Gesture, Normal, NormalParam, XYPad};

use util::info_text::info_text_f32;

use crate::style::xy_pad::CustomStyle;

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
    DefaultX(Gesture),
    DefaultY(Gesture),
    CustomX(Gesture),
    CustomY(Gesture),
    Knob(Normal),
}

pub struct XYPadExample {
    xy_pad_default_x_param: NormalParam,
    xy_pad_default_y_param: NormalParam,
    xy_pad_custom_x_param: NormalParam,
    xy_pad_custom_y_param: NormalParam,

    output_text_x: String,
    output_text_y: String,
}

impl Default for XYPadExample {
    fn default() -> Self {
        Self {
            // initialize the state of the xy_pad widget
            xy_pad_default_x_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            xy_pad_default_y_param: FloatRange::NORMAL_BIPOLAR.default_param(),

            xy_pad_custom_x_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            xy_pad_custom_y_param: FloatRange::NORMAL_BIPOLAR.default_param(),

            output_text_x: String::new(),
            output_text_y: String::new(),
        }
    }
}

impl XYPadExample {
    fn update(&mut self, message: Message) {
        dbg!(&message);

        match message {
            Message::DefaultX(Gesture::Gesturing(normal)) => {
                self.xy_pad_default_x_param.set(normal);
                self.output_text_x =
                    info_text_f32("XYPadDefaultX", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::DefaultY(Gesture::Gesturing(normal)) => {
                self.xy_pad_default_y_param.set(normal);
                self.output_text_y =
                    info_text_f32("XYPadDefaultY", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::CustomX(Gesture::Gesturing(normal)) => {
                self.xy_pad_custom_x_param.set(normal);
                self.output_text_x =
                    info_text_f32("XYPadCustomX", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::CustomY(Gesture::Gesturing(normal)) => {
                self.xy_pad_custom_y_param.set(normal);
                self.output_text_y =
                    info_text_f32("XYPadCustomY", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default = XYPad::new(
            Some(self.xy_pad_default_x_param),
            Some(self.xy_pad_default_y_param),
        )
        .on_gesture_x(Some(Message::DefaultX))
        .on_gesture_y(Some(Message::DefaultY));

        let xy_pad_custom = XYPad::new(
            Some(self.xy_pad_custom_x_param),
            Some(self.xy_pad_custom_y_param),
        )
        .on_gesture_x(Some(Message::CustomX))
        .on_gesture_y(Some(Message::CustomY))
        .style(CustomStyle);

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
