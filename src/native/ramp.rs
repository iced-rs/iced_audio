//! Display a ramp control that controls a [`NormalParam`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, NormalParam};
use crate::IntRange;

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
pub struct Ramp<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
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

impl<'a, Message, Renderer: self::Renderer> Ramp<'a, Message, Renderer> {
    /// Creates a new [`Ramp`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`Ramp`]
    ///   * a function that will be called when the [`Ramp`] is dragged.
    ///   * the [`RampDirection`] of the [`Ramp`], which tells if the ramp line
    /// should point `Up` (from `bottom-left` to `top-right`), or `Down` (from
    /// `top-left` to `bottom-right`)
    ///
    /// [`RampDirection`]: enum.RampDirection.html
    /// [`State`]: struct.State.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new<F>(
        state: &'a mut State,
        on_change: F,
        direction: RampDirection,
    ) -> Self
    where
        F: 'static + Fn(Normal) -> Message,
    {
        Ramp {
            state,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers {
                control: true,
                ..Default::default()
            },
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
        messages: &mut Vec<Message>,
        mut normal_delta: f32,
    ) {
        if self.state.pressed_modifiers.matches(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        let mut normal = self.state.continuous_normal - normal_delta;

        if normal < 0.0 {
            normal = 0.0;
        } else if normal > 1.0 {
            normal = 1.0;
        }

        self.state.continuous_normal = normal;

        self.state.normal_param.value = normal.into();

        messages.push((self.on_change)(self.state.normal_param.value));
    }
}

/// The local state of a [`Ramp`].
///
/// [`Ramp`]: struct.Ramp.html
#[derive(Debug, Copy, Clone)]
pub struct State {
    normal_param: NormalParam,
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
    /// * a [`NormalParam`] to assign to this widget. A [`NormalParam`] with a [`Normal`]
    /// value of `0.5` represents a straight line, `0.0` is curved downward all
    /// the way, and `1.0` is curved upward all the way.
    ///
    /// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new(normal_param: NormalParam) -> Self {
        Self {
            normal_param,
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normal_param.value.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }

    /// Set the normalized value of the [`Ramp`].
    pub fn set_normal(&mut self, normal: Normal) {
        self.normal_param.value = normal;
        self.continuous_normal = normal.into();
    }

    /// Get the normalized value of the [`Ramp`].
    pub fn normal(&self) -> Normal {
        self.normal_param.value
    }

    /// Set the normalized default value of the [`Ramp`].
    pub fn set_default(&mut self, normal: Normal) {
        self.normal_param.default = normal;
    }

    /// Get the normalized default value of the [`Ramp`].
    pub fn default(&self) -> Normal {
        self.normal_param.default
    }

    /// Snap the visible value of the [`Ramp`] to the nearest value
    /// in the integer range.
    ///
    /// # Example
    ///
    /// ```
    /// use iced_audio::{ramp, IntRange};
    ///
    /// let mut state = ramp::State::new(Default::default());
    /// let int_range = IntRange::new(0, 10);
    ///
    /// state.snap_visible_to(&int_range);
    ///
    /// ```
    pub fn snap_visible_to(&mut self, range: &IntRange) {
        self.normal_param.value = range.snapped(self.normal_param.value);
    }

    /// Is the [`Ramp`] currently in the dragging state?
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Ramp<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
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
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => {
                    if self.state.is_dragging {
                        let bounds_height = layout.bounds().height;

                        if bounds_height > 0.0 {
                            let normal_delta = (cursor_position.y
                                - self.state.prev_drag_y)
                                / bounds_height
                                * self.scalar;

                            self.state.prev_drag_y = cursor_position.y;

                            self.move_virtual_slider(messages, normal_delta);

                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::WheelScrolled { delta } => {
                    if self.wheel_scalar == 0.0 {
                        return event::Status::Ignored;
                    }

                    if layout.bounds().contains(cursor_position) {
                        let lines = match delta {
                            iced_native::mouse::ScrollDelta::Lines {
                                y,
                                ..
                            } => y,
                            iced_native::mouse::ScrollDelta::Pixels {
                                y,
                                ..
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

                            self.move_virtual_slider(messages, normal_delta);

                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if layout.bounds().contains(cursor_position) {
                        let click = mouse::Click::new(
                            cursor_position,
                            self.state.last_click,
                        );

                        match click.kind() {
                            mouse::click::Kind::Single => {
                                self.state.is_dragging = true;
                                self.state.prev_drag_y = cursor_position.y;
                            }
                            _ => {
                                self.state.is_dragging = false;

                                self.state.normal_param.value =
                                    self.state.normal_param.default;

                                messages.push((self.on_change)(
                                    self.state.normal_param.value,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);

                        return event::Status::Captured;
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.normal_param.value.as_f32();

                    return event::Status::Captured;
                }
                _ => {}
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;

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
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.state.normal_param.value,
            self.state.is_dragging,
            &self.style,
            self.direction,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
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
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Ramp<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        ramp: Ramp<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(ramp)
    }
}
