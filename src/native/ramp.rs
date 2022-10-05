//! Display a ramp control that controls a [`NormalParam`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced_native::widget::tree::{self, Tree};
use iced_native::{
    event, keyboard, layout, mouse, touch, Clipboard, Element, Event, Layout,
    Length, Point, Rectangle, Shell, Size, Widget,
};

use crate::core::{Normal, NormalParam};

static DEFAULT_WIDTH: u16 = 40;
static DEFAULT_HEIGHT: u16 = 20;
static DEFAULT_SCALAR: f32 = 0.00385;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// The direction of a [`Ramp`] widget.
#[derive(Debug, Copy, Clone)]
pub enum RampDirection {
    /// The line points upwards from `bottom-left` to `top-right`.
    Up,
    /// The line points downwards from `top-left` to `bottom-right`.
    Down,
}

impl Default for RampDirection {
    fn default() -> Self {
        RampDirection::Up
    }
}

/// A ramp GUI widget that controls a [`NormalParam`]. It is usually used to
/// represent the easing of a parameter between two points in time.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`Ramp`]: struct.Ramp.html
#[allow(missing_debug_implementations)]
pub struct Ramp<Message, Renderer: self::Renderer> {
    normal_param: NormalParam,
    on_change: Box<dyn Fn(Normal) -> Message>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    width: Length,
    height: Length,
    style: Renderer::Style,
    direction: RampDirection,
}

impl<Message, Renderer: self::Renderer> Ramp<Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    /// Creates a new [`Ramp`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`Ramp`]
    ///   * a function that will be called when the [`Ramp`] is dragged.
    ///   * the [`RampDirection`] of the [`Ramp`], which tells if the ramp line
    /// should point `Up` (from `bottom-left` to `top-right`), or `Down` (from
    /// `top-left` to `bottom-right`)
    ///
    /// [`RampDirection`]: enum.RampDirection.html
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new<F>(
        normal_param: NormalParam,
        on_change: F,
        direction: RampDirection,
    ) -> Self
    where
        F: 'static + Fn(Normal) -> Message,
    {
        Ramp {
            normal_param,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::from(Length::Units(DEFAULT_HEIGHT)),
            style: Renderer::Style::default(),
            direction,
        }
    }

    /// Sets the width of the [`Ramp`].
    /// The default width is `Length::from(Length::Units(30))`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Ramp`].
    /// The default height is `Length::from(Length::Units(20))`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`Ramp`].
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Ramp`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.00385`
    ///
    /// [`Ramp`]: struct.Ramp.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Ramp`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`Ramp`]: struct.Ramp.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the modifier keys of the [`Ramp`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the Ramps while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Ramp::scalar()` (which the default is `0.00385`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the ramp to move
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    fn move_virtual_slider(
        &mut self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        mut normal_delta: f32,
    ) {
        if normal_delta.abs() < f32::EPSILON {
            return;
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        self.normal_param.value =
            (state.continuous_normal - normal_delta).into();
        state.continuous_normal = self.normal_param.value.as_f32();

        shell.publish((self.on_change)(self.normal_param.value));
    }
}

/// The local state of a [`Ramp`].
///
/// [`Ramp`]: struct.Ramp.html
#[derive(Debug, Copy, Clone)]
struct State {
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
}

impl State {
    /// Creates a new [`Ramp`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`Ramp`].
    /// A [`Normal`] value of `0.5` represents a straight line,
    /// `0.0` is curved downward all the way,
    /// and `1.0` is curved upward all the way.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Ramp`]: struct.Ramp.html
    fn new(normal: Normal) -> Self {
        Self {
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Ramp<Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param.value))
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let state = state.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.is_dragging {
                    let normal_delta =
                        (cursor_position.y - state.prev_drag_y) * self.scalar;

                    state.prev_drag_y = cursor_position.y;

                    self.move_virtual_slider(state, shell, normal_delta);

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.wheel_scalar == 0.0 {
                    return event::Status::Ignored;
                }

                if layout.bounds().contains(cursor_position) {
                    let lines = match delta {
                        iced_native::mouse::ScrollDelta::Lines {
                            y, ..
                        } => y,
                        iced_native::mouse::ScrollDelta::Pixels {
                            y, ..
                        } => {
                            if y > 0.0 {
                                1.0
                            } else if y < 0.0 {
                                -1.0
                            } else {
                                0.0
                            }
                        }
                    };

                    if lines != 0.0 {
                        let normal_delta = -lines * self.wheel_scalar;

                        self.move_virtual_slider(state, shell, normal_delta);

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    let click =
                        mouse::Click::new(cursor_position, state.last_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            state.is_dragging = true;
                            state.prev_drag_y = cursor_position.y;
                            state.continuous_normal =
                                self.normal_param.value.as_f32();
                        }
                        _ => {
                            state.is_dragging = false;

                            self.normal_param.value = self.normal_param.default;
                            state.continuous_normal =
                                self.normal_param.default.as_f32();

                            shell.publish((self.on_change)(
                                self.normal_param.value,
                            ));
                        }
                    }

                    state.last_click = Some(click);

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.is_dragging {
                    state.is_dragging = false;
                    state.continuous_normal = self.normal_param.value.as_f32();

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                _ => {}
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let state = state.state.downcast_ref::<State>();
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.normal_param.value,
            state.is_dragging,
            &self.style,
            self.direction,
        )
    }
}

/// The renderer of a [`Ramp`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Ramp`] in your user interface.
///
/// [`Ramp`]: struct.Ramp.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Ramp`].
    ///
    /// It receives:
    ///   * the bounds of the [`Ramp`]
    ///   * the current cursor position
    ///   * the current normal of the [`Ramp`]
    ///   * whether the ramp is currently being dragged
    ///   * the style of the [`Ramp`]
    ///   * the direction of the ramp line of the [`Ramp`]
    ///
    /// [`Ramp`]: struct.Ramp.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style: &Self::Style,
        direction: RampDirection,
    );
}

impl<Message, Renderer> From<Ramp<Message, Renderer>>
    for Element<'_, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'static + self::Renderer,
{
    fn from(
        ramp: Ramp<Message, Renderer>,
    ) -> Element<'static, Message, Renderer> {
        Element::new(ramp)
    }
}
