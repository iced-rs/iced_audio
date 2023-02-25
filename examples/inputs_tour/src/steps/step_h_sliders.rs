use iced::widget::{column, image, row, text};
use iced::{Element, Length, Rectangle};

use iced_audio::{
    text_marks, tick_marks, FloatRange, FreqRange, HSlider, IntRange,
    LogDBRange, Normal, NormalParam,
};

use crate::{style, StepContainer};

#[derive(Debug, Clone)]
pub enum Message {
    Float(Normal),
    Int(Normal),
    DB(Normal),
    Freq(Normal),
    RectStyle(Normal),
    BipolarRectStyle(Normal),
    TextureStyle(Normal),
}

pub struct HSliderStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    float_param: NormalParam,
    int_param: NormalParam,
    db_param: NormalParam,
    freq_param: NormalParam,
    rect_param: NormalParam,
    rect_bp_param: NormalParam,
    texture_param: NormalParam,

    h_slider_texture_handle: image::Handle,

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

impl Default for HSliderStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = LogDBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            int_range,
            db_range,
            freq_range,

            // initialize the parameter of the HSlider widget
            float_param: float_range.default_normal_param(),
            int_param: int_range.default_normal_param(),
            db_param: db_range.default_normal_param(),
            freq_param: freq_range.normal_param(1000.0, 1000.0),
            rect_param: float_range.default_normal_param(),
            rect_bp_param: float_range.default_normal_param(),
            texture_param: float_range.default_normal_param(),

            h_slider_texture_handle: format!(
                "{}/../images/iced_h_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: tick_marks::Group::subdivided(
                1,
                1,
                1,
                Some(tick_marks::Tier::Two),
            ),

            int_tick_marks: tick_marks::Group::evenly_spaced(
                6,
                tick_marks::Tier::Two,
            ),

            db_tick_marks: vec![
                (db_range.map_to_normal(0.0), tick_marks::Tier::One),
                (db_range.map_to_normal(1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(12.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-12.0), tick_marks::Tier::Two),
            ]
            .into(),

            freq_tick_marks: vec![
                (freq_range.map_to_normal(20.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(50.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(100.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(200.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(400.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(1000.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(2000.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(5000.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(10000.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(20000.0), tick_marks::Tier::Two),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center(
                "-1", "+1", "0",
            ),
            int_text_marks: text_marks::Group::evenly_spaced(&[
                "A", "B", "C", "D", "E", "F",
            ]),
            db_text_marks: text_marks::Group::min_max_and_center(
                "-12", "+12", "0",
            ),
            freq_text_marks: vec![
                (freq_range.map_to_normal(100.0), "100"),
                (freq_range.map_to_normal(1000.0), "1k"),
                (freq_range.map_to_normal(10000.0), "10k"),
            ]
            .into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl HSliderStep {
    pub fn title(&self) -> &str {
        "Horizontal Sliders"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Float(normal) => {
                self.float_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "HSliderFloat",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.int_param.update(self.int_range.snapped(normal));

                self.output_text = crate::info_text_i32(
                    "HSliderInt",
                    self.int_range.unmap_to_value(normal),
                );
            }
            Message::DB(normal) => {
                self.db_param.update(normal);

                self.output_text = crate::info_text_db(
                    "HSliderDB",
                    self.db_range.unmap_to_value(normal),
                );
            }
            Message::Freq(normal) => {
                self.freq_param.update(normal);

                self.output_text = crate::info_text_freq(
                    "HSliderFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::RectStyle(normal) => {
                self.rect_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "HSliderRect",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::BipolarRectStyle(normal) => {
                self.rect_bp_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "HSliderBipolar",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::TextureStyle(normal) => {
                self.texture_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "HSliderTexture",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&self, _debug: bool) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float = HSlider::new(self.float_param, Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let h_slider_int = HSlider::new(self.int_param, Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let h_slider_db = HSlider::new(self.db_param, Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let h_slider_freq = HSlider::new(self.freq_param, Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let h_slider_rect = HSlider::new(self.rect_param, Message::RectStyle)
            .height(Length::Fixed(24.0))
            .style(style::h_slider::RectStyle);

        let h_slider_rect_bp =
            HSlider::new(self.rect_bp_param, Message::BipolarRectStyle)
                .height(Length::Fixed(24.0))
                .style(style::h_slider::RectBipolarStyle);

        let h_slider_texture =
            HSlider::new(self.texture_param, Message::TextureStyle)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks)
                // the height of the texture
                .height(Length::Fixed(20.0))
                .style(style::h_slider::TextureStyle(
                    // clone the handle to the loaded texture
                    self.h_slider_texture_handle.clone(),
                    // bounds of the texture, where the origin is in the center
                    // of the image
                    Rectangle {
                        x: -38.0 / 2.0,
                        y: -20.0 / 2.0,
                        width: 38.0,
                        height: 20.0,
                    },
                ));

        // push the widgets into rows
        let h_slider_row = row![
            column![
                text("Float Range"),
                h_slider_float,
                text("Log DB Range"),
                h_slider_db,
                text("Custom Style"),
                h_slider_rect,
                text("Custom Texture Style"),
                h_slider_texture,
            ]
            .width(Length::Fill)
            .spacing(20),
            column![
                text("Int Range"),
                h_slider_int,
                text("Freq Range"),
                h_slider_freq,
                text("Custom Bipolar Style"),
                h_slider_rect_bp,
            ]
            .width(Length::Fill)
            .spacing(20),
        ]
        .spacing(16);

        let content = column![h_slider_row, text(&self.output_text).size(16),]
            .spacing(20)
            .padding(20);

        StepContainer::<Self, _, _>::new("Horizontal Sliders (HSlider)")
            .push(content)
            .into()
    }
}
