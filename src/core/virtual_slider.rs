use std::time::{Duration, Instant};

use iced_core::{
    Event, Shell, keyboard, mouse, touch,
    window::{self, RedrawRequest},
};

use super::{Normal, NormalParam};

pub const DEFAULT_SCALAR: f32 = 0.00385;
pub const DEFAULT_WHEEL_SCALAR: f32 = 0.01;
pub const DEFAULT_MODIFIER_SCALAR: f32 = 0.02;
pub const DEFAULT_SCROLL_WHEEL_TIMEOUT_SECS: f32 = 0.25;

/// The configuration of a [`VirtualSlider`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// A scalar which adjusts how fast the [`Normal`] value will change for each logical
    /// pixel the mouse moves.
    ///
    /// The default value is `0.00385`
    pub drag_scalar: f32,

    /// A scalar which adjusts how fast the [`Normal`] value will change for each line
    /// scrolled by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    pub wheel_scalar: f32,

    /// A scalar applied to the movement speed of the virtual slider when the user drags
    /// the knobs while holding down the fine tune modifier key.
    ///
    /// For example, a scalar of `0.5` will cause the knob to turn half as fast when the
    /// modifier key is down.
    ///
    /// The default value is `0.02`.
    pub fine_tune_scalar: f32,

    /// The modifier key/keys to use for fine-tune dragging.
    ///
    /// The default key is `Ctrl`.
    pub fine_tune_modifiers: keyboard::Modifiers,

    /// How long to wait (in seconds) after a scroll wheel event to consider the gesture to
    /// be finished.
    ///
    /// Set this to `0.0` to disable the timeout (the gesture end message will be sent
    /// immediately).
    ///
    /// The default is "0.25".
    pub scroll_wheel_timeout_seconds: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            drag_scalar: 0.00385,
            wheel_scalar: 0.01,
            fine_tune_scalar: 0.02,
            fine_tune_modifiers: keyboard::Modifiers::CTRL,
            scroll_wheel_timeout_seconds: DEFAULT_SCROLL_WHEEL_TIMEOUT_SECS,
        }
    }
}

/// The current state of the user gesturing this widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    /// The gesture has just started. (i.e. the user has just pressed down on this widget
    /// to begin dragging it).
    ///
    /// This will always be sent before [`Gesture::Gesturing`]/`[`Gesture::GestureEnd`].
    GestureStart,
    /// The user is currently gesturing the widget and this is the new normalized value
    /// of the parameter.
    Gesturing(Normal),
    /// The user has finished gesturing the widget. This will always be sent exactly
    /// once after the last [`Gesture::Gesturing`] was sent and before the next
    /// [`Gesture::GestureStart`] is sent.
    GestureEnd,
}

impl Gesture {
    pub fn new_normal(&self) -> Option<Normal> {
        if let Self::Gesturing(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

/// The [`State`](iced_core::widget::tree::State) of a [`VirtualSlider`].
pub struct State {
    is_dragging: bool,
    prev_drag_pos: f32,
    prev_normal: Normal,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
    last_sent_gesture: Gesture,
    last_scroll_wheel_gesture_instant: Option<Instant>,
    hovered: bool,
}

impl State {
    pub fn new(param_normal: Normal) -> Self {
        Self {
            is_dragging: false,
            prev_drag_pos: 0.0,
            prev_normal: param_normal,
            continuous_normal: param_normal.as_f32(),
            pressed_modifiers: keyboard::Modifiers::NONE,
            last_click: None,
            last_sent_gesture: Gesture::GestureEnd,
            last_scroll_wheel_gesture_instant: None,
            hovered: false,
        }
    }

    pub fn is_gesturing(&self) -> bool {
        self.last_sent_gesture != Gesture::GestureEnd
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UpdateStatus {
    pub param_changed: bool,
    pub hover_state_changed: bool,
    pub gesturing_state_changed: bool,
}

impl UpdateStatus {
    pub fn should_redraw(&self) -> bool {
        self.param_changed || self.hover_state_changed || self.gesturing_state_changed
    }
}

/// The shared input logic for a "virtual slider" widget.
///
/// The input logic for a "virtual slider" works as follows:
/// (TODO)
pub struct VirtualSlider<'a, Message> {
    /// The configuration of this virtual slider.
    pub config: Config,
    on_gesture: Option<Box<dyn 'a + FnMut(Gesture) -> Message>>,
    param: NormalParam,
}

impl<'a, Message> VirtualSlider<'a, Message> {
    pub fn new(normal_param: NormalParam) -> Self {
        Self {
            param: normal_param,
            on_gesture: None,
            config: Config::default(),
        }
    }

    /// Sets the message to emit when the user gestures this widget.
    pub fn set_on_gesture(&mut self, on_gesture: impl 'a + FnMut(Gesture) -> Message) {
        self.on_gesture = Some(Box::new(on_gesture));
    }

    pub fn param(&self) -> &NormalParam {
        &self.param
    }

    /// A method that custom virtual slider widgets can call to implement
    /// virtual slider input logic.
    ///
    /// * `state` - The state of this virtual slider. (Store this in your widget's
    ///   tree state).
    /// * `cursor_is_over` - Whether or not the cursor is currently over the input
    ///   region of this widget. Typically this will be `cursor.is_over(layout.bounds())`,
    ///   but this can be customized to only be a portion of the widget's bounds.
    /// * `drag_horizontally` - If `true`, then the use drags horizontally to modify
    ///   this widget instead of vertically.
    /// * `drag_horizontally` - If `true`, then the speed of dragging/mouse wheel
    ///   scrolling will be halved. This is useful for some widgets such as the modulation
    ///   input range widget.
    /// * `event` - The event passed in to the widget's `update` method.
    /// * `cursor` - The cursor passed in to the widget's `update` method.
    /// * `shell` - The shell passed in to the widget's `update` method.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        state: &mut State,
        cursor_is_over: bool,
        drag_horizontally: bool,
        halve_speed: bool,
        event: &Event,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) -> UpdateStatus {
        // Update state if the value was modified outside of the widget.
        if !state.is_dragging && state.prev_normal != self.param.normal {
            state.prev_normal = self.param.normal;
            state.continuous_normal = self.param.normal.as_f32();
        }

        let mut capture_event = false;
        let mut status = UpdateStatus::default();

        if cursor_is_over && !state.hovered {
            state.hovered = true;
            status.hover_state_changed = true;
        } else if !cursor_is_over && state.hovered {
            state.hovered = false;
            status.hover_state_changed = true;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.is_dragging {
                    let (pos, delta) = if drag_horizontally {
                        (position.x, state.prev_drag_pos - position.x)
                    } else {
                        (position.y, position.y - state.prev_drag_pos)
                    };

                    state.prev_drag_pos = pos;

                    let drag_scalar = if halve_speed {
                        self.config.drag_scalar * 0.5
                    } else {
                        self.config.drag_scalar
                    };

                    self.move_virtual_slider(state, delta * drag_scalar, shell, &mut status);

                    capture_event = true;
                } else if cursor_is_over {
                    capture_event = true;
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.config.wheel_scalar == 0.0 {
                    return status;
                }

                if cursor_is_over {
                    let lines = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y,
                        mouse::ScrollDelta::Pixels { y, .. } => {
                            if *y > 0.0 {
                                1.0
                            } else if *y < 0.0 {
                                -1.0
                            } else {
                                0.0
                            }
                        }
                    };

                    if lines != 0.0 {
                        let wheel_scalar = if halve_speed {
                            self.config.wheel_scalar * 0.5
                        } else {
                            self.config.wheel_scalar
                        };

                        let normal_delta = -lines * wheel_scalar;

                        self.move_virtual_slider(state, normal_delta, shell, &mut status);

                        if status.param_changed {
                            if self.config.scroll_wheel_timeout_seconds > 0.0 {
                                let now = Instant::now();
                                let timeout_instant = now
                                    + Duration::from_secs_f32(
                                        self.config.scroll_wheel_timeout_seconds,
                                    );

                                // Wait for the `RedrawRequested` event to send the gesture end message.
                                state.last_scroll_wheel_gesture_instant = Some(timeout_instant);

                                shell.request_redraw_at(RedrawRequest::At(timeout_instant));
                            } else if !state.is_dragging {
                                self.end_gesture(state, shell, &mut status);
                            }
                        }
                    }

                    capture_event = true;
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                if let Some(timeout_instant) = state.last_scroll_wheel_gesture_instant {
                    if *now >= timeout_instant {
                        self.end_gesture(state, shell, &mut status);
                    } else {
                        shell.request_redraw_at(timeout_instant);
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor_is_over && let Some(cursor_position) = cursor.position() {
                    let click =
                        mouse::Click::new(cursor_position, mouse::Button::Left, state.last_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            state.is_dragging = true;
                            state.prev_drag_pos = if drag_horizontally {
                                cursor_position.x
                            } else {
                                cursor_position.y
                            };

                            if let Gesture::GestureEnd = &state.last_sent_gesture {
                                if let Some(on_gesture) = &mut self.on_gesture {
                                    shell.publish((on_gesture)(Gesture::GestureStart));
                                }
                                state.last_sent_gesture = Gesture::GestureStart;
                            }
                        }
                        _ => {
                            self.set_param_value(
                                self.param.default.as_f32(),
                                state,
                                shell,
                                &mut status,
                            );
                            self.end_gesture(state, shell, &mut status);
                        }
                    }

                    state.last_click = Some(click);

                    capture_event = true;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                self.end_gesture(state, shell, &mut status);
                capture_event = true;
            }
            Event::Window(window::Event::Unfocused) => {
                self.end_gesture(state, shell, &mut status);
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    capture_event = true;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    capture_event = true;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = *modifiers;
                    capture_event = true;
                }
            },
            _ => {}
        }

        if capture_event {
            shell.capture_event();
        }

        status
    }

    fn move_virtual_slider(
        &mut self,
        state: &mut State,
        mut normal_delta: f32,
        shell: &mut Shell<'_, Message>,
        status: &mut UpdateStatus,
    ) {
        if state
            .pressed_modifiers
            .contains(self.config.fine_tune_modifiers)
        {
            normal_delta *= self.config.fine_tune_scalar;
        }

        self.set_param_value(state.continuous_normal - normal_delta, state, shell, status)
    }

    fn set_param_value(
        &mut self,
        value: f32,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        status: &mut UpdateStatus,
    ) {
        let prev_value = self.param.normal;
        self.param.normal.set(value);
        state.continuous_normal = self.param.normal.as_f32();

        if (self.param.normal.as_f32() - prev_value.as_f32()).abs() <= f32::EPSILON {
            return;
        }
        status.param_changed = true;

        if let Gesture::GestureEnd = &state.last_sent_gesture {
            // Send a GestureStart message first.
            if let Some(on_gesture) = &mut self.on_gesture {
                shell.publish((on_gesture)(Gesture::GestureStart));
            }
            state.last_sent_gesture = Gesture::GestureStart;
            status.gesturing_state_changed = true;
        }

        if let Some(on_gesture) = &mut self.on_gesture {
            shell.publish((on_gesture)(Gesture::Gesturing(self.param.normal)));
        }
        state.last_sent_gesture = Gesture::Gesturing(self.param.normal);
    }

    fn end_gesture(
        &mut self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        status: &mut UpdateStatus,
    ) {
        state.is_dragging = false;
        state.last_scroll_wheel_gesture_instant = None;

        if state.last_sent_gesture == Gesture::GestureEnd {
            return;
        }
        status.gesturing_state_changed = true;

        if let Some(on_gesture) = &mut self.on_gesture {
            shell.publish((on_gesture)(Gesture::GestureEnd));
        }
        state.last_sent_gesture = Gesture::GestureEnd;
    }
}
