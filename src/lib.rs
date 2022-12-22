//! Iced Audio is an extension to the [`Iced`] GUI library with useful widgets
//! for audio applications such as VST / LV2 plugins.
//!
//! # Installation
//!
//! Add `iced_audio` as dependency in your `Cargo.toml`:
//! ```toml
//! iced_audio = "0.7"
//! ```
//! Or if you want to use the GitHub version of `iced`:
//! ```toml
//! iced_audio = { git = "https://github.com/iced-rs/iced_audio", branch = "iced_git" }
//! ```
//! __Both Iced Audio and [`Iced`] move fast and the `main` and `iced_git` branch can contain breaking changes!__ If
//! you want to learn about a specific release, check out [the release list].
//!
//! # Simple Usage Example
//!
//! This crate assumes you know the basics of how to use [`Iced`]. If you
//! haven't already, please check it out [`here`].
//!
//! ```no_run
//! // Import iced modules.
//! use iced::widget::{column, container, text};
//! use iced::{Alignment, Element, Length, Sandbox, Settings};
//! // Import iced_audio modules.
//! use iced_audio::{
//!     tick_marks, FloatRange, FreqRange, HSlider, IntRange, Knob, LogDBRange,
//!     Normal, NormalParam, VSlider, XYPad,
//! };
//!
//! // The message when a parameter widget is moved by the user
//! #[derive(Debug, Clone)]
//! pub enum Message {
//!     HSliderInt(Normal),
//!     VSliderDB(Normal),
//!     KnobFreq(Normal),
//!     XYPadFloat(Normal, Normal),
//! }
//!
//! pub fn main() {
//!     App::run(Settings::default()).unwrap();
//! }
//!
//! pub struct App {
//!     // The ranges handle converting the input/output of a parameter to and from
//!     // a usable value.
//!     //
//!     // There are 4 built-in options available for a range:
//!     //
//!     // * FloatRange - a linear range of f32 values
//!     // * IntRange - a discrete range of i32 values. This will cause the widget
//!     // to "step" when moved.
//!     // * LogDBRange - a logarithmic range of decibel values. Values around 0 dB
//!     // will increment slower than values farther away from 0 dB.
//!     // * FreqRange - a logarithmic range of frequency values. Each octave in
//!     // the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.
//!     //
//!     float_range: FloatRange,
//!     int_range: IntRange,
//!     db_range: LogDBRange,
//!     freq_range: FreqRange,
//!
//!     // The parameters of the widgets.
//!     h_slider_param: NormalParam,
//!     v_slider_param: NormalParam,
//!     knob_param: NormalParam,
//!     xy_pad_x_param: NormalParam,
//!     xy_pad_y_param: NormalParam,
//!
//!     // A group of tick marks with their size and position.
//!     center_tick_mark: tick_marks::Group,
//!
//!     output_text: String,
//! }
//!
//! impl Sandbox for App {
//!     type Message = Message;
//!
//!     fn new() -> App {
//!         // Initalize each range:
//!         let float_range = FloatRange::default_bipolar();
//!         let int_range = IntRange::new(0, 10);
//!         let db_range = LogDBRange::new(-12.0, 12.0, Normal::CENTER);
//!         let freq_range = FreqRange::default();
//!
//!         App {
//!             // Add the ranges.
//!             float_range,
//!             int_range,
//!             db_range,
//!             freq_range,
//!
//!             // Initialize the state of the widgets with a normalized parameter
//!             // that has a value and a default value.
//!             h_slider_param: int_range.normal_param(5, 5),
//!             v_slider_param: db_range.default_normal_param(),
//!             knob_param: freq_range.normal_param(1000.0, 1000.0),
//!             xy_pad_x_param: float_range.default_normal_param(),
//!             xy_pad_y_param: float_range.default_normal_param(),
//!
//!             // Add a tick mark at the center position with the tier 2 size
//!             center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),
//!
//!             output_text: "Move a widget!".into(),
//!         }
//!     }
//!
//!     fn title(&self) -> String {
//!         format!("Simple Example - Iced Audio")
//!     }
//!
//!     fn update(&mut self, event: Message) {
//!         match event {
//!             // Retrieve the value by mapping the normalized value of the parameter
//!             // to the corresponding range.
//!             //
//!             // Now do something useful with that value!
//!             Message::HSliderInt(normal) => {
//!                 // Integer parameters must be snapped to make the widget "step" when moved.
//!                 self.h_slider_param.update(self.int_range.snapped(normal));
//!
//!                 let value = self.int_range.unmap_to_value(normal);
//!                 self.output_text = format!("HSliderInt: {}", value);
//!             }
//!             Message::VSliderDB(normal) => {
//!                 self.v_slider_param.update(normal);
//!
//!                 let value = self.db_range.unmap_to_value(normal);
//!                 self.output_text = format!("VSliderDB: {:.3}", value);
//!             }
//!             Message::KnobFreq(normal) => {
//!                 self.knob_param.update(normal);
//!
//!                 let value = self.freq_range.unmap_to_value(normal);
//!                 self.output_text = format!("KnobFreq: {:.2}", value);
//!             }
//!             Message::XYPadFloat(normal_x, normal_y) => {
//!                 self.xy_pad_x_param.update(normal_x);
//!                 self.xy_pad_y_param.update(normal_y);
//!
//!                 let value_x = self.float_range.unmap_to_value(normal_x);
//!                 let value_y = self.float_range.unmap_to_value(normal_y);
//!                 self.output_text =
//!                     format!("XYPadFloat: x: {:.2}, y: {:.2}", value_x, value_y);
//!             }
//!         }
//!     }
//!
//!     fn view(&self) -> Element<Message> {
//!         // Create each parameter widget, passing in the current state of the widget.
//!         let h_slider_widget =
//!             HSlider::new(self.h_slider_param, Message::HSliderInt)
//!                 // Add the tick mark group to this widget.
//!                 .tick_marks(&self.center_tick_mark);
//!
//!         let v_slider_widget =
//!             VSlider::new(self.v_slider_param, Message::VSliderDB)
//!                 .tick_marks(&self.center_tick_mark);
//!
//!         let knob_widget =
//!             Knob::new(self.knob_param, Message::KnobFreq);
//!
//!         let xy_pad_widget = XYPad::new(
//!             self.xy_pad_x_param,
//!             self.xy_pad_y_param,
//!             Message::XYPadFloat
//!         );
//!
//!         // Push the widgets into the iced DOM
//!         let content = column![
//!             h_slider_widget,
//!             v_slider_widget,
//!             knob_widget,
//!             xy_pad_widget,
//!             container(text(&self.output_text)).width(Length::Fill),
//!         ]
//!         .max_width(300)
//!         .spacing(20)
//!         .padding(20)
//!         .align_items(Alignment::Center);
//!
//!         container(content)
//!             .max_height(500)
//!             .width(Length::Fill)
//!             .height(Length::Fill)
//!             .center_x()
//!             .center_y()
//!             .into()
//!     }
//! }
//!
//! ```
//! [`Iced`]: https://github.com/iced-rs/iced
//! [`here`]: https://github.com/iced-rs/iced

#![deny(
    unused_results,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion
)]
#![deny(missing_docs, unused_results)]
#![forbid(unsafe_code, rust_2018_idioms)]
#![allow(
    clippy::needless_doctest_main,
    clippy::inherent_to_string,
    clippy::type_complexity
)]

pub mod core;
pub mod graphics;
pub mod native;
pub mod style;

#[doc(no_inline)]
pub use crate::core::*;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[doc(no_inline)]
    pub use crate::graphics::{
        h_slider, knob, mod_range_input, ramp, text_marks, tick_marks,
        v_slider, xy_pad,
    };

    #[doc(no_inline)]
    pub use {
        h_slider::HSlider, knob::Knob, mod_range_input::ModRangeInput,
        ramp::Ramp, v_slider::VSlider, xy_pad::XYPad,
    };
}

#[cfg(all(not(feature = "glow"), feature = "wgpu"))]
use iced_wgpu as renderer;

#[cfg(feature = "glow")]
use iced_glow as renderer;

use renderer::Renderer;

#[doc(no_inline)]
pub use platform::*;
