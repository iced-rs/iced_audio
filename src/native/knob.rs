//! Display an interactive rotating knob that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use std::fmt::Debug;

use iced_native::{
    input::{mouse, ButtonState, keyboard},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, Param};

static DEFAULT_SIZE: u16 = 31;
static DEFAULT_SCALAR: f32 = 0.008;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A rotating knob GUI widget that controls a [`Param`]
///
/// [`Param`]: ../../core/param/trait.Param.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone
{
    state: &'a mut State,
    size: Length,
    id: ID,
    normal: Normal,
    default_normal: Normal,
    on_change: Box<dyn Fn((ID, Normal)) -> Message>,
    scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer, ID>
    Knob<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone
{
    /// Creates a new [`Knob`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`Knob`]
    ///   * a [`Param`] with the current and default values
    ///   * a function that will be called when the [`Knob`] is turned.
    ///   It receives the parameter's `ID` and the new [`Normal`] of the
    /// [`Knob`].
    /// `ID` is a user supplied type. It can be an `enum`, `u32`, `i32`,
    /// `String`, etc. Each parameter must have a unique `ID` value!
    ///
    /// [`State`]: struct.State.html
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<F>(
        state: &'a mut State,
        param: &impl Param<ID=ID>,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn((ID, Normal)) -> Message,
    {  
        Knob {
            state,
            size: Length::from(Length::Units(DEFAULT_SIZE)),
            id: param.id(),
            normal: param.normal(),
            default_normal: param.default_normal(),
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            style: Renderer::Style::default(),
        }
    }

    /// Sets the diameter of the [`Knob`]. The default size is
    /// `Length::from(Length::Units(31))`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.008`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets the modifier keys of the [`Knob`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the knobs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Knob::scalar()` (which the default is `0.008`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the knob to turn
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }
}

/// The local state of a [`Knob`].
///
/// [`Knob`]: struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct State{
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl State {
    /// Creates a new [`Knob`] state.
    ///
    /// It expects:
    /// * a [`Param`] with the initial value
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<ID>(param: &impl Param<ID=ID>) -> Self {
        Self {
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: param.normal().value(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}


impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for Knob<'a, Message, Renderer, ID>
where
    Renderer: self::Renderer,
    ID: Debug + Copy + Clone,
{
    fn width(&self) -> Length {
        self.size
    }

    fn height(&self) -> Length {
        self.size
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
            let limits = limits
            .width(self.size)
            .height(self.size);
        
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

                                messages.push((self.on_change)(
                                    (self.id, self.default_normal)
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal = self.normal.value();
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging && cursor_position.y != -1.0 {
                    let mut movement_y =
                        (cursor_position.y - self.state.prev_drag_y)
                            * self.scalar;
                    
                    if self.state.pressed_modifiers.matches(
                        self.modifier_keys) {
                        movement_y *= self.modifier_scalar;
                    }

                    let mut normal =
                        self.state.continuous_normal - movement_y;
                    
                    if normal < 0.0 { normal = 0.0; }
                    else if normal > 1.0 { normal = 1.0; }

                    self.state.continuous_normal = normal;
                    self.state.prev_drag_y = cursor_position.y;

                    messages.push((self.on_change)(
                        (self.id, normal.into())
                    ));
                }
            },
            Event::Keyboard(keyboard::Event::Input {
                modifiers,
                ..
            }) => {
                self.state.pressed_modifiers = modifiers;
            },
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
            self.normal,
            self.state.is_dragging,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.size.hash(state);
    }
}

/// The renderer of a [`Knob`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Knob`] in your user interface.
///
/// [`Knob`]: struct.Knob.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Knob`].
    ///
    /// It receives:
    ///   * the bounds of the [`Knob`]
    ///   * the current cursor position
    ///   * the current normal of the [`Knob`]
    ///   * the local state of the [`Knob`]
    ///   * the style of the [`Knob`]
    ///
    /// [`Knob`]: struct.Knob.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer, ID>
    From<Knob<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        v_slider: Knob<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(v_slider)
    }
}