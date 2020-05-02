# iced_audio
Audio based widgets for the iced GUI library for Rust

## This crate is currently very alpha and incomplete.

## Run example with

```
cargo run --example all_widgets
```

or

```
cargo run --example all_widgets --release
```

## Widgets partially implemented
* HSlider - a horizontal slider

## Roadmap of planned widgets
### Inputs

* VSlider - vertical slider with optional notches
* HSlider - horizontal slider with optional notches
* HRangeSlider - a horizontal slider with two or more handles for controlling the automation range of a parameter.
* VTextureSlider - same as VSlider but using a texture
* HTextureSlider - same as HSlider but using a texture
* Knob - a rotating knob with optional notches
* TextureKnob - same as Knob but using a texture. May also have optional highlight and shadow layers.
* KnobAutoRange - an adjustable line around a Knob that represents the range of automation active on that parameter. Will have a unipolar and bipolar mode. May also have multiple of these widgets in a ring-like pattern like in the original Massive synthesizer.
* EnvelopeEditor - adjustable points connected by lines that represent an envelope / lfo. Lines can be straight or curved, and extra points can be added or removed.
* StepEditor - a row of vertical sliders for step automation
* ParEqEditor - a row of points connected by lines used to control parametric equalizers. These points can also be controlled with the scroll wheel to adjust Q value.
* Keys - piano keys that can be clicked with a mouse to play a synthesizer. Velocity is controlled by how low on the key the mouse was clicked at. It can be horizontal or vertical.
* TextureKeys - same as Keys but with Textures
* PitchWheel - like VSlider but the slider snaps back to the middle when the mouse is released.
* TexturePitchWheel - same as PitchWheel but using a texture. May also use an optional highlight and shadow layer.
* XYPad - a draggable point in a 2D square used to control 2 parameters at once

### Visualizers

* DBMeter - a meter that displays peak loudness of a signal. This can have optional colors for good headroom (green), low headroom (yellow), and peaking (red). It can have be either vertical or horizontal. It can also have an optional line showing the average loudness.
* ReductionMeter - a meter that displays the reduction of loudness in a signal. It can be either vertical or horizontal.
* KnobReductionMeter - same as Reduction meter but displays around a knob
* Oscilloscope - displays oscillations of an audio signal in a given time window
* Spectrometer - displays the amplitude of a range of frequencies from 20hz to 20000hz.
* SpectrometerGrid - a grid behind a Spectrometer that shows frequency on x axis and amplitude on y axis
* WaveformView - displays the peak amplitude of a signal over time. It can optionally be zoomed in and out of (like Audacity).
* PhaseMeter - a line that shows the phase correlation of an audio signal. It can be horizontal or vertical.
* Goniometer - displays a polar graph representing the stereo phase of an audio signal
* WavetableView - same as oscilloscope but specifically for rendering single waveforms instead of an audio signal


## Each continuous widget can accept one of four types of parameters:
* FloatParam - a linear range of f32 values
* IntParam - a discrete range of i32 values. This will cause the widget to "step" when moved.
* LogDBParam - a logarithmic range of decibel values. Values around 0 dB will increment slower than values farther away from 0 dB.
* OctaveParam - a logarithmic range of frequency values. Each octave in the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.