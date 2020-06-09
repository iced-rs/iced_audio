//! Display a ramp control that controls a [`Param`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`Param`]: ../core/param/trait.Param.html

use std::fmt::Debug;

use iced_native::{
    input::{keyboard, mouse, ButtonState},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, Param};

static DEFAULT_WIDTH: u16 = 40;
static DEFAULT_HEIGHT: u16 = 20;
static DEFAULT_SCALAR: f32 = 0.008;
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

/// A ramp GUI widget that controls a [`Param`]. It is usually used to
/// represent the easing of a parameter between two points in time.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`Ramp`]: struct.Ramp.html
#[allow(missing_debug_implementations)]
pub struct Ramp<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    state: &'a mut State<ID>,
    on_change: Box<dyn Fn(ID) -> Message>,
    scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    width: Length,
    height: Length,
    style: Renderer::Style,
    direction: RampDirection,
}

impl<'a, Message, Renderer: self::Renderer, ID> Ramp<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
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
        state: &'a mut State<ID>,
        on_change: F,
        direction: RampDirection,
    ) -> Self
    where
        F: 'static + Fn(ID) -> Message,
    {
        Ramp {
            state,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
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
    /// The default value is `0.008`
    ///
    /// [`Ramp`]: struct.Ramp.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets the modifier keys of the [`Ramp`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the Ramps while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Ramp::scalar()` (which the default is `0.008`).
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
}

/// The local state of a [`Ramp`].
///
/// [`Ramp`]: struct.Ramp.html
#[derive(Debug, Copy, Clone)]
pub struct State<ID: Debug + Copy + Clone> {
    /// The [`Param`] assigned to this widget
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    pub param: Param<ID>,
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl<ID: Debug + Copy + Clone> State<ID> {
    /// Creates a new [`Ramp`] state.
    ///
    /// It expects:
    /// * a [`Param`] to assign to this widget. A [`Param`] with a [`Normal`]
    /// value of `0.5` represents a straight line, `0.0` is curved downward all
    /// the way, and `1.0` is curved upward all the way.
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new(param: Param<ID>) -> Self {
        Self {
            param,
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: param.normal.value(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for Ramp<'a, Message, Renderer, ID>
where
    Renderer: self::Renderer,
    ID: Debug + Copy + Clone,
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
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state,
            }) => match state {
                ButtonState::Pressed => {
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

                                self.state.param.normal =
                                    self.state.param.default_normal;

                                messages.push((self.on_change)(
                                    self.state.param.id,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.param.normal.value();
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging && cursor_position.y != -1.0 {
                    let mut movement_y = (cursor_position.y
                        - self.state.prev_drag_y)
                        * self.scalar;

                    if self.state.pressed_modifiers.matches(self.modifier_keys)
                    {
                        movement_y *= self.modifier_scalar;
                    }

                    let mut normal = self.state.continuous_normal - movement_y;

                    if normal < 0.0 {
                        normal = 0.0;
                    } else if normal > 1.0 {
                        normal = 1.0;
                    }

                    self.state.continuous_normal = normal;
                    self.state.prev_drag_y = cursor_position.y;

                    self.state.param.normal = normal.into();

                    messages.push((self.on_change)(self.state.param.id));
                }
            }
            Event::Keyboard(keyboard::Event::Input { modifiers, .. }) => {
                self.state.pressed_modifiers = modifiers;
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.state.param.normal,
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

impl<'a, Message, Renderer, ID> From<Ramp<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        ramp: Ramp<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(ramp)
    }
}
