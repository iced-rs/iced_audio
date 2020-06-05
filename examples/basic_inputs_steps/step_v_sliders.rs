use iced::{Column, Element, Length, Row, Text};
use iced_native::image;

use iced_audio::{
    v_slider, FloatParam, IntParam, LogDBParam, Normal, OctaveParam, TickMark,
    TickMarkGroup, TickMarkTier, VSlider,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum VSlidersID {
    Float,
    Int,
    DB,
    Octave,
    RectStyle,
    BipolarRectStyle,
    TextureStyle,
}

#[derive(Debug, Clone)]
pub enum Message {
    VSlidersChanged((VSlidersID, Normal)),
}

pub struct VSliderStep {
    v_slider_float_param: FloatParam<VSlidersID>,
    v_slider_float_state: v_slider::State,
    v_slider_float_label: String,

    v_slider_int_param: IntParam<VSlidersID>,
    v_slider_int_state: v_slider::State,
    v_slider_int_label: String,

    v_slider_db_param: LogDBParam<VSlidersID>,
    v_slider_db_state: v_slider::State,
    v_slider_db_label: String,

    v_slider_oct_param: OctaveParam<VSlidersID>,
    v_slider_oct_state: v_slider::State,
    v_slider_oct_label: String,

    v_slider_rect_param: FloatParam<VSlidersID>,
    v_slider_rect_state: v_slider::State,
    v_slider_rect_label: String,

    v_slider_rect_bp_param: FloatParam<VSlidersID>,
    v_slider_rect_bp_state: v_slider::State,
    v_slider_rect_bp_label: String,

    v_slider_texture_param: FloatParam<VSlidersID>,
    v_slider_texture_state: v_slider::State,
    v_slider_texture_label: String,

    v_slider_texture_handle: image::Handle,

    float_tick_marks: TickMarkGroup,
    int_tick_marks: TickMarkGroup,
    db_tick_marks: TickMarkGroup,
    octave_tick_marks: TickMarkGroup,

    output_text: String,
}

impl Default for VSliderStep {
    fn default() -> Self {
        // initalize parameters

        let v_slider_float_param = FloatParam::<VSlidersID>::new(
            VSlidersID::Float,
            -1.0,
            1.0,
            0.0,
            0.0,
        );

        let v_slider_int_param =
            IntParam::<VSlidersID>::new(VSlidersID::Int, 0, 5, 0, 2);

        let v_slider_db_param = LogDBParam::<VSlidersID>::new(
            VSlidersID::DB,
            -12.0,
            12.0,
            0.0,
            0.0,
            0.5.into(),
        );

        let v_slider_oct_param = OctaveParam::<VSlidersID>::new(
            VSlidersID::Octave,
            20.0,
            20_480.0,
            1000.0,
            1000.0,
        );

        let v_slider_rect_param = FloatParam::<VSlidersID>::new(
            VSlidersID::RectStyle,
            0.0,
            1.0,
            0.0,
            0.0,
        );

        let v_slider_rect_bp_param = FloatParam::<VSlidersID>::new(
            VSlidersID::BipolarRectStyle,
            -1.0,
            1.0,
            0.0,
            0.0,
        );

        let v_slider_texture_param = FloatParam::<VSlidersID>::new(
            VSlidersID::TextureStyle,
            0.0,
            1.0,
            1.0,
            1.0,
        );

        // create application

        Self {
            // add the parameter
            v_slider_float_param,
            // initialize the state of the VSlider widget
            v_slider_float_state: v_slider::State::new(&v_slider_float_param),
            // initialize the label above the VSlider widget
            v_slider_float_label: String::from("Float Range"),

            v_slider_int_param,
            v_slider_int_state: v_slider::State::new(&v_slider_int_param),
            v_slider_int_label: String::from("Int Range"),

            v_slider_db_param,
            v_slider_db_state: v_slider::State::new(&v_slider_db_param),
            v_slider_db_label: String::from("Log dB Range"),

            v_slider_oct_param,
            v_slider_oct_state: v_slider::State::new(&v_slider_oct_param),
            v_slider_oct_label: String::from("Octave Freq Range"),

            v_slider_rect_param,
            v_slider_rect_state: v_slider::State::new(&v_slider_rect_param),
            v_slider_rect_label: String::from("Custom Rect Style"),

            v_slider_rect_bp_param,
            v_slider_rect_bp_state: v_slider::State::new(
                &v_slider_rect_bp_param,
            ),
            v_slider_rect_bp_label: String::from("Custom Rect Bipolar Style"),

            v_slider_texture_param,
            v_slider_texture_state: v_slider::State::new(
                &v_slider_texture_param,
            ),
            v_slider_texture_label: String::from("Custom Texture Style"),

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

            int_tick_marks: TickMarkGroup::subdivided(
                0,
                4,
                0,
                Some(TickMarkTier::Two),
            ),

            db_tick_marks: vec![
                TickMark {
                    position: v_slider_db_param.value_to_normal(0.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(12.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(-1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(-3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(-6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_db_param.value_to_normal(-12.0),
                    tier: TickMarkTier::Two,
                },
            ]
            .into(),

            octave_tick_marks: vec![
                TickMark {
                    position: v_slider_oct_param.value_to_normal(20.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(50.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(100.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(200.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(400.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(1000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(2000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(5000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(10000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: v_slider_oct_param.value_to_normal(20000.0),
                    tier: TickMarkTier::Two,
                },
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
            Message::VSlidersChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // VSlider widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    VSlidersID::Float => {
                        self.v_slider_float_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.v_slider_float_param.value(),
                        );
                    }
                    VSlidersID::Int => {
                        self.v_slider_int_param.set_from_normal(normal);
                        self.output_text = crate::info_text_i32(
                            id,
                            self.v_slider_int_param.value(),
                        );
                    }
                    VSlidersID::DB => {
                        self.v_slider_db_param.set_from_normal(normal);
                        self.output_text = crate::info_text_db(
                            id,
                            self.v_slider_db_param.value(),
                        );
                    }
                    VSlidersID::Octave => {
                        self.v_slider_oct_param.set_from_normal(normal);
                        self.output_text = crate::info_text_octave(
                            id,
                            self.v_slider_oct_param.value(),
                        );
                    }
                    VSlidersID::RectStyle => {
                        self.v_slider_rect_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.v_slider_rect_param.value(),
                        );
                    }
                    VSlidersID::BipolarRectStyle => {
                        self.v_slider_rect_bp_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.v_slider_rect_bp_param.value(),
                        );
                    }
                    VSlidersID::TextureStyle => {
                        self.v_slider_texture_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.v_slider_texture_param.value(),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the VSlider widgets, passing in the value of
        // the corresponding parameter

        let v_slider_float = VSlider::new(
            &mut self.v_slider_float_state,
            &self.v_slider_float_param,
            Message::VSlidersChanged,
        )
        .tick_marks(&self.float_tick_marks);

        let v_slider_int = VSlider::new(
            &mut self.v_slider_int_state,
            &self.v_slider_int_param,
            Message::VSlidersChanged,
        )
        .tick_marks(&self.int_tick_marks);

        let v_slider_db = VSlider::new(
            &mut self.v_slider_db_state,
            &self.v_slider_db_param,
            Message::VSlidersChanged,
        )
        .tick_marks(&self.db_tick_marks);

        let v_slider_oct = VSlider::new(
            &mut self.v_slider_oct_state,
            &self.v_slider_oct_param,
            Message::VSlidersChanged,
        )
        .tick_marks(&self.octave_tick_marks);

        let v_slider_style = VSlider::new(
            &mut self.v_slider_rect_state,
            &self.v_slider_rect_param,
            Message::VSlidersChanged,
        )
        .width(Length::from(Length::Units(28)))
        .style(style::VSliderRectStyle);

        let v_slider_rect_bp = VSlider::new(
            &mut self.v_slider_rect_bp_state,
            &self.v_slider_rect_bp_param,
            Message::VSlidersChanged,
        )
        .width(Length::from(Length::Units(28)))
        .style(style::VSliderRectBipolarStyle);

        let v_slider_texture = VSlider::new(
            &mut self.v_slider_texture_state,
            &self.v_slider_texture_param,
            Message::VSlidersChanged,
        )
        .tick_marks(&self.float_tick_marks)
        // the width of the handle texture
        .width(Length::from(Length::Units(20)))
        // clone the handle to the loaded texture
        .style(style::VSliderTextureStyle(
            self.v_slider_texture_handle.clone(),
        ));

        // push the sliders into columns

        let v_slider_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .max_height(400)
                    // * Note: `.width(Length::Fill)` is broken for `VSlider`
                    // for the time being.
                    .max_width(120)
                    .spacing(10)
                    .push(Text::new(&self.v_slider_float_label))
                    .push(v_slider_float)
                    .push(Text::new(&self.v_slider_int_label))
                    .push(v_slider_int),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .max_width(120)
                    .spacing(10)
                    .push(Text::new(&self.v_slider_db_label))
                    .push(v_slider_db)
                    .push(Text::new(&self.v_slider_oct_label))
                    .push(v_slider_oct),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .max_width(120)
                    .spacing(10)
                    .push(Text::new(&self.v_slider_rect_label))
                    .push(v_slider_style)
                    .push(Text::new(&self.v_slider_texture_label))
                    .push(v_slider_texture),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .max_width(120)
                    .spacing(10)
                    .push(Text::new(&self.v_slider_rect_bp_label))
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
