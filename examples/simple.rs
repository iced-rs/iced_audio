// Import iced modules.
use iced::{
    Align, Column, Container, Element, Length, Sandbox, Settings, Text,
};
// Import iced_audio modules.
use iced_audio::{
    h_slider, knob, v_slider, xy_pad, DBRange, FloatRange, FreqRange, HSlider,
    IntRange, Knob, TickMark, TickMarkGroup, TickMarkTier, VSlider, XYPad,
};

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    HSliderInt,
    VSliderDB,
    KnobFreq,
    XYPadFloatX,
    XYPadFloatY,
}

// The message when a parameter widget is moved by the user
#[derive(Debug, Clone)]
pub enum Message {
    ParamMoved(ParamID),
}

pub fn main() {
    App::run(Settings::default())
}

pub struct App {
    // The ranges handle converting the input/output of a parameter to and from
    // a usable value.
    //
    // There are 4 options available for a range:
    //
    // * FloatRange - a linear range of f32 values
    // * IntRange - a discrete range of i32 values. This will cause the widget
    // to "step" when moved.
    // * DBRange - a logarithmic range of decibel values. Values around 0 dB
    // will increment slower than values farther away from 0 dB.
    // * FreqRange - a logarithmic range of frequency values. Each octave in
    // the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.
    //
    float_range: FloatRange,
    int_range: IntRange,
    db_range: DBRange,
    freq_range: FreqRange,

    // The states of the widgets that will control the parameters.
    //
    // The `ID` can be any user-defined type such as an enum, i32, u32, String, etc.
    //
    h_slider_state: h_slider::State<ParamID>,
    v_slider_state: v_slider::State<ParamID>,
    knob_state: knob::State<ParamID>,
    xy_pad_state: xy_pad::State<ParamID>,

    // A group of tick marks with their size and position.
    center_tick_mark: TickMarkGroup,

    output_text: String,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        // Initalize each range:
        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 10);
        let db_range = DBRange::new(-12.0, 12.0, 0.5.into());
        let freq_range = FreqRange::default();

        App {
            // Add the ranges.
            float_range,
            int_range,
            db_range,
            freq_range,

            // Initialize the state of the widgets with a parameter than has an ID, value,
            // and default value.
            h_slider_state: h_slider::State::new(int_range.create_param(
                ParamID::HSliderInt,
                5,
                5,
            )),
            v_slider_state: v_slider::State::new(
                db_range.create_param_default(ParamID::VSliderDB),
            ),
            knob_state: knob::State::new(freq_range.create_param(
                ParamID::KnobFreq,
                1000.0,
                1000.0,
            )),
            xy_pad_state: xy_pad::State::new(
                float_range.create_param_default(ParamID::XYPadFloatX),
                float_range.create_param_default(ParamID::XYPadFloatY),
            ),

            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: vec![TickMark::center(TickMarkTier::Two)].into(),

            output_text: "Move a widget!".into(),
        }
    }

    fn title(&self) -> String {
        format!("Simple Example - Iced Audio")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ParamMoved(id) => {
                // Retrieve the value by mapping the normal of the parameter
                // to the corresponding range.
                //
                // Now do something useful with that value!
                //
                match id {
                    ParamID::HSliderInt => {
                        // Integer ranges must be snapped to make the widget "step"
                        // when moved.
                        self.int_range
                            .snap_normal(&mut self.h_slider_state.param.normal);

                        let value = self
                            .int_range
                            .to_value(self.h_slider_state.param.normal);
                        self.output_text = format!("{:?}: {}", id, value);
                    }
                    ParamID::VSliderDB => {
                        let value = self
                            .db_range
                            .to_value(self.v_slider_state.param.normal);
                        self.output_text = format!("{:?}: {:.3}", id, value);
                    }
                    ParamID::KnobFreq => {
                        let value = self
                            .freq_range
                            .to_value(self.knob_state.param.normal);
                        self.output_text = format!("{:?}: {:.3}", id, value);
                    }
                    ParamID::XYPadFloatX => {
                        let value = self
                            .float_range
                            .to_value(self.xy_pad_state.param_x.normal);
                        self.output_text = format!("{:?}: {:.3}", id, value);
                    }
                    ParamID::XYPadFloatY => {
                        let value = self
                            .float_range
                            .to_value(self.xy_pad_state.param_y.normal);
                        self.output_text = format!("{:?}: {:.3}", id, value);
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Create each parameter widget, passing in the current state of the widget.
        let h_slider_widget =
            HSlider::new(&mut self.h_slider_state, Message::ParamMoved)
                // Add the tick mark group to this widget.
                .tick_marks(&self.center_tick_mark);

        let v_slider_widget =
            VSlider::new(&mut self.v_slider_state, Message::ParamMoved)
                .tick_marks(&self.center_tick_mark);

        let knob_widget = Knob::new(&mut self.knob_state, Message::ParamMoved);

        let xy_pad_widget =
            XYPad::new(&mut self.xy_pad_state, Message::ParamMoved);

        // Push the widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(250)
            .max_height(400)
            .spacing(20)
            .padding(20)
            .align_items(Align::Center)
            .push(h_slider_widget)
            .push(v_slider_widget)
            .push(knob_widget)
            .push(xy_pad_widget)
            .push(
                Container::new(Text::new(&self.output_text))
                    .width(Length::Fill),
            )
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
