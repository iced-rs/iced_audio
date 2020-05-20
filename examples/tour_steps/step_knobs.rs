use iced::{
    Column, Element, Length, Text, Row
};
//use iced_native::image;

use iced_audio::{Normal, FloatParam, IntParam, LogDBParam,
    OctaveParam, knob, Knob
};

use crate::Step;

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum KnobsID {
    Float,
    Int,
    DB,
    Octave,
    Vector,
    Texture,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobsChanged((KnobsID, Normal)),
}

pub struct KnobsStep {
    knob_float_param: FloatParam<KnobsID>,
    knob_float_state: knob::State,
    knob_float_label: String,

    knob_int_param: IntParam<KnobsID>,
    knob_int_state: knob::State,
    knob_int_label: String,

    knob_log_param: LogDBParam<KnobsID>,
    knob_log_state: knob::State,
    knob_log_label: String,

    knob_oct_param: OctaveParam<KnobsID>,
    knob_oct_state: knob::State,
    knob_oct_label: String,

    /*
    knob_vector_param: FloatParam<KnobsID>,
    knob_vector_state: knob::State,
    knob_vector_label: String,
    */

    /*
    knob_texture_param: FloatParam<KnobsID>,
    knob_texture_state: knob::State,
    knob_texture_label: String,

    knob_texture_handle: image::Handle,
    */

    output_text: String,
}

impl Default for KnobsStep {
    fn default() -> Self {
        // initalize parameters

        let knob_float_param = FloatParam::<KnobsID>::new(
            KnobsID::Float, -1.0, 1.0, 0.0, 0.0);

        let knob_int_param = IntParam::<KnobsID>::new(
            KnobsID::Int, 0, 5, 0, 2);

        let knob_log_param = LogDBParam::<KnobsID>::new(
            KnobsID::DB, -12.0, 12.0, 0.0, 0.0, 0.5.into());

        let knob_oct_param = OctaveParam::<KnobsID>::new(
            KnobsID::Octave, 20.0, 20_480.0, 1000.0, 1000.0);

        /*
        let knob_vector_param = FloatParam::<KnobsID>::new(
            KnobsID::Vector, -1.0, 1.0, 0.0, 0.0);
        */
        /*
        let knob_texture_param = FloatParam::<KnobsID>::new(
            KnobsID::Texture, -1.0, 1.0, 0.0, 0.0);
        */
        
        // create application
        
        Self {
            // add the parameter
            knob_float_param,
            // initialize the state of the Knob widget
            knob_float_state: knob::State::new(
                &knob_float_param
            ),
            // initialize the label above the Knob widget
            knob_float_label: String::from("Float Range"),

            knob_int_param,
            knob_int_state: knob::State::new(
                &knob_int_param
            ),
            knob_int_label: String::from("Int Range"),
            

            knob_log_param,
            knob_log_state: knob::State::new(
                &knob_log_param
            ),
            knob_log_label: String::from("Log dB Range"),
            

            knob_oct_param,
            knob_oct_state: knob::State::new(
                &knob_oct_param
            ),
            knob_oct_label: String::from("Octave Freq Range"),

            /*
            knob_vector_param,
            knob_vector_state: knob::State::new(
                &knob_vector_param
            ),
            knob_vector_label: String::from("Custom Vector Style"),
            */
            
            /*
            knob_texture_param,
            knob_texture_state: knob::State::new(
                &knob_texture_param
            ),
            knob_texture_label: String::from("Custom Texture Style"),


            knob_texture_handle: format!(
                "{}/examples/images/iced_knob.png",
                env!("CARGO_MANIFEST_DIR")
            ).into(),
            */

            output_text: String::from("Move a widget"),
        }
    }
}

impl KnobsStep {
    pub fn title(&self) -> &str {
        "Knobs"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobsChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // Knobs widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    KnobsID::Float => {
                        self.knob_float_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.knob_float_param.value());
                    },
                    KnobsID::Int => {
                        self.knob_int_param.set_from_normal(normal);
                        self.output_text = crate::info_text_i32(id,
                            self.knob_int_param.value());
                    },
                    KnobsID::DB => {
                        self.knob_log_param.set_from_normal(normal);
                        self.output_text = crate::info_text_db(id,
                            self.knob_log_param.value());
                    },
                    KnobsID::Octave => {
                        self.knob_oct_param.set_from_normal(normal);
                        self.output_text = crate::info_text_octave(id,
                            self.knob_oct_param.value());
                    },
                    KnobsID::Vector => {
                        /*
                        self.knob_vector_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.knob_vector_param.value());
                        */
                    },
                    KnobsID::Texture => {
                        /*
                        self.knob_texture_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.knob_texture_param.value());
                        */
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knobs widgets, passing in the value of
        // the corresponding parameter

        let knob_float = Knob::new(
            &mut self.knob_float_state,
            &self.knob_float_param,
            Message::KnobsChanged,
        );

        let knob_int = Knob::new(
            &mut self.knob_int_state,
            &self.knob_int_param,
            Message::KnobsChanged,
        );

        let knob_log = Knob::new(
            &mut self.knob_log_state,
            &self.knob_log_param,
            Message::KnobsChanged,
        );

        let knob_oct = Knob::new(
            &mut self.knob_oct_state,
            &self.knob_oct_param,
            Message::KnobsChanged,
        );

        /*
        let knob_texture = Knob::new(
            &mut self.knob_texture_state,
            &self.knob_texture_param,
            Message::KnobsChanged,
        )
        // clone the handle to the loaded texture
        .style(style::KnobTextureStyle(
            self.knob_texture_handle.clone()
        ));
        */

        /*
        let knob_vector = Knob::new(
            &mut self.knob_vector_state,
            &self.knob_vector_param,
            Message::KnobsChanged,
        )
        .style(style::KnobVectorStyle);
        */


        // push the knobs into columns

        let knob_row = Row::new()
            .spacing(20)

            .push(Column::new()
                .max_height(400)
                .width(Length::Fill)
                .spacing(10)
                .push(Text::new(&self.knob_float_label))
                .push(knob_float)

                .push(Text::new(&self.knob_int_label))
                .push(knob_int)
            )

            .push(Column::new()
                .max_height(400)
                .width(Length::Fill)
                .spacing(10)
                .push(Text::new(&self.knob_log_label))
                .push(knob_log)

                .push(Text::new(&self.knob_oct_label))
                .push(knob_oct)
            );

            /*
            .push(Column::new()
                .max_height(400)
                .width(Length::Fill)
                .spacing(10)
                //.push(Text::new(&self.knob_vector_label))
                //.push(knob_vector)

                //.push(Text::new(&self.knob_texture_label))
                //.push(knob_texture)
            );
            */

            let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));


        Step::container("Knobs").push(content).into()
    }
}