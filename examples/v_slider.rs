mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, container, row, text},
};
use iced_audio::{
    DBRange, FloatRange, FreqRange, Gesture, IntRange, NormalParam, VSlider, text_marks, tick_marks,
};

use crate::util::info_text::{info_text_db, info_text_f32, info_text_freq, info_text_i32};

const INT_RANGE: IntRange = IntRange::new(0, 5);

fn main() -> Result {
    application(
        VSliderExample::default,
        VSliderExample::update,
        VSliderExample::view,
    )
    .window_size(Size::new(600.0, 400.0))
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    Float(Gesture),
    Int(Gesture),
    DB(Gesture),
    Freq(Gesture),
    RectStyle(Gesture),
    RectBipolarStyle(Gesture),
    TextureStyle(Gesture),
}

pub struct VSliderExample {
    float_param: NormalParam,
    int_param: NormalParam,
    db_param: NormalParam,
    freq_param: NormalParam,
    rect_param: NormalParam,
    rect_bp_param: NormalParam,
    texture_param: NormalParam,

    v_slider_texture_handle: iced::widget::image::Handle,

    float_tick_marks: tick_marks::Group,
    int_tick_marks: tick_marks::Group,
    db_tick_marks: tick_marks::Group,
    freq_tick_marks: tick_marks::Group,

    float_text_marks: text_marks::Group,
    int_text_marks: text_marks::Group,
    db_text_marks: text_marks::Group,
    freq_text_marks: text_marks::Group,

    output_text: String,
}

impl Default for VSliderExample {
    fn default() -> Self {
        Self {
            // initialize the parameter of the VSlider widget
            float_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            int_param: INT_RANGE.default_param(),
            db_param: DBRange::NEG_12_TO_12.default_param(),
            freq_param: FreqRange::HZ_20_TO_20K.param(1000.0, 1000.0),
            rect_param: FloatRange::NORMAL.default_param(),
            rect_bp_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            texture_param: FloatRange::NORMAL_BIPOLAR.default_param(),

            v_slider_texture_handle: format!(
                "{}/examples/images/iced_v_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: tick_marks::Group::subdivided(1, 1, 1, Some(tick_marks::Tier::Two)),
            int_tick_marks: tick_marks::Group::evenly_spaced(6, tick_marks::Tier::Two),
            db_tick_marks: vec![
                (DBRange::NEG_12_TO_12.map_db(0.0), tick_marks::Tier::One),
                (DBRange::NEG_12_TO_12.map_db(1.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(3.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(6.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(9.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(12.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-1.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-3.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-6.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-9.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-12.0), tick_marks::Tier::Two),
            ]
            .into(),
            freq_tick_marks: vec![
                (
                    FreqRange::HZ_20_TO_20K.map_freq(20.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(50.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(100.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(200.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(400.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(1000.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(2000.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(5000.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(10000.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(20000.0),
                    tick_marks::Tier::Two,
                ),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center("-1", "+1", "0"),
            int_text_marks: text_marks::Group::evenly_spaced(&["A", "B", "C", "D", "E", "F"]),
            db_text_marks: text_marks::Group::min_max_and_center("-12", "+12", "0"),
            freq_text_marks: vec![
                (FreqRange::HZ_20_TO_20K.map_freq(100.0), "100"),
                (FreqRange::HZ_20_TO_20K.map_freq(1000.0), "1k"),
                (FreqRange::HZ_20_TO_20K.map_freq(10000.0), "10k"),
            ]
            .into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl VSliderExample {
    fn update(&mut self, message: Message) {
        dbg!(&message);

        match message {
            Message::Float(Gesture::Gesturing(normal)) => {
                self.float_param.set(normal);
                self.output_text =
                    info_text_f32("VSliderFloat", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::Int(Gesture::Gesturing(normal)) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.int_param.set(INT_RANGE.snap(normal));
                self.output_text = info_text_i32("VSliderInt", normal, &INT_RANGE);
            }
            Message::DB(Gesture::Gesturing(normal)) => {
                self.db_param.set(normal);
                self.output_text = info_text_db("VSliderDB", normal, &DBRange::NEG_12_TO_12);
            }
            Message::Freq(Gesture::Gesturing(normal)) => {
                self.freq_param.set(normal);
                self.output_text = info_text_freq("VSliderFreq", normal, &FreqRange::HZ_20_TO_20K);
            }
            Message::RectStyle(Gesture::Gesturing(normal)) => {
                self.rect_param.set(normal);
                self.output_text = info_text_f32("VSliderRect", normal, &FloatRange::NORMAL);
            }
            Message::RectBipolarStyle(Gesture::Gesturing(normal)) => {
                self.rect_bp_param.set(normal);
                self.output_text =
                    info_text_f32("VSliderBipolar", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::TextureStyle(Gesture::Gesturing(normal)) => {
                self.texture_param.set(normal);
                self.output_text =
                    info_text_f32("VSliderTexture", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the VSlider widgets, passing in the value of
        // the corresponding parameter

        let v_slider_float = VSlider::new(self.float_param)
            .on_gesture(Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let v_slider_int = VSlider::new(self.int_param)
            .on_gesture(Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let v_slider_db = VSlider::new(self.db_param)
            .on_gesture(Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let v_slider_freq = VSlider::new(self.freq_param)
            .on_gesture(Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let v_slider_rect = VSlider::new(self.rect_param)
            .on_gesture(Message::RectStyle)
            .width(Length::Fixed(24.0))
            .style(style::v_slider::RectStyle);

        let v_slider_rect_bp = VSlider::new(self.rect_bp_param)
            .on_gesture(Message::RectBipolarStyle)
            .width(Length::Fixed(24.0))
            .style(style::v_slider::RectBipolarStyle);

        let v_slider_texture = VSlider::new(self.texture_param)
            .on_gesture(Message::TextureStyle)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks)
            // the width of the texture
            .width(Length::Fixed(20.0))
            .style(style::v_slider::TextureStyle(
                // clone the handle to the loaded texture
                self.v_slider_texture_handle.clone(),
                // bounds of the texture, where the origin is in the center
                // of the image
                iced::Rectangle {
                    x: -20.0 / 2.0,
                    y: -38.0 / 2.0,
                    width: 20.0,
                    height: 38.0,
                },
            ));

        // push the widgets into rows
        let v_slider_row = container(
            row![
                column![
                    text("Float Range"),
                    v_slider_float,
                    text("Log DB Range"),
                    v_slider_db,
                ]
                .max_width(120)
                .height(Length::Fill)
                .width(Length::Fill)
                .spacing(10),
                column![
                    text("Custom Style"),
                    v_slider_rect,
                    text("Custom Texture Style"),
                    v_slider_texture,
                ]
                .max_width(120)
                .height(Length::Fill)
                .spacing(10),
                column![
                    text("Int Range"),
                    v_slider_int,
                    text("Freq Range"),
                    v_slider_freq,
                ]
                .max_width(120)
                .height(Length::Fill)
                .spacing(10),
                column![text("Custom Bipolar Style"), v_slider_rect_bp,]
                    .max_width(120)
                    .height(Length::Fill)
                    .spacing(10),
            ]
            .spacing(20),
        )
        .max_height(400);

        column![v_slider_row, text(&self.output_text).size(16),]
            .spacing(20)
            .padding(20)
            .into()
    }
}
