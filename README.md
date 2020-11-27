# Iced Audio
[![Crates.io](https://img.shields.io/crates/v/iced_audio.svg)](https://crates.io/crates/iced_audio)
[![License](https://img.shields.io/crates/l/iced_audio.svg)](https://github.com/BillyDM/iced_audio/blob/master/LICENSE)
[![project chat](https://img.shields.io/badge/chat-on_zulip-brightgreen.svg)](https://iced.zulipchat.com)

An extension to the [Iced] GUI library with useful widgets for audio applications such as VST / LV2 plugins.

<div align="center">
    <img src="/screenshots/HSliders.png">
    <img src="/screenshots/Modulation_Ranges.png">
    <img src="/screenshots/XYPads.png">
</div>

[more screenshots]

## Widgets implemented
### Inputs
* [x] `HSlider` - Horizontal Slider
* [x] `VSlider` - Vertical Slider
* [x] `Knob` - A classic knob widget. (no texture style yet)
* [x] `Ramp` - Ramp used to control the easing between two points in time
* [x] `XYPad`- XY Pad for controlling two parameters at once
* [x] `ModRangeInput` - A dot used to control the range of modulation for a parameter. Styles that add visual feedback of the modulation range exist for the `HSlider`, `VSlider`, and `Knob` widgets.

Take a look at the [roadmap] for a list of planned widgets.

## Each parameter can be mapped to one of four ranges:
* `FloatRange` - a linear range of f32 values
* `IntRange` - a discrete range of i32 values. This will cause the widget to "step" when moved.
* `LogDBRange` - a logarithmic range of decibel values. Values around 0 dB will increment slower than values farther away from 0 dB.
* `FreqRange` - a logarithmic range of frequency values. Each octave in the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.

## Run examples with

```
cargo run --package inputs_tour --release
cargo run --package simple --release
```

## Installation

Add `iced` and `iced_audio` as dependencies in your `Cargo.toml`:
```toml
iced = { version = "0.2", features = ["image"] }
iced_audio = "0.5"
```
Or if you want to use the GitHub version of `iced`:
```toml
iced = { git = "https://github.com/hecrj/iced", branch = "master", features=["image"] }
iced_audio = { git = "https://github.com/BillyDM/iced_audio", branch = "iced_git" }
```
You may emit `features = ["image"]` if you do not plan on using images.
__Both Iced Audio and [Iced] move fast and the `main` and `iced_git` branch can contain breaking changes!__ If
you want to learn about a specific release, check out [the release list].

## Simple Usage Example
This crate assumes you know the basics of how to use [Iced]. If you haven't alreay, please check it out [here].
```rust
// Import iced modules.
use iced::{
    Align, Column, Container, Element, Length, Sandbox, Settings, Text,
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
                // Integer ranges must be snapped to make the widget "step" when moved.
                self.int_range
                    .snap(&mut self.h_slider_state.normal_param.value);

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

        let knob_widget = Knob::new(&mut self.knob_state, Message::KnobFreq);

        let xy_pad_widget =
            XYPad::new(&mut self.xy_pad_state, Message::XYPadFloat);

        // Push the widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(300)
            .max_height(500)
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


```

## VST / LV2 / AU Plugins
If you wish to use `iced_audio` for audio plugins, check out my other repos.
__Please not these are experimental and currently lacking many features.__
* [`iced_baseview`] - Run [Iced] using [`baseview`] as a backend.
* [`iced-baseplug-examples`] - Example audio plugins using [`baseplug`] as a plugin wrapper, [`iced_baseview`] as a GUI backend, and `iced_audio` widgets.

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
[more screenshots]: https://github.com/BillyDM/iced_audio/tree/master/screenshots
[roadmap]: https://github.com/BillyDM/iced_audio/tree/master/ROADMAP.md
[`iced_baseview`]: https://github.com/BillyDM/iced_baseview
[`baseview`]: https://github.com/RustAudio/baseview
[`baseplug`]: https://github.com/wrl/baseplug