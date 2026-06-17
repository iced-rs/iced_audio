mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, container, row, text},
};
use iced_audio::{
    FloatRange, FreqRange, IntRange, LogDBRange, Normal, NormalParam, VSlider, text_marks,
    tick_marks,
};
use util::info_text;

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
    Float(Normal),
    Int(Normal),
    DB(Normal),
    Freq(Normal),
    RectStyle(Normal),
    RectBipolarStyle(Normal),
    #[cfg(feature = "texture")]
    TextureStyle(Normal),
}

pub struct VSliderExample {
    float_range: FloatRange,
    float_range_bp: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    float_param: NormalParam,
    int_param: NormalParam,
    db_param: NormalParam,
    freq_param: NormalParam,
    rect_param: NormalParam,
    rect_bp_param: NormalParam,
    #[cfg(feature = "texture")]
    texture_param: NormalParam,

    #[cfg(feature = "texture")]
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
        // initalize parameters

        let float_range = FloatRange::default();
        let float_range_bp = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = LogDBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            float_range_bp,
            int_range,
            db_range,
            freq_range,

            // initialize the parameter of the VSlider widget
            float_param: float_range_bp.default_normal_param(),
            int_param: int_range.default_normal_param(),
            db_param: db_range.default_normal_param(),
            freq_param: freq_range.normal_param(1000.0, 1000.0),
            rect_param: float_range.default_normal_param(),
            rect_bp_param: float_range_bp.default_normal_param(),
            #[cfg(feature = "texture")]
            texture_param: float_range_bp.default_normal_param(),

            #[cfg(feature = "texture")]
            v_slider_texture_handle: format!(
                "{}/examples/images/iced_v_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: tick_marks::Group::subdivided(1, 1, 1, Some(tick_marks::Tier::Two)),

            int_tick_marks: tick_marks::Group::evenly_spaced(6, tick_marks::Tier::Two),

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

            float_text_marks: text_marks::Group::min_max_and_center("-1", "+1", "0"),
            int_text_marks: text_marks::Group::evenly_spaced(&["A", "B", "C", "D", "E", "F"]),
            db_text_marks: text_marks::Group::min_max_and_center("-12", "+12", "0"),
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

impl VSliderExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::Float(normal) => {
                self.float_param.update(normal);

                self.output_text = info_text::info_text_f32(
                    "VSliderFloat",
                    self.float_range_bp.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.int_param.update(self.int_range.snapped(normal));

                self.output_text =
                    info_text::info_text_i32("VSliderInt", self.int_range.unmap_to_value(normal));
            }
            Message::DB(normal) => {
                self.db_param.update(normal);

                self.output_text =
                    info_text::info_text_db("VSliderDB", self.db_range.unmap_to_value(normal));
            }
            Message::Freq(normal) => {
                self.freq_param.update(normal);

                self.output_text = info_text::info_text_freq(
                    "VSliderFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::RectStyle(normal) => {
                self.rect_param.update(normal);

                self.output_text = info_text::info_text_f32(
                    "VSliderRect",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::RectBipolarStyle(normal) => {
                self.rect_bp_param.update(normal);

                self.output_text = info_text::info_text_f32(
                    "VSliderBipolar",
                    self.float_range_bp.unmap_to_value(normal),
                );
            }
            #[cfg(feature = "texture")]
            Message::TextureStyle(normal) => {
                self.texture_param.update(normal);

                self.output_text = info_text::info_text_f32(
                    "VSliderTexture",
                    self.float_range_bp.unmap_to_value(normal),
                );
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the VSlider widgets, passing in the value of
        // the corresponding parameter

        let v_slider_float = VSlider::new(self.float_param, Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let v_slider_int = VSlider::new(self.int_param, Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let v_slider_db = VSlider::new(self.db_param, Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let v_slider_freq = VSlider::new(self.freq_param, Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let v_slider_rect = VSlider::new(self.rect_param, Message::RectStyle)
            .width(Length::Fixed(24.0))
            .style(style::v_slider::RectStyle);

        let v_slider_rect_bp = VSlider::new(self.rect_bp_param, Message::RectBipolarStyle)
            .width(Length::Fixed(24.0))
            .style(style::v_slider::RectBipolarStyle);

        #[cfg(feature = "texture")]
        let v_slider_texture = VSlider::new(self.texture_param, Message::TextureStyle)
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

        #[cfg(not(feature = "texture"))]
        let v_slider_texture = text("(enable the \"texture\" feature)");

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
