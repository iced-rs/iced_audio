//! Display an interactive 2D XY Pad that controls two [`NormalParam`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
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
    state: &'a mut State,
    on_change: Box<dyn Fn(Normal, Normal) -> Message>,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    size: Length,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer> XYPad<'a, Message, Renderer> {
    /// Creates a new [`XYPad`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`XYPad`]
    ///   * a function that will be called when the [`XYPad`] is dragged.
    ///
    /// [`State`]: struct.State.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new<F>(state: &'a mut State, on_change: F) -> Self
    where
        F: 'static + Fn(Normal, Normal) -> Message,
    {
        XYPad {
            state,
            on_change: Box::new(on_change),
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers {
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
pub struct State {
    normal_param_x: NormalParam,
    normal_param_y: NormalParam,
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
    /// * a [`NormalParam`] to assign to this widget's x axis
    /// * a [`NormalParam`] to assign to this widget's y axis
    ///
    /// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new(
        normal_param_x: NormalParam,
        normal_param_y: NormalParam,
    ) -> Self {
        Self {
            normal_param_x,
            normal_param_y,
            is_dragging: false,
            prev_drag_x: 0.0,
            prev_drag_y: 0.0,
            continuous_normal_x: normal_param_x.value.as_f32(),
            continuous_normal_y: normal_param_y.value.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }

    /// Set the normalized value of the x axis of the [`XYPad`].
    pub fn set_normal_x(&mut self, normal: Normal) {
        self.normal_param_x.value = normal;
        self.continuous_normal_x = normal.into();
    }

    /// Set the normalized value of the y axis of the [`XYPad`].
    pub fn set_normal_y(&mut self, normal: Normal) {
        self.normal_param_y.value = normal;
        self.continuous_normal_y = normal.into();
    }

    /// Get the normalized value of the x axis of the [`XYPad`].
    pub fn normal_x(&self) -> Normal {
        self.normal_param_x.value
    }

    /// Get the normalized value of the y axis of the [`XYPad`].
    pub fn normal_y(&self) -> Normal {
        self.normal_param_y.value
    }

    /// Set the normalized default value of the x axis of the [`XYPad`].
    pub fn set_default_x(&mut self, normal: Normal) {
        self.normal_param_x.default = normal;
    }

    /// Set the normalized default value of the y axis of the [`XYPad`].
    pub fn set_default_y(&mut self, normal: Normal) {
        self.normal_param_y.default = normal;
    }

    /// Get the normalized default value of the x axis of the [`XYPad`].
    pub fn default_x(&self) -> Normal {
        self.normal_param_x.default
    }

    /// Get the normalized default value of the y axis of the [`XYPad`].
    pub fn default_y(&self) -> Normal {
        self.normal_param_y.default
    }

    /// Snap the visible value of the x axis of the [`XYPad`] to the nearest value
    /// in the integer range.
    ///
    /// # Example
    ///
    /// ```
    /// use iced_audio::{xy_pad, IntRange};
    ///
    /// let mut state = xy_pad::State::new(Default::default(), Default::default());
    /// let int_range = IntRange::new(0, 10);
    ///
    /// state.snap_visible_x_to(&int_range);
    ///
    /// ```
    pub fn snap_visible_x_to(&mut self, range: &IntRange) {
        self.normal_param_x.value = range.snapped(self.normal_param_x.value);
    }

    /// Snap the visible value of the y axis of the [`XYPad`] to the nearest value
    /// in the integer range.
    ///
    /// # Example
    ///
    /// ```
    /// use iced_audio::{xy_pad, IntRange};
    ///
    /// let mut state = xy_pad::State::new(Default::default(), Default::default());
    /// let int_range = IntRange::new(0, 10);
    ///
    /// state.snap_visible_y_to(&int_range);
    ///
    /// ```
    pub fn snap_visible_y_to(&mut self, range: &IntRange) {
        self.normal_param_y.value = range.snapped(self.normal_param_y.value);
    }

    /// Is the [`XYPad`] currently in the dragging state?
    ///
    /// [`XYPad`]: struct.XYPad.html
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for XYPad<'a, Message, Renderer>
where
    Renderer: self::Renderer,
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
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
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

                            self.state.continuous_normal_x = normal_x;
                            self.state.normal_param_x.value = normal_x.into();

                            self.state.continuous_normal_y = normal_y;
                            self.state.normal_param_y.value = normal_y.into();

                            messages.push((self.on_change)(
                                self.state.normal_param_x.value,
                                self.state.normal_param_y.value,
                            ));

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

                                self.state.continuous_normal_x = normal_x;
                                self.state.normal_param_x.value =
                                    normal_x.into();

                                self.state.continuous_normal_y = normal_y;
                                self.state.normal_param_y.value =
                                    normal_y.into();

                                messages.push((self.on_change)(
                                    self.state.normal_param_x.value,
                                    self.state.normal_param_y.value,
                                ));
                            }
                            _ => {
                                self.state.is_dragging = false;

                                self.state.normal_param_x.value =
                                    self.state.normal_param_x.default;
                                self.state.normal_param_y.value =
                                    self.state.normal_param_y.default;

                                messages.push((self.on_change)(
                                    self.state.normal_param_x.value,
                                    self.state.normal_param_y.value,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);

                        return event::Status::Captured;
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal_x =
                        self.state.normal_param_x.value.as_f32();
                    self.state.continuous_normal_y =
                        self.state.normal_param_y.value.as_f32();

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
            self.state.normal_param_x.value,
            self.state.normal_param_y.value,
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

impl<'a, Message, Renderer> From<XYPad<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        xy_pad: XYPad<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(xy_pad)
    }
}
