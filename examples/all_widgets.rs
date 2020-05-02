extern crate iced;

use iced::{
    Column, Container, Element, Length, Sandbox, Text, Settings, Row,
};

use iced_audio::{h_slider, HSlider};
use iced_audio::{Normal, Param, FloatParam, IntParam, LogDBParam,
    OctaveParam
};

pub fn main() {
    AllWidgets::run(Settings::default());
}

struct AllWidgets {
    h_slider_float_param: FloatParam,
    h_slider_float_state: h_slider::State,
    h_slider_float_label: String,
    h_slider_float_text: String,

    h_slider_int_param: IntParam,
    h_slider_int_state: h_slider::State,
    h_slider_int_label: String,
    h_slider_int_text: String,

    h_slider_log_param: LogDBParam,
    h_slider_log_state: h_slider::State,
    h_slider_log_label: String,
    h_slider_log_text: String,

    h_slider_oct_param: OctaveParam,
    h_slider_oct_state: h_slider::State,
    h_slider_oct_label: String,
    h_slider_oct_text: String,
}

impl Default for AllWidgets {
    fn default() -> Self {
        let h_slider_float_param =
            FloatParam::new(0, -1.0, 1.0,
                              0.0, 0.0);

        let h_slider_int_param =
            IntParam::new(1, 0, 5,
                              0, 2);

        let h_slider_log_param =
            LogDBParam::new(2, -12.0, 12.0,
                              0.0, 0.0,
                              0.5.into());

        let h_slider_oct_param =
            OctaveParam::new(3, 20.0, 20_480.0,
                              1000.0, 1000.0);

        Self {
            h_slider_float_param,
            h_slider_float_state: h_slider::State::new(
                &h_slider_float_param
            ),
            h_slider_float_label: String::from("HSlider Float Range"),
            h_slider_float_text: info_text_f32(
                h_slider_float_param.id(),
                h_slider_float_param.value()),

            h_slider_int_param,
            h_slider_int_state: h_slider::State::new(
                &h_slider_int_param
            ),
            h_slider_int_label: String::from("HSlider Int Range"),
            h_slider_int_text: info_text_i32(
                h_slider_int_param.id(),
                h_slider_int_param.value()),
            
            h_slider_log_param,
            h_slider_log_state: h_slider::State::new(
                &h_slider_log_param
            ),
            h_slider_log_label: String::from("HSlider Log dB Range"),
            h_slider_log_text: info_text_db(
                h_slider_log_param.id(),
                h_slider_log_param.value()),
            
            h_slider_oct_param,
            h_slider_oct_state: h_slider::State::new(
                &h_slider_oct_param
            ),
            h_slider_oct_label: String::from("HSlider Octave Freq Range"),
            h_slider_oct_text: info_text_octave(
                h_slider_oct_param.id(),
                h_slider_oct_param.value()),
        }
    }
}

fn info_text_f32(id: u32, value: f32) -> String {
    format!("ID {}  |  {:.3}", id, value)
}

fn info_text_i32(id: u32, value: i32) -> String {
    format!("ID {}  |  {}", id, value)
}

fn info_text_db(id: u32, value: f32) -> String {
    format!("ID {}  |  {:.3} dB", id, value)
}

fn info_text_octave(id: u32, value: f32) -> String {
    if value < 1000.0 {
        format!("ID {}  |  {:.2} Hz", id, value)
    } else {
        format!("ID {}  |  {:.2} kHz", id, value / 1000.0)
    }
}

#[derive(Debug, Clone)]
enum Message {
    HSliderChanged((u32, Normal)),
}

impl Sandbox for AllWidgets {
    type Message = Message;

    fn new() -> Self {
        AllWidgets::default()
    }

    fn title(&self) -> String {
        String::from("All Widgets - Iced Audio")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HSliderChanged((id, normal)) => {
                match id {
                    0 => {
                        self.h_slider_float_param.set_from_normal(normal);
                        self.h_slider_float_text = info_text_f32(id,
                            self.h_slider_float_param.value());
                    },
                    1 => {
                        self.h_slider_int_param.set_from_normal(normal);
                        self.h_slider_int_text = info_text_i32(id,
                            self.h_slider_int_param.value());
                    },
                    2 => {
                        self.h_slider_log_param.set_from_normal(normal);
                        self.h_slider_log_text = info_text_db(id,
                            self.h_slider_log_param.value());
                    },
                    3 => {
                        self.h_slider_oct_param.set_from_normal(normal);
                        self.h_slider_oct_text = info_text_octave(id,
                            self.h_slider_oct_param.value());
                    },
                    _ => (),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let h_slider_float = HSlider::new(
            &mut self.h_slider_float_state,
            &self.h_slider_float_param,
            Message::HSliderChanged,
        );

        let h_slider_int = HSlider::new(
            &mut self.h_slider_int_state,
            &self.h_slider_int_param,
            Message::HSliderChanged,
        );

        let h_slider_log = HSlider::new(
            &mut self.h_slider_log_state,
            &self.h_slider_log_param,
            Message::HSliderChanged,
        );

        let h_slider_oct = HSlider::new(
            &mut self.h_slider_oct_state,
            &self.h_slider_oct_param,
            Message::HSliderChanged,
        );

        let h_slider_row = Row::new()
            .spacing(20)

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_float_label))
                .push(h_slider_float)
                .push(Text::new(&self.h_slider_float_text).size(16))

                .push(Text::new(&self.h_slider_log_label))
                .push(h_slider_log)
                .push(Text::new(&self.h_slider_log_text).size(16))
            )

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_int_label))
                .push(h_slider_int)
                .push(Text::new(&self.h_slider_int_text).size(16))

                .push(Text::new(&self.h_slider_oct_label))
                .push(h_slider_oct)
                .push(Text::new(&self.h_slider_oct_text).size(16))
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Text::new("Horizontal Sliders (HSlider)").size(42))
            .push(Text::new("Hold down Ctrl for fine adjustments. Double-click to reset to the default value.").size(16))
            .push(h_slider_row);
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}