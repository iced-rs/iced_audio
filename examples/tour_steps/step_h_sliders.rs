use iced::{
    Column, Element, Length, Text, Row
};
use iced_native::image;

use iced_audio::{Normal, FloatParam, IntParam, LogDBParam,
    OctaveParam, h_slider, HSlider
};

use crate::{Step, style};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum HSlidersID {
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
    HSlidersChanged((HSlidersID, Normal)),
}

pub struct HSliderStep {
    h_slider_float_param: FloatParam<HSlidersID>,
    h_slider_float_state: h_slider::State<HSlidersID>,
    h_slider_float_label: String,

    h_slider_int_param: IntParam<HSlidersID>,
    h_slider_int_state: h_slider::State<HSlidersID>,
    h_slider_int_label: String,

    h_slider_log_param: LogDBParam<HSlidersID>,
    h_slider_log_state: h_slider::State<HSlidersID>,
    h_slider_log_label: String,

    h_slider_oct_param: OctaveParam<HSlidersID>,
    h_slider_oct_state: h_slider::State<HSlidersID>,
    h_slider_oct_label: String,

    h_slider_rect_param: FloatParam<HSlidersID>,
    h_slider_rect_state: h_slider::State<HSlidersID>,
    h_slider_rect_label: String,

    h_slider_rect_bp_param: FloatParam<HSlidersID>,
    h_slider_rect_bp_state: h_slider::State<HSlidersID>,
    h_slider_rect_bp_label: String,

    h_slider_texture_param: FloatParam<HSlidersID>,
    h_slider_texture_state: h_slider::State<HSlidersID>,
    h_slider_texture_label: String,

    h_slider_texture_handle: image::Handle,

    output_text: String,
}

impl Default for HSliderStep {
    fn default() -> Self {
        // initalize parameters

        let h_slider_float_param = FloatParam::<HSlidersID>::new(
            HSlidersID::Float, -1.0, 1.0, 0.0, 0.0);

        let h_slider_int_param = IntParam::<HSlidersID>::new(
            HSlidersID::Int, 0, 5, 0, 2);

        let h_slider_log_param = LogDBParam::<HSlidersID>::new(
            HSlidersID::DB, -12.0, 12.0, 0.0, 0.0, 0.5.into());

        let h_slider_oct_param = OctaveParam::<HSlidersID>::new(
            HSlidersID::Octave, 20.0, 20_480.0, 1000.0, 1000.0);

        let h_slider_rect_param = FloatParam::<HSlidersID>::new(
            HSlidersID::RectStyle, 0.0, 1.0, 0.0, 0.0);

        let h_slider_rect_bp_param = FloatParam::<HSlidersID>::new(
            HSlidersID::BipolarRectStyle, -1.0, 1.0, 0.0, 0.0);

        let h_slider_texture_param = FloatParam::<HSlidersID>::new(
            HSlidersID::TextureStyle, 0.0, 1.0, 0.0, 0.0);
        
        // create application
        
        Self {
            // add the parameter
            h_slider_float_param,
            // initialize the state of the HSlider widget
            h_slider_float_state: h_slider::State::new(
                &h_slider_float_param
            ),
            // initialize the label above the HSlider widget
            h_slider_float_label: String::from("Float Range"),


            h_slider_int_param,
            h_slider_int_state: h_slider::State::new(
                &h_slider_int_param
            ),
            h_slider_int_label: String::from("Int Range"),
            

            h_slider_log_param,
            h_slider_log_state: h_slider::State::new(
                &h_slider_log_param
            ),
            h_slider_log_label: String::from("Log dB Range"),
            

            h_slider_oct_param,
            h_slider_oct_state: h_slider::State::new(
                &h_slider_oct_param
            ),
            h_slider_oct_label: String::from("Octave Freq Range"),


            h_slider_rect_param,
            h_slider_rect_state: h_slider::State::new(
                &h_slider_rect_param
            ),
            h_slider_rect_label: String::from("Rect Style"),
            

            h_slider_rect_bp_param,
            h_slider_rect_bp_state: h_slider::State::new(
                &h_slider_rect_bp_param
            ),
            h_slider_rect_bp_label: String::from("Rect Bipolar Style"),


            h_slider_texture_param,
            h_slider_texture_state: h_slider::State::new(
                &h_slider_texture_param
            ),
            h_slider_texture_label: String::from("Texture Style"),


            h_slider_texture_handle: format!(
                "{}/examples/images/iced_h_slider.png",
                env!("CARGO_MANIFEST_DIR")
            ).into(),


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
            Message::HSlidersChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // HSlider widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    HSlidersID::Float => {
                        self.h_slider_float_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.h_slider_float_param.value());
                    },
                    HSlidersID::Int => {
                        self.h_slider_int_param.set_from_normal(normal);
                        self.output_text = crate::info_text_i32(id,
                            self.h_slider_int_param.value());
                    },
                    HSlidersID::DB => {
                        self.h_slider_log_param.set_from_normal(normal);
                        self.output_text = crate::info_text_db(id,
                            self.h_slider_log_param.value());
                    },
                    HSlidersID::Octave => {
                        self.h_slider_oct_param.set_from_normal(normal);
                        self.output_text = crate::info_text_octave(id,
                            self.h_slider_oct_param.value());
                    },
                    HSlidersID::RectStyle => {
                        self.h_slider_rect_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.h_slider_rect_param.value());
                    },
                    HSlidersID::BipolarRectStyle => {
                        self.h_slider_rect_bp_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.h_slider_rect_bp_param.value());
                    },
                    HSlidersID::TextureStyle => {
                        self.h_slider_texture_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.h_slider_texture_param.value());
                    },
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float = HSlider::new(
            &mut self.h_slider_float_state,
            &self.h_slider_float_param,
            Message::HSlidersChanged,
        );

        let h_slider_int = HSlider::new(
            &mut self.h_slider_int_state,
            &self.h_slider_int_param,
            Message::HSlidersChanged,
        );

        let h_slider_log = HSlider::new(
            &mut self.h_slider_log_state,
            &self.h_slider_log_param,
            Message::HSlidersChanged,
        );

        let h_slider_oct = HSlider::new(
            &mut self.h_slider_oct_state,
            &self.h_slider_oct_param,
            Message::HSlidersChanged,
        );

        let h_slider_style = HSlider::new(
            &mut self.h_slider_rect_state,
            &self.h_slider_rect_param,
            Message::HSlidersChanged,
        )
        .style(style::HSliderRectStyle);

        let h_slider_rect_bp = HSlider::new(
            &mut self.h_slider_rect_bp_state,
            &self.h_slider_rect_bp_param,
            Message::HSlidersChanged,
        )
        .style(style::HSliderRectBipolarStyle);

        let h_slider_texture = HSlider::new(
            &mut self.h_slider_texture_state,
            &self.h_slider_texture_param,
            Message::HSlidersChanged,
        )
        // clone the handle to the loaded texture
        .style(style::HSliderTextureStyle(
            self.h_slider_texture_handle.clone()
        ));


        // push the sliders into rows

        let h_slider_row = Row::new()
            .spacing(20)

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_float_label))
                .push(h_slider_float)

                .push(Text::new(&self.h_slider_log_label))
                .push(h_slider_log)

                .push(Text::new(&self.h_slider_rect_label))
                .push(h_slider_style)

                .push(Text::new(&self.h_slider_texture_label))
                .push(h_slider_texture)
            )

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_int_label))
                .push(h_slider_int)

                .push(Text::new(&self.h_slider_oct_label))
                .push(h_slider_oct)

                .push(Text::new(&self.h_slider_rect_bp_label))
                .push(h_slider_rect_bp)
            );

            let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(h_slider_row)
            .push(Text::new(&self.output_text).size(16));


        Step::container("Horizontal Sliders (HSlider)").push(content).into()
    }
}