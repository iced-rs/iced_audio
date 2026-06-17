mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, row, text},
};
use iced_audio::{FloatRange, Normal, NormalParam, Ramp, ramp::RampDirection};
use util::info_text;

fn main() -> Result {
    application(RampExample::default, RampExample::update, RampExample::view)
        .window_size(Size::new(600.0, 400.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    DefaultUp(Normal),
    DefaultDown(Normal),
    CustomUp(Normal),
    CustomDown(Normal),
}

pub struct RampExample {
    float_range: FloatRange,

    ramp_default_up_param: NormalParam,
    ramp_default_down_param: NormalParam,
    ramp_custom_up_param: NormalParam,
    ramp_custom_down_param: NormalParam,

    output_text: String,
}

impl Default for RampExample {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();

        // create application

        Self {
            float_range,

            // initialize the state of the ramp widget
            ramp_default_up_param: float_range.default_normal_param(),
            ramp_default_down_param: float_range.default_normal_param(),
            ramp_custom_up_param: float_range.default_normal_param(),
            ramp_custom_down_param: float_range.default_normal_param(),

            output_text: String::new(),
        }
    }
}

impl RampExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::DefaultUp(normal) => {
                self.ramp_default_up_param.update(normal);

                self.output_text =
                    info_text::info_text_f32("DefaultUp", self.float_range.unmap_to_value(normal));
            }
            Message::DefaultDown(normal) => {
                self.ramp_default_down_param.update(normal);

                self.output_text = info_text::info_text_f32(
                    "DefaultDown",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::CustomUp(normal) => {
                self.ramp_custom_up_param.update(normal);

                self.output_text =
                    info_text::info_text_f32("CutomUp", self.float_range.unmap_to_value(normal));
            }
            Message::CustomDown(normal) => {
                self.ramp_custom_down_param.update(normal);

                self.output_text =
                    info_text::info_text_f32("CustomDown", self.float_range.unmap_to_value(normal));
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the Ramp widgets, passing in the value of
        // the corresponding parameter

        let ramp_default_up = Ramp::new(
            self.ramp_default_up_param,
            Message::DefaultUp,
            RampDirection::Up,
        );

        let ramp_default_down = Ramp::new(
            self.ramp_default_down_param,
            Message::DefaultDown,
            RampDirection::Down,
        );

        let ramp_custom_up = Ramp::new(
            self.ramp_custom_up_param,
            Message::CustomUp,
            RampDirection::Up,
        )
        .style(style::ramp::CustomStyle);

        let ramp_custom_down = Ramp::new(
            self.ramp_custom_down_param,
            Message::CustomDown,
            RampDirection::Down,
        )
        .style(style::ramp::CustomStyle);

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
