//! Display an interactive 2D XY Pad that controls two [`NormalParam`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced_native::widget::tree::{self, Tree};
use iced_native::{
    event, keyboard, layout, mouse, touch, Clipboard, Element, Event, Layout,
    Length, Point, Rectangle, Shell, Size, Widget,
};

use crate::core::{Normal, NormalParam};

static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A 2D XY pad GUI widget that controls two [`NormalParam`] parameters at
/// once. One in the `x` coordinate and one in the `y` coordinate.
///
/// an [`XYPad`] will try to fill the space of its container while keeping a
/// square aspect ratio.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`XYPad`]: struct.XYPad.html
#[allow(missing_debug_implementations)]
pub struct XYPad<'a, Message, Renderer: self::Renderer> {
    normal_param_x: NormalParam,
    normal_param_y: NormalParam,
    on_change: Box<dyn Fn(Normal, Normal) -> Message + 'a>,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    size: Length,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> XYPad<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: self::Renderer,
{
    /// Creates a new [`XYPad`].
    ///
    /// It expects:
    ///   * the [`NormalParam`]s for the x & y axis of the [`XYPad`]
    ///   * a function that will be called when the [`XYPad`] is dragged.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new<F>(
        normal_param_x: NormalParam,
        normal_param_y: NormalParam,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(Normal, Normal) -> Message,
    {
        XYPad {
            normal_param_x,
            normal_param_y,
            on_change: Box::new(on_change),
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
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
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
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
struct State {
    is_dragging: bool,
    prev_drag_x: f32,
    prev_drag_y: f32,
    continuous_normal_x: f32,
    continuous_normal_y: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
}

impl State {
    /// Creates a new [`XYPad`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value of the x & y axis for the [`XYPad`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`XYPad`]: struct.XYPad.html
    fn new(normal_x: Normal, normal_y: Normal) -> Self {
        Self {
            is_dragging: false,
            prev_drag_x: 0.0,
            prev_drag_y: 0.0,
            continuous_normal_x: normal_x.as_f32(),
            continuous_normal_y: normal_y.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for XYPad<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: self::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(
            self.normal_param_x.value,
            self.normal_param_y.value,
        ))
    }

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
                    let bounds_size = {
                        if layout.bounds().width <= layout.bounds().height {
                            layout.bounds().width
                        } else {
                            layout.bounds().height
                        }
                    };
                    if bounds_size != 0.0 {
                        let mut movement_x = (cursor_position.x
                            - state.prev_drag_x)
                            / bounds_size;

                        let mut movement_y = (cursor_position.y
                            - state.prev_drag_y)
                            / bounds_size;

                        if state.pressed_modifiers.contains(self.modifier_keys)
                        {
                            movement_x *= self.modifier_scalar;
                            movement_y *= self.modifier_scalar;
                        }

                        let normal_x = state.continuous_normal_x + movement_x;
                        let normal_y = state.continuous_normal_y - movement_y;

                        state.prev_drag_x = cursor_position.x;
                        state.prev_drag_y = cursor_position.y;

                        state.continuous_normal_x = normal_x;
                        self.normal_param_x.value = normal_x.into();

                        state.continuous_normal_y = normal_y;
                        self.normal_param_y.value = normal_y.into();

                        shell.publish((self.on_change)(
                            self.normal_param_x.value,
                            self.normal_param_y.value,
                        ));

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
                            state.prev_drag_x = cursor_position.x;
                            state.prev_drag_y = cursor_position.y;
                            state.continuous_normal_x =
                                self.normal_param_x.value.as_f32();
                            state.continuous_normal_y =
                                self.normal_param_y.value.as_f32();

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

                            state.continuous_normal_x = normal_x;
                            self.normal_param_x.value = normal_x.into();

                            state.continuous_normal_y = normal_y;
                            self.normal_param_y.value = normal_y.into();

                            shell.publish((self.on_change)(
                                self.normal_param_x.value,
                                self.normal_param_y.value,
                            ));
                        }
                        _ => {
                            state.is_dragging = false;

                            self.normal_param_x.value =
                                self.normal_param_x.default;
                            state.continuous_normal_x =
                                self.normal_param_x.default.as_f32();

                            self.normal_param_y.value =
                                self.normal_param_y.default;
                            state.continuous_normal_y =
                                self.normal_param_y.default.as_f32();

                            shell.publish((self.on_change)(
                                self.normal_param_x.value,
                                self.normal_param_y.value,
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
                    state.continuous_normal_x =
                        self.normal_param_x.value.as_f32();
                    state.continuous_normal_y =
                        self.normal_param_y.value.as_f32();

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
            self.normal_param_x.value,
            self.normal_param_y.value,
            state.is_dragging,
            &self.style,
        )
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
    );
}

impl<'a, Message, Renderer> From<XYPad<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer,
{
    fn from(
        xy_pad: XYPad<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(xy_pad)
    }
}
