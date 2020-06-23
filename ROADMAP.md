I am currently taking a break from this project for now. I need to learn more DSP concepts to properly implement the `Oscilloscope` and `SpectrumAnalyzer`. I will get back to this project once I have done that. I will also keep `iced_audio` updated whenever `iced` is updated to the new version on crates.io. If you need anything please post an issue. Thank you for your patience!

# Roadmap

* [x] `HSlider` - horizontal slider with optional tick marks
* [x] `VSlider` - vertical slider with optional tick marks
* [x] `Knob` - a rotating knob with optional tick marks. Texture style may have optional highlight and shadow layers.
* [x] `XYPad` - a draggable point in a 2D square used to control 2 parameters at once
* [x] `Ramp` - a line that curves up and down while being dragged. It is used to represent the easing of a parameter between two points in time.
* [x] `DBMeter` - a meter that displays peak loudness of a signal. This can have optional colors for good headroom (green), low headroom (yellow), and peaking (red). It can be either vertical or horizontal. It can also have an optional line showing the peak loudness.
* [x] `ModRangeInput` - A dot used to control the range of modulation for a parameter. Styles for visual feedback of the modulation range exist for the `HSlider`, `VSlider`, and `Knob` widgets.
* [x] `ReductionMeter` - a meter that displays the reduction of loudness in a signal. It can be either vertical or horizontal. It can also have an optional line showing the average loudness.
* [x] `PhaseMeter` - a meter that displays the phase correlation of an audio signal. It can be horizontal or vertical.

## Widgets I will get to once I learn more DSP concepts

* [ ] `Oscilloscope` - displays oscillations of an audio signal in a given time window
* [ ] `SpectrumAnalyzer` - displays the amplitude of a range of frequencies from 20hz to 20000hz.

## Widgets I will get to at some point

* [ ] `EnvelopeEditor` - adjustable points connected by lines that represent automation / envelopes / lfo`s. Lines can be straight or curved. Extra points can be added or removed.
* [ ] `ADSREnvelope` - adjustable points connected by lines that represent an ADSR envelope. Can also have optional delay and hold points. The curve of each line may also optionally be controlled by dragging up and down on that line. This widget may be unnecessary though depending on how `EnvelopeEditor` is implemented.

## Widgets I want but may or may not be implemented depending on demand and the time I have

* [ ] `HRangeSlider` - a horizontal slider with two or more handles for controlling the modulation range of a parameter.
* [ ] `Keys` - piano keys that can be clicked with a mouse to play a synthesizer. Velocity is controlled by how low on the key the mouse was clicked at. It can be horizontal or vertical.
* [ ] `ModWheel` - like VSlider, but the Texture style is that of a mod wheel with optional highlight and shadow layers. Will also have a PitchWheel mode where it will automatically snap to the middle position when the mouse button is released.
* [ ] `ParEqEditor` - a row of points connected by lines used to control parametric equalizers. These points can also be controlled with the scroll wheel to adjust the Q value.
* [ ] `WaveformView` - displays the peak amplitude of a signal over time. It can optionally be zoomed in and out of (like Audacity).
* [ ] `Goniometer` - displays a polar graph representing the stereo phase of an audio signal
* [ ] `WavetableView` - same as oscilloscope but specifically for rendering single waveforms instead of an audio signal
* [ ] Extra styles for `HSlider`, `Vslider`, `Knob`, `EnvelopeEditor`, and `ADSREnvelope` that expose an animatable moving dot that displays the modulation that is happening in real time.