use iced::{image, Column, Element, Length, Rectangle, Row, Text};

use iced_audio::{
    text_marks, tick_marks, v_slider, FloatRange, FreqRange, IntRange,
    LogDBRange, Normal, VSlider,
};

use crate::{style, Step};

#[derive(Debug, Clone)]
pub enum Message {
    Float(Normal),
    Int(Normal),
    DB(Normal),
    Freq(Normal),
    RectStyle(Normal),
    RectBipolarStyle(Normal),
    TextureStyle(Normal),
}

pub struct VSliderStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    v_slider_float_state: v_slider::State,
    v_slider_int_state: v_slider::State,
    v_slider_db_state: v_slider::State,
    v_slider_freq_state: v_slider::State,
    v_slider_rect_state: v_slider::State,
    v_slider_rect_bp_state: v_slider::State,
    v_slider_texture_state: v_slider::State,

    v_slider_texture_handle: image::Handle,

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

impl Default for VSliderStep {
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

            // initialize the state of the VSlider widget
            v_slider_float_state: v_slider::State::new(
                float_range.default_normal_param(),
            ),

            v_slider_int_state: v_slider::State::new(
                int_range.default_normal_param(),
            ),

            v_slider_db_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),

            v_slider_freq_state: v_slider::State::new(
                freq_range.normal_param(1000.0, 1000.0),
            ),

            v_slider_rect_state: v_slider::State::new(
                float_range.default_normal_param(),
            ),

            v_slider_rect_bp_state: v_slider::State::new(
                float_range.default_normal_param(),
            ),

            v_slider_texture_state: v_slider::State::new(
                float_range.default_normal_param(),
            ),

            v_slider_texture_handle: format!(
                "{}/../images/iced_v_slider.png",
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

impl VSliderStep {
    pub fn title(&self) -> &str {
        "Vertical Sliders"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Float(normal) => {
                self.output_text = crate::info_text_f32(
                    "VSliderFloat",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.v_slider_int_state.snap_visible_to(&self.int_range);

                self.output_text = crate::info_text_i32(
                    "VSliderInt",
                    self.int_range.unmap_to_value(normal),
                );
            }
            Message::DB(normal) => {
                self.output_text = crate::info_text_db(
                    "VSliderDB",
                    self.db_range.unmap_to_value(normal),
                );
            }
            Message::Freq(normal) => {
                self.output_text = crate::info_text_freq(
                    "VSliderFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::RectStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "VSliderRect",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::RectBipolarStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "VSliderBipolar",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::TextureStyle(normal) => {
                self.output_text = crate::info_text_f32(
                    "VSliderTexture",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the VSlider widgets, passing in the value of
        // the corresponding parameter

        let v_slider_float =
            VSlider::new(&mut self.v_slider_float_state, Message::Float)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks);

        let v_slider_int =
            VSlider::new(&mut self.v_slider_int_state, Message::Int)
                .tick_marks(&self.int_tick_marks)
                .text_marks(&self.int_text_marks);

        let v_slider_db =
            VSlider::new(&mut self.v_slider_db_state, Message::DB)
                .tick_marks(&self.db_tick_marks)
                .text_marks(&self.db_text_marks);

        let v_slider_freq =
            VSlider::new(&mut self.v_slider_freq_state, Message::Freq)
                .tick_marks(&self.freq_tick_marks)
                .text_marks(&self.freq_text_marks);

        let v_slider_rect =
            VSlider::new(&mut self.v_slider_rect_state, Message::RectStyle)
                .width(Length::from(Length::Units(24)))
                .style(style::v_slider::RectStyle);

        let v_slider_rect_bp = VSlider::new(
            &mut self.v_slider_rect_bp_state,
            Message::RectBipolarStyle,
        )
        .width(Length::from(Length::Units(24)))
        .style(style::v_slider::RectBipolarStyle);

        let v_slider_texture = VSlider::new(
            &mut self.v_slider_texture_state,
            Message::TextureStyle,
        )
        .tick_marks(&self.float_tick_marks)
        .text_marks(&self.float_text_marks)
        // the width of the texture
        .width(Length::from(Length::Units(20)))
        .style(style::v_slider::TextureStyle(
            // clone the handle to the loaded texture
            self.v_slider_texture_handle.clone(),
            // bounds of the texture, where the origin is in the center
            // of the image
            Rectangle {
                x: -20.0 / 2.0,
                y: -38.0 / 2.0,
                width: 20.0,
                height: 38.0,
            },
        ));

        // push the widgets into rows
        let v_slider_row = Row::new()
            .spacing(20)
            .max_height(400)
            .push(
                Column::new()
                    .max_width(120)
                    .height(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Float Range"))
                    .push(v_slider_float)
                    .push(Text::new("Log DB Range"))
                    .push(v_slider_db),
            )
            .push(
                Column::new()
                    .max_width(120)
                    .height(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Custom Style"))
                    .push(v_slider_rect)
                    .push(Text::new("Custom Texture Style"))
                    .push(v_slider_texture),
            )
            .push(
                Column::new()
                    .max_width(120)
                    .height(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Int Range"))
                    .push(v_slider_int)
                    .push(Text::new("Freq Range"))
                    .push(v_slider_freq),
            )
            .push(
                Column::new()
                    .max_width(120)
                    .height(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Custom Bipolar Style"))
                    .push(v_slider_rect_bp),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(v_slider_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Vertical Sliders (VSlider)")
            .push(content)
            .into()
    }
}
