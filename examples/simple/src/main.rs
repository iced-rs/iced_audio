// Import iced modules.
use iced::{
    Alignment, Column, Container, Element, Length, Sandbox, Settings, Text,
};
// Import iced_audio modules.
use iced_audio::{
    h_slider, knob, tick_marks, v_slider, xy_pad, FloatRange, FreqRange,
    HSlider, IntRange, Knob, LogDBRange, Normal, VSlider, XYPad,
};

// The message when a parameter widget is moved by the user
#[derive(Debug, Clone)]
pub enum Message {
    HSliderInt(Normal),
    VSliderDB(Normal),
    KnobFreq(Normal),
    XYPadFloat(Normal, Normal),
}

pub fn main() {
    App::run(Settings::default()).unwrap();
}

pub struct App {
    // The ranges handle converting the input/output of a parameter to and from
    // a usable value.
    //
    // There are 4 built-in options available for a range:
    //
    // * FloatRange - a linear range of f32 values
    // * IntRange - a discrete range of i32 values. This will cause the widget
    // to "step" when moved.
    // * LogDBRange - a logarithmic range of decibel values. Values around 0 dB
    // will increment slower than values farther away from 0 dB.
    // * FreqRange - a logarithmic range of frequency values. Each octave in
    // the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.
    //
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    // The states of the widgets that will control the parameters.
    h_slider_state: h_slider::State,
    v_slider_state: v_slider::State,
    knob_state: knob::State,
    xy_pad_state: xy_pad::State,

    // A group of tick marks with their size and position.
    center_tick_mark: tick_marks::Group,

    output_text: String,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        // Initalize each range:
        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 10);
        let db_range = LogDBRange::new(-12.0, 12.0, 0.5.into());
        let freq_range = FreqRange::default();

        App {
            // Add the ranges.
            float_range,
            int_range,
            db_range,
            freq_range,

            // Initialize the state of the widgets with a normalized parameter
            // that has a value and a default value.
            h_slider_state: h_slider::State::new(int_range.normal_param(5, 5)),
            v_slider_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),
            knob_state: knob::State::new(
                freq_range.normal_param(1000.0, 1000.0),
            ),
            xy_pad_state: xy_pad::State::new(
                float_range.default_normal_param(),
                float_range.default_normal_param(),
            ),

            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),

            output_text: "Move a widget!".into(),
        }
    }

    fn title(&self) -> String {
        format!("Simple Example - Iced Audio")
    }

    fn update(&mut self, event: Message) {
        match event {
            // Retrieve the value by mapping the normalized value of the parameter
            // to the corresponding range.
            //
            // Now do something useful with that value!
            Message::HSliderInt(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.h_slider_state.snap_visible_to(&self.int_range);

                let value = self.int_range.unmap_to_value(normal);
                self.output_text = format!("HSliderInt: {}", value);
            }
            Message::VSliderDB(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                self.output_text = format!("VSliderDB: {:.3}", value);
            }
            Message::KnobFreq(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.output_text = format!("KnobFreq: {:.2}", value);
            }
            Message::XYPadFloat(normal_x, normal_y) => {
                let value_x = self.float_range.unmap_to_value(normal_x);
                let value_y = self.float_range.unmap_to_value(normal_y);
                self.output_text =
                    format!("XYPadFloat: x: {:.2}, y: {:.2}", value_x, value_y);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Create each parameter widget, passing in the current state of the widget.
        let h_slider_widget =
            HSlider::new(&mut self.h_slider_state, Message::HSliderInt)
                // Add the tick mark group to this widget.
                .tick_marks(&self.center_tick_mark);

        let v_slider_widget =
            VSlider::new(&mut self.v_slider_state, Message::VSliderDB)
                .tick_marks(&self.center_tick_mark);

        let knob_widget = Knob::new(
            &mut self.knob_state,
            Message::KnobFreq,
            || None,
            || None,
        );

        let xy_pad_widget =
            XYPad::new(&mut self.xy_pad_state, Message::XYPadFloat);

        // Push the widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(300)
            .max_height(500)
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center)
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
