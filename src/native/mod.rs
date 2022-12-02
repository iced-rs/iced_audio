//! A renderer-agnostic native GUI runtime for Iced Audio.

pub mod h_slider;
pub mod knob;
pub mod mod_range_input;
pub mod ramp;
pub mod text_marks;
pub mod tick_marks;
pub mod v_slider;
pub mod xy_pad;

#[doc(no_inline)]
pub use h_slider::HSlider;
#[doc(no_inline)]
pub use knob::Knob;
#[doc(no_inline)]
pub use mod_range_input::ModRangeInput;
#[doc(no_inline)]
pub use ramp::Ramp;
#[doc(no_inline)]
pub use v_slider::VSlider;
#[doc(no_inline)]
pub use xy_pad::XYPad;

/// Moved status for the virtual sliders.
///
/// This allows tracking the virtual slider actual movements
/// thus preventing some events from unnecessary being emitted.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum SliderStatus {
    Moved,
    #[default]
    Unchanged,
}

impl SliderStatus {
    /// Sets the slider as moved.
    pub(crate) fn moved(&mut self) {
        *self = SliderStatus::Moved;
    }

    /// Whether the slider was moved.
    pub(crate) fn was_moved(self) -> bool {
        matches!(self, SliderStatus::Moved)
    }
}
