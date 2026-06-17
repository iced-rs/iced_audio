//! Display an interactive dot that controls an [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use crate::core::{Normal, NormalParam, SliderStatus};
use iced::{
    advanced::{
        graphics::core::{keyboard, touch},
        layout, mouse,
        renderer::{Quad, Style},
        widget::{tree, Tree},
        Clipboard, Layout, Renderer as _, Shell, Widget,
    },
    border::Radius,
    Border, Element, Event, Length, Rectangle, Renderer, Shadow, Size,
};

pub use crate::style::mod_range_input::{
    Appearance, CircleAppearance, InvisibleStyle, SquareAppearance, StyleSheet,
};

static DEFAULT_SIZE: f32 = 10.0;
static DEFAULT_SCALAR: f32 = 0.00385 / 2.0;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01 / 2.0;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// An interactive dot that controls an [`NormalParam`]
///
/// [`NormalParam`]: ../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct ModRangeInput<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    size: Length,
    enabled: bool,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme> ModRangeInput<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`ModRangeInput`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`ModRangeInput`]
    ///   * a function that will be called when the [`ModRangeInput`] is turned.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal) -> Message,
        <Theme as StyleSheet>::Style: Default,
    {
        ModRangeInput {
            normal_param,
            size: Length::Fixed(DEFAULT_SIZE),
            enabled: true,
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            style: Default::default(),
        }
    }

    /// Sets the grab message of the [`ModRangeInput`].
    /// This is called when the mouse grabs from the mod range input.
    ///
    /// Typically, the user's interaction with the mod range input starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`ModRangeInput`].
    /// This is called when the mouse is released from the mod range input.
    ///
    /// Typically, the user's interaction with the mod range input is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the mod range input's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the diameter of the [`ModRangeInput`]. The default size is
    /// `Length::from(Length::Fixed(31))`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`ModRangeInput`].
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`ModRangeInput`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.001925`
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`ModRangeInput`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.005`
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the modifier keys of the [`ModRangeInput`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the ModRangeInputs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `ModRangeInput::scalar()` (which the default is `0.001925`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the ModRangeInput to turn
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Enable/disable this widget.
    ///
    /// The default is `true`.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn move_virtual_slider(&mut self, state: &mut State, mut normal_delta: f32) -> SliderStatus {
        if normal_delta.abs() < f32::EPSILON {
            return SliderStatus::Unchanged;
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        self.normal_param
            .value
            .set_clipped(state.continuous_normal - normal_delta);
        state.continuous_normal = self.normal_param.value.as_f32();

        SliderStatus::Moved
    }

    fn maybe_fire_on_grab(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_grab.as_mut().and_then(|on_grab| on_grab()) {
            shell.publish(message);
        }
    }

    fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
        shell.publish((self.on_change)(self.normal_param.value));
    }

    fn maybe_fire_on_release(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_release.as_mut().and_then(|on_release| on_release()) {
            shell.publish(message);
        }
    }
}

/// The local state of an [`ModRangeInput`].
///
/// [`ModRangeInput`]: struct.ModRangeInput.html
#[derive(Debug, Copy, Clone)]
struct State {
    dragging_status: Option<SliderStatus>,
    prev_drag_y: f32,
    prev_normal: Normal,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
}

impl State {
    /// Creates a new [`ModRangeInput`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`ModRangeInput`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    fn new(normal: Normal) -> Self {
        Self {
            dragging_status: None,
            prev_drag_y: 0.0,
            prev_normal: normal,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for ModRangeInput<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param.value))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.size,
            height: self.size,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(self.size, self.size, Size::ZERO))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if !self.enabled {
            return;
        }

        let state = tree.state.downcast_mut::<State>();

        let is_over = cursor.is_over(layout.bounds());

        // Update state after a discontinuity
        if state.dragging_status.is_none() && state.prev_normal != self.normal_param.value {
            state.prev_normal = self.normal_param.value;
            state.continuous_normal = self.normal_param.value.as_f32();
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.dragging_status.is_some() {
                    let normal_delta = (position.y - state.prev_drag_y) * self.scalar;

                    state.prev_drag_y = position.y;

                    if self.move_virtual_slider(state, normal_delta).was_moved() {
                        self.fire_on_change(shell);

                        state
                            .dragging_status
                            .as_mut()
                            .expect("dragging_status taken")
                            .moved();
                    }
                }

                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.wheel_scalar == 0.0 {
                    return;
                }

                if is_over {
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
                        let normal_delta = -lines * self.wheel_scalar;

                        if self.move_virtual_slider(state, normal_delta).was_moved() {
                            if state.dragging_status.is_none() {
                                self.maybe_fire_on_grab(shell);
                            }

                            self.fire_on_change(shell);

                            if let Some(slider_status) = state.dragging_status.as_mut() {
                                // Widget was grabbed => keep it grabbed
                                slider_status.moved();
                            } else {
                                self.maybe_fire_on_release(shell);
                            }
                        }

                        shell.capture_event();
                        shell.request_redraw();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if is_over {
                    let cursor_position = cursor.position().unwrap();

                    let click =
                        mouse::Click::new(cursor_position, mouse::Button::Left, state.last_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            self.maybe_fire_on_grab(shell);

                            state.dragging_status = Some(Default::default());
                            state.prev_drag_y = cursor_position.y;
                        }
                        _ => {
                            // Reset to default

                            let prev_dragging_status = state.dragging_status.take();

                            if self.normal_param.value != self.normal_param.default {
                                if prev_dragging_status.is_none() {
                                    self.maybe_fire_on_grab(shell);
                                }

                                self.normal_param.value = self.normal_param.default;

                                self.fire_on_change(shell);

                                self.maybe_fire_on_release(shell);
                            } else if prev_dragging_status.is_some() {
                                self.maybe_fire_on_release(shell);
                            }
                        }
                    }

                    state.last_click = Some(click);

                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if let Some(slider_status) = state.dragging_status.take() {
                    if self.on_grab.is_some() || slider_status.was_moved() {
                        // maybe fire on release if `on_grab` is defined
                        // so as to terminate the action, regardless of the actual user movement.
                        self.maybe_fire_on_release(shell);
                    }

                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
                }
            },
            _ => {}
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = state.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let is_over = cursor.is_over(layout.bounds());

        let appearance = if !self.enabled {
            theme.disabled(&self.style)
        } else if state.dragging_status.is_some() {
            theme.dragging(&self.style)
        } else if is_over {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        match appearance {
            Appearance::Circle(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                let radius = bounds_size / 2.0;

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds_x,
                            y: bounds_y,
                            width: bounds_size,
                            height: bounds_size,
                        },
                        border: Border {
                            color: style.border_color,
                            width: style.border_width,
                            radius: Radius::new(radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    style.color,
                );
            }
            Appearance::Square(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds_x,
                            y: bounds_y,
                            width: bounds_size,
                            height: bounds_size,
                        },
                        border: Border {
                            color: style.border_color,
                            width: style.border_width,
                            radius: Radius::new(style.border_radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    style.color,
                );
            }
            Appearance::Invisible => {}
        };
    }
}

impl<'a, Message, Theme> From<ModRangeInput<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(mod_range_input: ModRangeInput<'a, Message, Theme>) -> Self {
        Self::new(mod_range_input)
    }
}
