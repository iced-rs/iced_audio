use iced::{Column, Element, Length, Row, Text};
use iced_native::image;

use iced_audio::{
    h_slider, DBRange, FloatRange, FreqRange, HSlider, IntRange, TickMark,
    TickMarkGroup, TickMarkTier,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum HSlidersID {
    Float,
    Int,
    DB,
    Freq,
    RectStyle,
    BipolarRectStyle,
    TextureStyle,
}

#[derive(Debug, Clone)]
pub enum Message {
    HSliderMoved(HSlidersID),
}

pub struct HSliderStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: DBRange,
    freq_range: FreqRange,

    h_slider_float_state: h_slider::State<HSlidersID>,
    h_slider_int_state: h_slider::State<HSlidersID>,
    h_slider_db_state: h_slider::State<HSlidersID>,
    h_slider_freq_state: h_slider::State<HSlidersID>,
    h_slider_rect_state: h_slider::State<HSlidersID>,
    h_slider_rect_bp_state: h_slider::State<HSlidersID>,
    h_slider_texture_state: h_slider::State<HSlidersID>,

    h_slider_texture_handle: image::Handle,

    float_tick_marks: TickMarkGroup,
    int_tick_marks: TickMarkGroup,
    db_tick_marks: TickMarkGroup,
    freq_tick_marks: TickMarkGroup,

    output_text: String,
}

impl Default for HSliderStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = DBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            int_range,
            db_range,
            freq_range,

            // initialize the state of the HSlider widget
            h_slider_float_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::Float),
            ),

            h_slider_int_state: h_slider::State::new(
                int_range.create_param_default(HSlidersID::Int),
            ),

            h_slider_db_state: h_slider::State::new(
                db_range.create_param_default(HSlidersID::DB),
            ),

            h_slider_freq_state: h_slider::State::new(
                freq_range.create_param_default(HSlidersID::Freq),
            ),

            h_slider_rect_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::RectStyle),
            ),

            h_slider_rect_bp_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::BipolarRectStyle),
            ),

            h_slider_texture_state: h_slider::State::new(
                float_range.create_param_default(HSlidersID::TextureStyle),
            ),

            h_slider_texture_handle: format!(
                "{}/examples/images/iced_h_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: TickMarkGroup::subdivided(
                1,
                1,
                1,
                Some(TickMarkTier::Two),
            ),

            int_tick_marks: TickMarkGroup::subdivided(
                0,
                4,
                0,
                Some(TickMarkTier::Two),
            ),

            db_tick_marks: vec![
                TickMark {
                    position: db_range.to_normal(0.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: db_range.to_normal(1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(12.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(-1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(-3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(-6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: db_range.to_normal(-12.0),
                    tier: TickMarkTier::Two,
                },
            ]
            .into(),

            freq_tick_marks: vec![
                TickMark {
                    position: freq_range.to_normal(20.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(50.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(100.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: freq_range.to_normal(200.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(400.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(1000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: freq_range.to_normal(2000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(5000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: freq_range.to_normal(10000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: freq_range.to_normal(20000.0),
                    tier: TickMarkTier::Two,
                },
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
            Message::HSliderMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    HSlidersID::Float => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_float_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::Int => {
                        // Integer parameters must be snapped for the widget to
                        // "step" when moved.
                        self.int_range.snap_normal(
                            &mut self.h_slider_int_state.param.normal,
                        );

                        self.output_text = crate::info_text_i32(
                            id,
                            self.int_range
                                .to_value(self.h_slider_int_state.param.normal),
                        );
                    }
                    HSlidersID::DB => {
                        self.output_text = crate::info_text_db(
                            id,
                            self.db_range
                                .to_value(self.h_slider_db_state.param.normal),
                        );
                    }
                    HSlidersID::Freq => {
                        self.output_text = crate::info_text_freq(
                            id,
                            self.freq_range.to_value(
                                self.h_slider_freq_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::RectStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_rect_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::BipolarRectStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_rect_bp_state.param.normal,
                            ),
                        );
                    }
                    HSlidersID::TextureStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.h_slider_texture_state.param.normal,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float =
            HSlider::new(&mut self.h_slider_float_state, Message::HSliderMoved)
                .tick_marks(&self.float_tick_marks);

        let h_slider_int =
            HSlider::new(&mut self.h_slider_int_state, Message::HSliderMoved)
                .tick_marks(&self.int_tick_marks);

        let h_slider_db =
            HSlider::new(&mut self.h_slider_db_state, Message::HSliderMoved)
                .tick_marks(&self.db_tick_marks);

        let h_slider_freq =
            HSlider::new(&mut self.h_slider_freq_state, Message::HSliderMoved)
                .tick_marks(&self.freq_tick_marks);

        let h_slider_rect =
            HSlider::new(&mut self.h_slider_rect_state, Message::HSliderMoved)
                .height(Length::from(Length::Units(24)))
                .style(style::HSliderRectStyle);

        let h_slider_rect_bp = HSlider::new(
            &mut self.h_slider_rect_bp_state,
            Message::HSliderMoved,
        )
        .height(Length::from(Length::Units(24)))
        .style(style::HSliderRectBipolarStyle);

        let h_slider_texture = HSlider::new(
            &mut self.h_slider_texture_state,
            Message::HSliderMoved,
        )
        .tick_marks(&self.float_tick_marks)
        // the height of the texture
        .height(Length::from(Length::Units(20)))
        .style(style::HSliderTextureStyle(
            // clone the handle to the loaded texture
            self.h_slider_texture_handle.clone(),
        ));

        // push the widgets into rows
        let h_slider_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new("Float Range"))
                    .push(h_slider_float)
                    .push(Text::new("DB Range"))
                    .push(h_slider_db)
                    .push(Text::new("Custom Style"))
                    .push(h_slider_rect)
                    .push(Text::new("Custom Texture Style"))
                    .push(h_slider_texture),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(10)
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
