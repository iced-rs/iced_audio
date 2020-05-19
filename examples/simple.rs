// Import iced crate.
use iced::{
    Column, Container, Element, Length, Sandbox, Settings, Align
};
// Import iced_audio crate.
use iced_audio::{
    Normal, FloatParam, LogDBParam, OctaveParam, h_slider, HSlider,
    v_slider, VSlider, knob, Knob
};

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    HSliderFloat,
    VSliderDB,
    KnobOctave,
}

// The message when a parameter widget is changed by the user
#[derive(Debug, Clone)]
pub enum Message {
    ParamChanged((ParamID, Normal)),
}

pub fn main() {
    App::run(Settings::default())
}

pub struct App {
    
    // The parameters (`Param`) hold the current and default values.
    // They also handle converting the output of the widget to a usable value.
    //
    // There are 4 options available for a parameter:
    //
    // * FloatParam - a linear range of f32 values
    // * IntParam - a discrete range of i32 values. This will cause the widget
    // to "step" when moved.
    // * LogDBParam - a logarithmic range of decibel values. Values around 0 dB
    // will increment slower than values farther away from 0 dB.
    // * OctaveParam - a logarithmic range of frequency values. Each octave in
    // the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.
    //
    h_slider_float_param: FloatParam<ParamID>,
    v_slider_db_param: LogDBParam<ParamID>,
    knob_octave_param: OctaveParam<ParamID>,

    // The states of the parameter widgets that will control the parameters.
    h_slider_state: h_slider::State,
    v_slider_state: v_slider::State,
    knob_state: knob::State,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {

        // Initialize each parameter:
        // * `ID` - A unique identifier for each parameter
        // * `min` - The minimum of the range (inclusive)
        // * `max` - The maximum of the range (inclusive)
        // * `value` - The initial value of the parameter
        // * `default_value` - The default value of the parameter
        let h_slider_float_param = FloatParam::<ParamID>::new(
            ParamID::HSliderFloat , -1.0, 1.0, 0.0, 0.0);

        let v_slider_db_param = LogDBParam::<ParamID>::new(
            ParamID::VSliderDB , -12.0, 12.0, 0.0, 0.0, 0.5.into());

        let knob_octave_param = OctaveParam::<ParamID>::new(
            ParamID::KnobOctave , 20.0, 20480.0, 1000.0, 1000.0);

        App {
            // Add the parameters.
            h_slider_float_param,
            v_slider_db_param,
            knob_octave_param,

            // Initialize the state of the widgets with the initial value
            // of the corresponding parameter.
            h_slider_state: h_slider::State::new(&h_slider_float_param),
            v_slider_state: v_slider::State::new(&v_slider_db_param),
            knob_state: knob::State::new(&knob_octave_param),
        }
    }

    fn title(&self) -> String {
        format!("Simple Example - Iced Audio")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ParamChanged((id, normal)) => {

                // Update each parameter with the `Normal` output value from
                // the corresponding parameter widget.
                //
                // Now do something useful with that value!
                //
                match id {
                    ParamID::HSliderFloat => {
                        self.h_slider_float_param.set_from_normal(normal);
                        // println!("{}", self.h_slider_float_param.value());
                    },
                    ParamID::VSliderDB => {
                        self.v_slider_db_param.set_from_normal(normal);
                        // println!("{}", self.v_slider_db_param.value());
                    },
                    ParamID::KnobOctave => {
                        self.knob_octave_param.set_from_normal(normal);
                        // println!("{}", self.knob_octave_param.value());
                    },
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        
        // Create each parameter widget, passing in the current value of the
        // corresponding parameter.
        let h_slider_widget = HSlider::new(
            &mut self.h_slider_state,
            &self.h_slider_float_param,
            Message::ParamChanged,
        );
        let v_slider_widget = VSlider::new(
            &mut self.v_slider_state,
            &self.v_slider_db_param,
            Message::ParamChanged,
        );
        let knob_widget = Knob::new(
            &mut self.knob_state,
            &self.knob_octave_param,
            Message::ParamChanged,
        );

        // Push the widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(250)
            .max_height(350)
            .spacing(20)
            .padding(20)
            .align_items(Align::Center)
            .push(h_slider_widget)
            .push(v_slider_widget)
            .push(knob_widget)
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}