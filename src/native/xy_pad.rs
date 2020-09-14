//! Display an interactive 2D XY Pad that controls two [`Param`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
//!
//! [`Param`]: ../core/param/trait.Param.html

use std::fmt::Debug;

use iced_native::{
    keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, Param};

static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A 2D XY pad GUI widget that controls two [`Param`] parameters at
/// once. One in the `x` coordinate and one in the `y` coordinate.
///
/// an [`XYPad`] will try to fill the space of its container while keeping a
/// square aspect ratio.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`XYPad`]: struct.XYPad.html
#[allow(missing_debug_implementations)]
pub struct XYPad<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    state: &'a mut State<ID>,
    on_change: Box<dyn Fn(ID) -> Message>,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    size: Length,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer, ID> XYPad<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    /// Creates a new [`XYPad`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`XYPad`]
    ///   * a function that will be called when the [`XYPad`] is dragged.
    ///
    /// [`State`]: struct.State.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new<F>(state: &'a mut State<ID>, on_change: F) -> Self
    where
        F: 'static + Fn(ID) -> Message,
    {
        XYPad {
            state,
            on_change: Box::new(on_change),
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            size: Length::Fill,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the size of the [`XYPad`].
    ///
    /// [`XYPad`]: struct.XYPad.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`XYPad`].
    ///
    /// [`XYPad`]: struct.XYPad.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the modifier keys of the [`XYPad`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`XYPad`]: struct.XYPad.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the slider while holding down
    /// the modifier key.
    ///
    /// For example, a scalar of `0.5` will cause the slider to move half a
    /// pixel for every pixel the mouse moves.
    ///
    /// The default scalar is `0.02`, and the default modifier key is `Ctrl`.
    ///
    /// [`XYPad`]: struct.XYPad.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }
}

/// The local state of a [`XYPad`].
///
/// [`XYPad`]: struct.XYPad.html
#[derive(Debug, Copy, Clone)]
pub struct State<ID: Debug + Copy + Clone> {
    /// The [`Param`] assigned to this widget's x axis
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    pub param_x: Param<ID>,
    /// The [`Param`] assigned to this widget's y axis
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    pub param_y: Param<ID>,
    is_dragging: bool,
    prev_drag_x: f32,
    prev_drag_y: f32,
    continuous_normal_x: f32,
    continuous_normal_y: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl<ID: Debug + Copy + Clone> State<ID> {
    /// Creates a new [`XYPad`] state.
    ///
    /// It expects:
    /// * a [`Param`] to assign to this widget's x axis
    /// * a [`Param`] to assign to this widget's y axis
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new(param_x: Param<ID>, param_y: Param<ID>) -> Self {
        Self {
            param_x,
            param_y,
            is_dragging: false,
            prev_drag_x: 0.0,
            prev_drag_y: 0.0,
            continuous_normal_x: param_x.normal.value(),
            continuous_normal_y: param_y.normal.value(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }

    /// Returns the [`Normal`] value of the x [`Param`]
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Param`]: ../../core/param/struct.Param.html
    pub fn x_normal(&mut self) -> &mut Normal {
        &mut self.param_x.normal
    }

    /// Returns the [`Normal`] value of the y [`Param`]
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Param`]: ../../core/param/struct.Param.html
    pub fn y_normal(&mut self) -> &mut Normal {
        &mut self.param_y.normal
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for XYPad<'a, Message, Renderer, ID>
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

        let mut size = limits.resolve(Size::ZERO);

        if size.width <= size.height {
            size.height = size.width;
        } else {
            size.width = size.height;
        }

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
                    if self.state.is_dragging {
                        let bounds_size = {
                            if layout.bounds().width <= layout.bounds().height {
                                layout.bounds().width
                            } else {
                                layout.bounds().height
                            }
                        };
                        if bounds_size != 0.0 {
                            let mut movement_x = (cursor_position.x
                                - self.state.prev_drag_x)
                                / bounds_size;

                            let mut movement_y = (cursor_position.y
                                - self.state.prev_drag_y)
                                / bounds_size;

                            if self
                                .state
                                .pressed_modifiers
                                .matches(self.modifier_keys)
                            {
                                movement_x *= self.modifier_scalar;
                                movement_y *= self.modifier_scalar;
                            }

                            let normal_x =
                                self.state.continuous_normal_x + movement_x;
                            let normal_y =
                                self.state.continuous_normal_y - movement_y;

                            self.state.prev_drag_x = cursor_position.x;
                            self.state.prev_drag_y = cursor_position.y;

                            if normal_x != self.state.continuous_normal_x {
                                self.state.continuous_normal_x = normal_x;

                                self.state.param_x.normal = normal_x.into();

                                messages.push((self.on_change)(
                                    self.state.param_x.id,
                                ));
                            }

                            if normal_y != self.state.continuous_normal_y {
                                self.state.continuous_normal_y = normal_y;

                                self.state.param_y.normal = normal_y.into();

                                messages.push((self.on_change)(
                                    self.state.param_y.id,
                                ));
                            }
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
                                self.state.prev_drag_x = cursor_position.x;
                                self.state.prev_drag_y = cursor_position.y;

                                let bounds_size = {
                                    if layout.bounds().width
                                        <= layout.bounds().height
                                    {
                                        layout.bounds().width
                                    } else {
                                        layout.bounds().height
                                    }
                                };

                                let normal_x = (cursor_position.x
                                    - layout.bounds().x)
                                    / bounds_size;

                                let normal_y = 1.0
                                    - ((cursor_position.y - layout.bounds().y)
                                        / bounds_size);

                                if normal_x != self.state.continuous_normal_x {
                                    self.state.continuous_normal_x = normal_x;

                                    self.state.param_x.normal = normal_x.into();

                                    messages.push((self.on_change)(
                                        self.state.param_x.id,
                                    ));
                                }

                                if normal_y != self.state.continuous_normal_y {
                                    self.state.continuous_normal_y = normal_y;

                                    self.state.param_y.normal = normal_y.into();

                                    messages.push((self.on_change)(
                                        self.state.param_y.id,
                                    ));
                                }
                            }
                            _ => {
                                self.state.is_dragging = false;

                                self.state.param_x.normal =
                                    self.state.param_x.default_normal;
                                self.state.param_y.normal =
                                    self.state.param_y.default_normal;

                                messages.push((self.on_change)(
                                    self.state.param_x.id,
                                ));
                                messages.push((self.on_change)(
                                    self.state.param_y.id,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal_x =
                        self.state.param_x.normal.value();
                    self.state.continuous_normal_y =
                        self.state.param_y.normal.value();
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
            self.state.param_x.normal,
            self.state.param_y.normal,
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

/// The renderer of an [`XYPad`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`XYPad`] in your user interface.
///
/// [`XYPad`]: struct.XYPad.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`XYPad`].
    ///
    /// It receives:
    ///   * the bounds of the [`XYPad`]
    ///   * the current cursor position
    ///   * the current normal of the x coordinate of the [`XYPad`]
    ///   * the current normal of the y coordinate of the [`XYPad`]
    ///   * whether the xy_pad is currently being dragged
    ///   * the style of the [`XYPad`]
    ///
    /// [`XYPad`]: struct.XYPad.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal_x: Normal,
        normal_y: Normal,
        is_dragging: bool,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer, ID> From<XYPad<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        xy_pad: XYPad<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(xy_pad)
    }
}
