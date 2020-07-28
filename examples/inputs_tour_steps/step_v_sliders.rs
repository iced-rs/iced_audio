use iced::{Column, Element, Length, Row, Text};
use iced_native::image;

use iced_audio::{
    v_slider, FloatRange, FreqRange, IntRange, LogDBRange, TextMark,
    TextMarkGroup, TickMark, TickMarkGroup, TickMarkTier, VSlider,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum VSlidersID {
    Float,
    Int,
    DB,
    Freq,
    RectStyle,
    RectBipolarStyle,
    TextureStyle,
}

#[derive(Debug, Clone)]
pub enum Message {
    VSliderMoved(VSlidersID),
}

pub struct VSliderStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    v_slider_float_state: v_slider::State<VSlidersID>,
    v_slider_int_state: v_slider::State<VSlidersID>,
    v_slider_db_state: v_slider::State<VSlidersID>,
    v_slider_freq_state: v_slider::State<VSlidersID>,
    v_slider_rect_state: v_slider::State<VSlidersID>,
    v_slider_rect_bp_state: v_slider::State<VSlidersID>,
    v_slider_texture_state: v_slider::State<VSlidersID>,

    v_slider_texture_handle: image::Handle,

    float_tick_marks: TickMarkGroup,
    int_tick_marks: TickMarkGroup,
    db_tick_marks: TickMarkGroup,
    freq_tick_marks: TickMarkGroup,

    float_text_marks: TextMarkGroup,
    int_text_marks: TextMarkGroup,
    db_text_marks: TextMarkGroup,
    freq_text_marks: TextMarkGroup,

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
                float_range.create_param_default(VSlidersID::Float),
            ),

            v_slider_int_state: v_slider::State::new(
                int_range.create_param_default(VSlidersID::Int),
            ),

            v_slider_db_state: v_slider::State::new(
                db_range.create_param_default(VSlidersID::DB),
            ),

            v_slider_freq_state: v_slider::State::new(freq_range.create_param(
                VSlidersID::Freq,
                1000.0,
                1000.0,
            )),

            v_slider_rect_state: v_slider::State::new(
                float_range.create_param_default(VSlidersID::RectStyle),
            ),

            v_slider_rect_bp_state: v_slider::State::new(
                float_range.create_param_default(VSlidersID::RectBipolarStyle),
            ),

            v_slider_texture_state: v_slider::State::new(
                float_range.create_param_default(VSlidersID::TextureStyle),
            ),

            v_slider_texture_handle: format!(
                "{}/examples/images/iced_v_slider.png",
                env!("CARGO_MANIFEST_DIR")
            )
            .into(),

            float_tick_marks: TickMarkGroup::subdivided(
                1,
                1,
                1,
                Some(TickMarkTier::Two),
            ),

            int_tick_marks: TickMarkGroup::evenly_spaced(6, TickMarkTier::Two),

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

            float_text_marks: TextMarkGroup::min_max_and_center(
                "-1", "+1", "0",
            ),
            int_text_marks: TextMarkGroup::evenly_spaced(vec![
                "A", "B", "C", "D", "E", "F",
            ]),
            db_text_marks: TextMarkGroup::min_max_and_center("-12", "+12", "0"),
            freq_text_marks: TextMarkGroup::new(vec![
                TextMark::new("100", freq_range.to_normal(100.0)),
                TextMark::new("1k", freq_range.to_normal(1000.0)),
                TextMark::new("10k", freq_range.to_normal(10000.0)),
            ]),

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
            Message::VSliderMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    VSlidersID::Float => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.v_slider_float_state.param.normal,
                            ),
                        );
                    }
                    VSlidersID::Int => {
                        // Integer parameters must be snapped for the widget to
                        // "step" when moved.
                        self.int_range.snap_normal(
                            &mut self.v_slider_int_state.param.normal,
                        );

                        self.output_text = crate::info_text_i32(
                            id,
                            self.int_range
                                .to_value(self.v_slider_int_state.param.normal),
                        );
                    }
                    VSlidersID::DB => {
                        self.output_text = crate::info_text_db(
                            id,
                            self.db_range
                                .to_value(self.v_slider_db_state.param.normal),
                        );
                    }
                    VSlidersID::Freq => {
                        self.output_text = crate::info_text_freq(
                            id,
                            self.freq_range.to_value(
                                self.v_slider_freq_state.param.normal,
                            ),
                        );
                    }
                    VSlidersID::RectStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.v_slider_rect_state.param.normal,
                            ),
                        );
                    }
                    VSlidersID::RectBipolarStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.v_slider_rect_bp_state.param.normal,
                            ),
                        );
                    }
                    VSlidersID::TextureStyle => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range.to_value(
                                self.v_slider_texture_state.param.normal,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the VSlider widgets, passing in the value of
        // the corresponding parameter

        let v_slider_float =
            VSlider::new(&mut self.v_slider_float_state, Message::VSliderMoved)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks);

        let v_slider_int =
            VSlider::new(&mut self.v_slider_int_state, Message::VSliderMoved)
                .tick_marks(&self.int_tick_marks)
                .text_marks(&self.int_text_marks);

        let v_slider_db =
            VSlider::new(&mut self.v_slider_db_state, Message::VSliderMoved)
                .tick_marks(&self.db_tick_marks)
                .text_marks(&self.db_text_marks);

        let v_slider_freq =
            VSlider::new(&mut self.v_slider_freq_state, Message::VSliderMoved)
                .tick_marks(&self.freq_tick_marks)
                .text_marks(&self.freq_text_marks);

        let v_slider_rect =
            VSlider::new(&mut self.v_slider_rect_state, Message::VSliderMoved)
                .width(Length::from(Length::Units(24)))
                .style(style::VSliderRectStyle);

        let v_slider_rect_bp = VSlider::new(
            &mut self.v_slider_rect_bp_state,
            Message::VSliderMoved,
        )
        .width(Length::from(Length::Units(24)))
        .style(style::VSliderRectBipolarStyle);

        let v_slider_texture = VSlider::new(
            &mut self.v_slider_texture_state,
            Message::VSliderMoved,
        )
        .tick_marks(&self.float_tick_marks)
        .text_marks(&self.float_text_marks)
        // the width of the texture
        .width(Length::from(Length::Units(20)))
        .style(style::VSliderTextureStyle(
            // clone the handle to the loaded texture
            self.v_slider_texture_handle.clone(),
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
