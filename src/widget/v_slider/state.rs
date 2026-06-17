use crate::{Normal, SliderStatus};
use iced::{advanced::mouse, keyboard};

/// The local state of a [`VSlider`].
///
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone)]
pub struct State {
    pub dragging_status: Option<SliderStatus>,
    pub prev_drag_y: f32,
    pub prev_normal: Normal,
    pub continuous_normal: f32,
    pub pressed_modifiers: keyboard::Modifiers,
    pub last_click: Option<mouse::Click>,
    //tick_marks_cache: crate::graphics::tick_marks::PrimitiveCache,
    //text_marks_cache: crate::graphics::text_marks::PrimitiveCache,
}

impl State {
    /// Creates a new [`VSlider`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`VSlider`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new(normal: Normal) -> Self {
        Self {
            dragging_status: None,
            prev_drag_y: 0.0,
            prev_normal: normal,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
            //tick_marks_cache: Default::default(),
            //text_marks_cache: Default::default(),
        }
    }
}
