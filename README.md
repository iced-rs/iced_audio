# Iced Audio
[![Documentation](https://docs.rs/iced_audio/badge.svg)](https://docs.rs/iced_audio)
[![Crates.io](https://img.shields.io/crates/v/iced_audio.svg)](https://crates.io/crates/iced_audio)
[![License](https://img.shields.io/crates/l/iced_audio.svg)](https://github.com/iced-rs/iced_audio/blob/main/LICENSE)
[![project chat](https://img.shields.io/badge/chat-on_zulip-brightgreen.svg)](https://iced.zulipchat.com)

An extension to the [Iced](https://github.com/iced-rs/iced) GUI library with useful widgets for audio applications such as VST3 / [CLAP] plugins.

<div align="center">
    <img src="/screenshots/HSliders.png">
    <img src="/screenshots/Modulation_Ranges.png">
    <img src="/screenshots/XYPads.png">
</div>

[more screenshots](https://github.com/iced-rs/iced_audio/tree/main/screenshots)

## Included Widgets
* `HSlider` - Horizontal Slider
* `VSlider` - Vertical Slider
* `Knob` - A classic knob widget
* `Ramp` - Ramp used to control the easing between two points in time
* `XYPad`- XY Pad for controlling two parameters at once
* `ModRangeInput` - A dot used to control the range of modulation for a parameter. Styles that add visual feedback of the modulation range exist for the `HSlider`, `VSlider`, and `Knob` widgets.

## Parameters
All widgets (with the exception of `XYPad`) borrow the same "virtual slider" input logic. Sliders, knobs, ramps, etc, are essentially just different ways to render the output of a virtual slider. You can also build your own custom virtual slider widgets that render whatever you want.

Each virtual slider operates on a normalized float value in the range `[0.0..1.0]`. Four mapping functions are included:

* `FloatRange` - a linear range of f32 values
* `IntRange` - a discrete range of i32 values. This will cause the widget to "step" when moved.
* `LogDBRange` - a logarithmic range of decibel values. Values around 0 dB will increment slower than values farther away from 0 dB.
* `FreqRange` - a logarithmic range of frequency values. Each octave in the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.

Custom mapping functions can be created as well.

## Audio plugins
If you wish to use `iced_audio` for audio plugins, check out these other crates:

* [nice-plug](https://codeberg.org/RustAudio/nice-plug) - Complete audio plugin development framework for VST3, [CLAP], and standalone targets with support for Iced.
* [clack-plugin](https://github.com/prokopyl/clack) - An easy way to create [CLAP] plugins in Rust.
* [iced_baseview](https://codeberg.org/RustAudio/iced_baseview) - Run Iced on the [baseview](https://github.com/RustAudio/baseview) windowing backend (use in conjuction with a crate like `clack-plugin`).

## Crate features

Note, you need to enable the `canvas` feature in iced for things to render properly. You also need to enable the `image` feature if you want to use the texture styles.

* `all` (default) - Enables all widgets
* `knob` (default) - Enables the knob widget
* `h_slider` (default) - Enables the horizontal slider widget
* `v_slider` (default) - Enables the vertical slider widget
* `ramp` (default) - Enables the ramp widget
* `xy_pad` (default) - Enables the XY pad widget

## Contributing / Feedback
Contributions are greatly appreciated! Before contributing, please read the official Iced [contributing guidelines](https://github.com/iced-rs/iced/blob/master/CONTRIBUTING.md).

[CLAP]: https://cleveraudio.org/
