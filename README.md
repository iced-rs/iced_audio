# Iced Audio
[![Documentation](https://docs.rs/iced_audio/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/iced_audio.svg)](https://crates.io/crates/iced_audio)
[![License](https://img.shields.io/crates/l/iced_audio.svg)](https://github.com/BillyDM/iced_audio/blob/master/LICENSE)
[![project chat](https://img.shields.io/badge/chat-on_zulip-brightgreen.svg)](https://iced.zulipchat.com)

An extension to the [Iced] GUI library with useful widgets for audio applications such as VST / LV2 plugins.

<div align="center">
    <img src="/screenshots/HSliders.png" height="450px">
    <img src="/screenshots/Simple_Example.png" height="450px">
</div>

## Run examples with

```
cargo run --example tour --release
cargo run --example simple --release
```

## Widgets implemented
* [x] HSlider
* [x] VSlider
* [x] XYPad

## Widgets partially implemented
* [x] Knob - Due to current limitations of iced, only a simple flat circle style is implemented for now. There is also a known bug where input will stop when the mouse leaves the window under some conditions.

## Roadmap of planned widgets
### Inputs

* [x] HSlider - horizontal slider with optional tick marks
* [x] VSlider - vertical slider with optional tick marks
* [ ] HRangeSlider - a horizontal slider with two or more handles for controlling the automation range of a parameter.
* [x] Knob - a rotating knob with optional tick marks. Texture style may have optional highlight and shadow layers.
* [ ] KnobAutoRange - an adjustable line around a Knob that represents the range of automation active on that parameter. Will have a unipolar and bipolar mode. May also have multiple of these widgets in a ring-like pattern like in the original Massive synthesizer.
* [ ] EnvelopeEditor - adjustable points connected by lines that represent an envelope / lfo. Lines can be straight or curved, and extra points can be added or removed.
* [ ] StepEditor - a row of vertical sliders for step automation
* [ ] ParEqEditor - a row of points connected by lines used to control parametric equalizers. These points can also be controlled with the scroll wheel to adjust Q value.
* [ ] Keys - piano keys that can be clicked with a mouse to play a synthesizer. Velocity is controlled by how low on the key the mouse was clicked at. It can be horizontal or vertical.
* [ ] PitchWheel - like VSlider but the slider snaps back to the middle when the mouse is released. Texture style may have an optional highlight and shadow layer.
* [x] XYPad - a draggable point in a 2D square used to control 2 parameters at once

### Visualizers

* [ ] DBMeter - a meter that displays peak loudness of a signal. This can have optional colors for good headroom (green), low headroom (yellow), and peaking (red). It can have be either vertical or horizontal. It can also have an optional line showing the average loudness.
* [ ] ReductionMeter - a meter that displays the reduction of loudness in a signal. It can be either vertical or horizontal.
* [ ] KnobReductionMeter - same as Reduction meter but displays around a knob
* [ ] Oscilloscope - displays oscillations of an audio signal in a given time window
* [ ] Spectrometer - displays the amplitude of a range of frequencies from 20hz to 20000hz.
* [ ] SpectrometerGrid - a grid behind a Spectrometer that shows frequency on x axis and amplitude on y axis
* [ ] WaveformView - displays the peak amplitude of a signal over time. It can optionally be zoomed in and out of (like Audacity).
* [ ] PhaseMeter - a line that shows the phase correlation of an audio signal. It can be horizontal or vertical.
* [ ] Goniometer - displays a polar graph representing the stereo phase of an audio signal
* [ ] WavetableView - same as oscilloscope but specifically for rendering single waveforms instead of an audio signal

## Each input widget with a continuous output can accept one of four types of parameters
* FloatParam - a linear range of f32 values
* IntParam - a discrete range of i32 values. This will cause the widget to "step" when moved.
* LogDBParam - a logarithmic range of decibel values. Values around 0 dB will increment slower than values farther away from 0 dB.
* OctaveParam - a logarithmic range of frequency values. Each octave in the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.

## Installation
Add `iced` and `iced_audio` as dependencies in your `Cargo.toml`:
```
iced = { version = "0.1", features = ["image"] }
iced_audio = "0.1"
```
__Both Iced Audio and [Iced] move fast and the `master` branch can contain breaking changes!__ If
you want to learn about a specific release, check out [the release list].

## Simple Usage Example
This crate assumes you know the basics of how to use [Iced]. If you haven't alreay, please check it out [here].
```rust
// Import iced modules.
use iced::{
    Column, Container, Element, Length, Sandbox, Settings, Align
};
// Import iced_audio modules.
use iced_audio::{
    Normal, FloatParam, LogDBParam, OctaveParam, h_slider, HSlider,
    v_slider, VSlider, knob, Knob, xy_pad, XYPad, TickMarkGroup, TickMark,
    TickMarkTier
};

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    HSliderFloat,
    VSliderDB,
    KnobOctave,
    XYPadFloatX,
    XYPadFloatY,
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
    xy_pad_float_x_param: FloatParam<ParamID>,
    xy_pad_float_y_param: FloatParam<ParamID>,

    // The states of the parameter widgets that will control the parameters.
    h_slider_state: h_slider::State,
    v_slider_state: v_slider::State,
    knob_state: knob::State,
    xy_pad_state: xy_pad::State,

    // A group of tick marks with their size and position.
    center_tick_mark: TickMarkGroup,
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

        let xy_pad_float_x_param = FloatParam::<ParamID>::new(
            ParamID::XYPadFloatX , -1.0, 1.0, 0.0, 0.0);
        let xy_pad_float_y_param = FloatParam::<ParamID>::new(
            ParamID::XYPadFloatY , -1.0, 1.0, 0.0, 0.0);

        App {
            // Add the parameters.
            h_slider_float_param,
            v_slider_db_param,
            knob_octave_param,
            xy_pad_float_x_param,
            xy_pad_float_y_param,

            // Initialize the state of the widgets with the initial value
            // of the corresponding parameter.
            h_slider_state: h_slider::State::new(&h_slider_float_param),
            v_slider_state: v_slider::State::new(&v_slider_db_param),
            knob_state: knob::State::new(&knob_octave_param),
            xy_pad_state: xy_pad::State::new(
                &xy_pad_float_x_param, &xy_pad_float_y_param),
            
            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: vec![
                TickMark::center(TickMarkTier::Two)
            ].into(),
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
                    ParamID::XYPadFloatX => {
                        self.xy_pad_float_x_param.set_from_normal(normal);
                        // println!("{}", self.xy_pad_float_x_param.value());
                    },
                    ParamID::XYPadFloatY => {
                        self.xy_pad_float_y_param.set_from_normal(normal);
                        // println!("{}", self.xy_pad_float_y_param.value());
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
        )
        // Add the tick mark group to this widget.
        .tick_marks(&self.center_tick_mark);

        let v_slider_widget = VSlider::new(
            &mut self.v_slider_state,
            &self.v_slider_db_param,
            Message::ParamChanged,
        )
        .tick_marks(&self.center_tick_mark);

        let knob_widget = Knob::new(
            &mut self.knob_state,
            &self.knob_octave_param,
            Message::ParamChanged,
        );

        let xy_pad_widget = XYPad::new(
            &mut self.xy_pad_state,
            &self.xy_pad_float_x_param,
            &self.xy_pad_float_y_param,
            Message::ParamChanged,
        );

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
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
```

## Contributing / Feedback
Contributions are greatly appreciated! If you want to contribute, please
read the official [Iced] [contributing guidelines] for more details.

Feedback is also welcome! You can open an issue or, if you want to talk,
come chat to our [Zulip server]. Moreover, you can find me (and a bunch of
awesome folks) over the `#gui-and-ui` channels in
the [Rust Community Discord]. I go by `BillyDM#3892` there.

[Iced]: https://github.com/hecrj/iced
[documentation]: https://docs.rs/iced_audio/
[here]: https://github.com/hecrj/iced
[contributing guidelines]: https://github.com/hecrj/iced/blob/master/CONTRIBUTING.md
[Zulip server]: https://iced.zulipchat.com/
[Rust Community Discord]: https://bit.ly/rust-community
[the release list]: https://github.com/BillyDM/iced_audio/releases