use iced::{image, Column, Element, Length, Rectangle, Row, Text};

use iced_audio::{
    h_slider, text_marks, tick_marks, FloatRange, FreqRange, HSlider, IntRange,
    LogDBRange, Normal,
};

use crate::{style, Step};

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

    h_slider_float_state: h_slider::State,
    h_slider_int_state: h_slider::State,
    h_slider_db_state: h_slider::State,
    h_slider_freq_state: h_slider::State,
    h_slider_rect_state: h_slider::State,
    h_slider_rect_bp_state: h_slider::State,
    h_slider_texture_state: h_slider::State,

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

            // initialize the state of the HSlider widget
            h_slider_float_state: h_slider::State::new(
                float_range.default_normal_param(),
            ),

            h_slider_int_state: h_slider::State::new(
                int_range.default_normal_param(),
            ),

            h_slider_db_state: h_slider::State::new(
                db_range.default_normal_param(),
            ),

            h_slider_freq_state: h_slider::State::new(
                freq_range.normal_param(1000.0, 1000.0),
            ),

            h_slider_rect_state: h_slider::State::new(
                float_range.default_normal_param(),
            ),

            h_slider_rect_bp_state: h_slider::State::new(
                float_range.default_normal_param(),
            ),

            h_slider_texture_state: h_slider::State::new(
                float_range.default_normal_param(),
            ),

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
                self.output_text = crate::info_text_f32(
                    "HSliderFloat",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped for the widget to
                // "step" when moved.
                self.int_range
                    .snap(&mut self.h_slider_int_state.normal_param.value);

                self.output_text = crate::info_text_i32(
                    "HSliderInt",
                    self.int_range.unmap_to_value(normal),
                );
            }
            Message::DB(normal) => {
                self.output_text = crate::info_text_db(
                    "HSliderDB",
                    self.db_range.unmap_to_value(normal),
                );
            }
            Message::Freq(normal) => {
                self.output_text = crate::info_text_freq(
                    "HSliderFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::RectStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "HSliderRect",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::BipolarRectStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "HSliderBipolar",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::TextureStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "HSliderTexture",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float =
            HSlider::new(&mut self.h_slider_float_state, Message::Float)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks);

        let h_slider_int =
            HSlider::new(&mut self.h_slider_int_state, Message::Int)
                .tick_marks(&self.int_tick_marks)
                .text_marks(&self.int_text_marks);

        let h_slider_db =
            HSlider::new(&mut self.h_slider_db_state, Message::DB)
                .tick_marks(&self.db_tick_marks)
                .text_marks(&self.db_text_marks);

        let h_slider_freq =
            HSlider::new(&mut self.h_slider_freq_state, Message::Freq)
                .tick_marks(&self.freq_tick_marks)
                .text_marks(&self.freq_text_marks);

        let h_slider_rect =
            HSlider::new(&mut self.h_slider_rect_state, Message::RectStyle)
                .height(Length::from(Length::Units(24)))
                .style(style::h_slider::RectStyle);

        let h_slider_rect_bp = HSlider::new(
            &mut self.h_slider_rect_bp_state,
            Message::BipolarRectStyle,
        )
        .height(Length::from(Length::Units(24)))
        .style(style::h_slider::RectBipolarStyle);

        let h_slider_texture = HSlider::new(
            &mut self.h_slider_texture_state,
            Message::TextureStyle,
        )
        .tick_marks(&self.float_tick_marks)
        .text_marks(&self.float_text_marks)
        // the height of the texture
        .height(Length::from(Length::Units(20)))
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
        let h_slider_row = Row::new()
            .spacing(16)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Float Range"))
                    .push(h_slider_float)
                    .push(Text::new("Log DB Range"))
                    .push(h_slider_db)
                    .push(Text::new("Custom Style"))
                    .push(h_slider_rect)
                    .push(Text::new("Custom Texture Style"))
                    .push(h_slider_texture),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Int Range"))
                    .push(h_slider_int)
                    .push(Text::new("Freq Range"))
                    .push(h_slider_freq)
                    .push(Text::new("Custom Bipolar Style"))
                    .push(h_slider_rect_bp),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(h_slider_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Horizontal Sliders (HSlider)")
            .push(content)
            .into()
    }
}
