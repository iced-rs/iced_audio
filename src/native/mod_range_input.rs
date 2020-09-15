//! Display an interactive dot that controls an [`Param`]
//!
//! [`Param`]: ../core/param/struct.Param.html

use std::fmt::Debug;

use iced_native::{
    keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Param, Normal};

static DEFAULT_SIZE: u16 = 10;
static DEFAULT_SCALAR: f32 = 0.005;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// An interactive dot that controls an [`Param`]
///
/// [`Param`]: ../core/param/struct.Param.html
#[allow(missing_debug_implementations)]
pub struct ModRangeInput<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    state: &'a mut State<ID>,
    size: Length,
    on_change: Box<dyn Fn(ID) -> Message>,
    scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer, ID>
    ModRangeInput<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    /// Creates a new [`ModRangeInput`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`ModRangeInput`]
    ///   * a function that will be called when the [`ModRangeInput`] is turned.
    ///
    /// [`State`]: struct.State.html
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn new<F>(state: &'a mut State<ID>, on_change: F) -> Self
    where
        F: 'static + Fn(ID) -> Message,
    {
        ModRangeInput {
            state,
            size: Length::from(Length::Units(DEFAULT_SIZE)),
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

    /// Sets the diameter of the [`ModRangeInput`]. The default size is
    /// `Length::from(Length::Units(31))`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`ModRangeInput`].
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`ModRangeInput`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.005`
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets the modifier keys of the [`ModRangeInput`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the ModRangeInputs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `ModRangeInput::scalar()` (which the default is `0.008`).
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
}

/// The local state of an [`ModRangeInput`].
///
/// [`ModRangeInput`]: struct.ModRangeInput.html
#[derive(Debug, Copy, Clone)]
pub struct State<ID: Debug + Copy + Clone> {
    /// The [`Param`] assigned to this widget
    ///
    /// [`Param`]: ../../core/param/struct.Param.html
    pub param: Param<ID>,
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl<ID: Debug + Copy + Clone> State<ID> {
    /// Creates a new [`ModRangeInput`] state.
    ///
    /// It expects:
    /// * a [`Param`] to assign to this widget
    ///
    /// [`Param`]: ../../core/param/struct.Param.html
    /// [`ModRangeInput`]: struct.ModRangeInput.html
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

    /// Returns the [`Normal`] value of the [`Param`]
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Param`]: ../../core/param/struct.Param.html
    pub fn normal(&mut self) -> &mut Normal {
        &mut self.param.normal
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for ModRangeInput<'a, Message, Renderer, ID>
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
        let limits = limits.width(self.size).height(self.size);

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
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => {
                    if self.state.is_dragging && cursor_position.y != -1.0 {
                        let mut movement_y = (cursor_position.y
                            - self.state.prev_drag_y)
                            * self.scalar;

                        if self
                            .state
                            .pressed_modifiers
                            .matches(self.modifier_keys)
                        {
                            movement_y *= self.modifier_scalar;
                        }

                        let mut normal =
                            self.state.continuous_normal - movement_y;

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
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.param.normal.value();
                }
                _ => {}
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;
                }
                _ => {}
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

/// The renderer of an [`ModRangeInput`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`ModRangeInput`] in your user interface.
///
/// [`ModRangeInput`]: struct.ModRangeInput.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`ModRangeInput`].
    ///
    /// It receives:
    ///   * the bounds of the [`ModRangeInput`]
    ///   * the current cursor position
    ///   * whether the ModRangeInput is currently being dragged
    ///   * the style of the [`ModRangeInput`]
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        is_dragging: bool,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer, ID> From<ModRangeInput<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        mod_range_input: ModRangeInput<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(mod_range_input)
    }
}
