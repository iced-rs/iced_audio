mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, row, text},
};
use iced_audio::{FloatRange, Gesture, NormalParam, Ramp, ramp::RampDirection};

use crate::util::info_text::info_text_f32;

fn main() -> Result {
    application(RampExample::default, RampExample::update, RampExample::view)
        .window_size(Size::new(600.0, 400.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    DefaultUp(Gesture),
    DefaultDown(Gesture),
    CustomUp(Gesture),
    CustomDown(Gesture),
}

pub struct RampExample {
    ramp_default_up_param: NormalParam,
    ramp_default_down_param: NormalParam,
    ramp_custom_up_param: NormalParam,
    ramp_custom_down_param: NormalParam,

    output_text: String,
}

impl Default for RampExample {
    fn default() -> Self {
        Self {
            // initialize the state of the ramp widget
            ramp_default_up_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            ramp_default_down_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            ramp_custom_up_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            ramp_custom_down_param: FloatRange::NORMAL_BIPOLAR.default_param(),

            output_text: String::new(),
        }
    }
}

impl RampExample {
    fn update(&mut self, message: Message) {
        dbg!(&message);

        match message {
            Message::DefaultUp(Gesture::Gesturing(normal)) => {
                self.ramp_default_up_param.set(normal);
                self.output_text = info_text_f32("DefaultUp", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::DefaultDown(Gesture::Gesturing(normal)) => {
                self.ramp_default_down_param.set(normal);
                self.output_text =
                    info_text_f32("DefaultDown", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::CustomUp(Gesture::Gesturing(normal)) => {
                self.ramp_custom_up_param.set(normal);
                self.output_text = info_text_f32("CutomUp", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::CustomDown(Gesture::Gesturing(normal)) => {
                self.ramp_custom_down_param.set(normal);
                self.output_text = info_text_f32("CustomDown", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the Ramp widgets, passing in the value of
        // the corresponding parameter

        let ramp_default_up =
            Ramp::new(self.ramp_default_up_param, RampDirection::Up).on_gesture(Message::DefaultUp);

        let ramp_default_down = Ramp::new(self.ramp_default_down_param, RampDirection::Down)
            .on_gesture(Message::DefaultDown);

        let ramp_custom_up = Ramp::new(self.ramp_custom_up_param, RampDirection::Up)
            .on_gesture(Message::CustomUp)
            .style(style::ramp::custom_style);

        let ramp_custom_down = Ramp::new(self.ramp_custom_down_param, RampDirection::Down)
            .on_gesture(Message::CustomDown)
            .style(style::ramp::custom_style);

        // push the widgets into rows
        let ramp_row = row![
            column![
                text("Default Style Up"),
                ramp_default_up,
                text("Default Style Down"),
                ramp_default_down,
            ]
            .width(Length::Fill)
            .spacing(10),
            column![
                text("Custom Style Up"),
                ramp_custom_up,
                text("Custom Style Down"),
                ramp_custom_down,
            ]
            .width(Length::Fill)
            .spacing(10),
        ]
        .spacing(20);

        column![ramp_row, text(&self.output_text).size(16),]
            .spacing(20)
            .padding(20)
            .into()
    }
}
