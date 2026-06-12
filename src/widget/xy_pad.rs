//! Display an interactive 2D XY Pad that controls two [`NormalParam`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
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
    Border, Color, Element, Event, Length, Rectangle, Renderer, Shadow, Size,
};

pub use crate::style::xy_pad::{Appearance, HandleCircle, HandleShape, HandleSquare, StyleSheet};

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
pub struct XYPad<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param_x: NormalParam,
    normal_param_y: NormalParam,
    on_change: Box<dyn 'a + Fn(Normal, Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    size: Length,
    style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme> XYPad<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`XYPad`].
    ///
    /// It expects:
    ///   * the [`NormalParam`]s for the x & y axis of the [`XYPad`]
    ///   * a function that will be called when the [`XYPad`] is dragged.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new<F>(normal_param_x: NormalParam, normal_param_y: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal, Normal) -> Message,
        <Theme as StyleSheet>::Style: Default,
    {
        XYPad {
            normal_param_x,
            normal_param_y,
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            size: Length::Fill,
            style: Default::default(),
        }
    }

    /// Sets the grab message of the [`XYPad`].
    /// This is called when the mouse grabs from the xy pad.
    ///
    /// Typically, the user's interaction with the xy pad starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`XYPad`].
    /// This is called when the mouse is released from the xy pad.
    ///
    /// Typically, the user's interaction with the xy pad is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the xy pad's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
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
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
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

    fn maybe_fire_on_grab(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_grab.as_mut().and_then(|on_grab| on_grab()) {
            shell.publish(message);
        }
    }

    fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
        shell.publish((self.on_change)(
            self.normal_param_x.value,
            self.normal_param_y.value,
        ));
    }

    fn maybe_fire_on_release(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) = self.on_release.as_mut().and_then(|on_release| on_release()) {
            shell.publish(message);
        }
    }
}

/// The local state of a [`XYPad`].
///
/// [`XYPad`]: struct.XYPad.html
#[derive(Debug, Copy, Clone)]
struct State {
    dragging_status: Option<SliderStatus>,
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
            dragging_status: None,
            prev_drag_x: 0.0,
            prev_drag_y: 0.0,
            continuous_normal_x: normal_x.as_f32(),
            continuous_normal_y: normal_y.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for XYPad<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: StyleSheet,
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
        let mut size = limits.resolve(self.size, self.size, Size::ZERO);

        if size.width <= size.height {
            size.height = size.width;
        } else {
            size.width = size.height;
        }

        layout::Node::new(size)
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
        let state = tree.state.downcast_mut::<State>();

        let is_over = cursor.is_over(layout.bounds());

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.dragging_status.is_some() {
                    let bounds_size = {
                        if layout.bounds().width <= layout.bounds().height {
                            layout.bounds().width
                        } else {
                            layout.bounds().height
                        }
                    };
                    if bounds_size != 0.0 {
                        let mut movement_x = (position.x - state.prev_drag_x) / bounds_size;

                        let mut movement_y = (position.y - state.prev_drag_y) / bounds_size;

                        if state.pressed_modifiers.contains(self.modifier_keys) {
                            movement_x *= self.modifier_scalar;
                            movement_y *= self.modifier_scalar;
                        }

                        let normal_x = state.continuous_normal_x + movement_x;
                        let normal_y = state.continuous_normal_y - movement_y;

                        state.prev_drag_x = position.x;
                        state.prev_drag_y = position.y;

                        state.continuous_normal_x = normal_x;
                        self.normal_param_x.value.set_clipped(normal_x);

                        state.continuous_normal_y = normal_y;
                        self.normal_param_y.value.set_clipped(normal_y);

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
                            state.prev_drag_x = cursor_position.x;
                            state.prev_drag_y = cursor_position.y;
                            state.continuous_normal_x = self.normal_param_x.value.as_f32();
                            state.continuous_normal_y = self.normal_param_y.value.as_f32();

                            let bounds_size = {
                                if layout.bounds().width <= layout.bounds().height {
                                    layout.bounds().width
                                } else {
                                    layout.bounds().height
                                }
                            };

                            let normal_x = (cursor_position.x - layout.bounds().x) / bounds_size;

                            let normal_y =
                                1.0 - ((cursor_position.y - layout.bounds().y) / bounds_size);

                            state.continuous_normal_x = normal_x;
                            self.normal_param_x.value.set_clipped(normal_x);

                            state.continuous_normal_y = normal_y;
                            self.normal_param_y.value.set_clipped(normal_y);

                            shell.publish((self.on_change)(
                                self.normal_param_x.value,
                                self.normal_param_y.value,
                            ));
                        }
                        _ => {
                            // Reset to default

                            let prev_dragging_status = state.dragging_status.take();

                            if (self.normal_param_x.value != self.normal_param_x.default)
                                && (self.normal_param_y.value != self.normal_param_y.default)
                            {
                                self.normal_param_x.value = self.normal_param_x.default;
                                state.continuous_normal_x = self.normal_param_x.default.as_f32();

                                self.normal_param_y.value = self.normal_param_y.default;
                                state.continuous_normal_y = self.normal_param_y.default.as_f32();

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

                    state.continuous_normal_x = self.normal_param_x.value.as_f32();
                    state.continuous_normal_y = self.normal_param_y.value.as_f32();

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

        let appearance = if state.dragging_status.is_some() {
            theme.dragging(&self.style)
        } else if is_over {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_size = {
            if bounds.width <= bounds.height {
                bounds.width.floor()
            } else {
                bounds.height.floor()
            }
        };

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y: bounds_y,
                    width: bounds_size,
                    height: bounds_size,
                },
                border: Border {
                    color: appearance.border_color,
                    width: appearance.border_width,
                    radius: Radius::new(0.0),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            appearance.back_color,
        );

        let handle_x = (bounds_x + (bounds_size * self.normal_param_x.value.as_f32())).floor();
        let handle_y =
            (bounds_y + (bounds_size * (1.0 - self.normal_param_y.value.as_f32()))).floor();

        let bounds_center = (bounds_size / 2.0).floor();

        if appearance.center_line_color != Color::TRANSPARENT {
            let center_line_width = appearance.center_line_width;
            let half_center_line_width = (center_line_width / 2.0).floor();

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y + bounds_center - half_center_line_width,
                        width: bounds_size,
                        height: center_line_width,
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(0.0),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                appearance.center_line_color,
            );

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds_x + bounds_center - half_center_line_width,
                        y: bounds_y,
                        width: center_line_width,
                        height: bounds_size,
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(0.0),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                appearance.center_line_color,
            );
        };

        if appearance.rail_width != 0.0 {
            let rail_width = appearance.rail_width;
            let half_rail_width = (rail_width / 2.0).floor();

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: handle_y - half_rail_width,
                        width: bounds_size,
                        height: appearance.rail_width,
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(0.0),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                appearance.h_rail_color,
            );

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: handle_x - half_rail_width,
                        y: bounds_y,
                        width: appearance.rail_width,
                        height: bounds_size,
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::new(0.0),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                appearance.v_rail_color,
            );
        };

        match appearance.handle {
            HandleShape::Circle(circle) => {
                let diameter = circle.diameter;
                let radius = diameter / 2.0;

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: handle_x - radius,
                            y: handle_y - radius,
                            width: diameter,
                            height: diameter,
                        },
                        border: Border {
                            color: circle.border_color,
                            width: circle.border_width,
                            radius: Radius::new(radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    circle.color,
                );
            }
            HandleShape::Square(square) => {
                let size = square.size as f32;
                let half_size = (size / 2.0).floor();

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: handle_x - half_size,
                            y: handle_y - half_size,
                            width: size,
                            height: size,
                        },
                        border: Border {
                            color: square.border_color,
                            width: square.border_width,
                            radius: Radius::new(square.border_radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    square.color,
                );
            }
        }
    }
}

impl<'a, Message, Theme> From<XYPad<'a, Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(xy_pad: XYPad<'a, Message, Theme>) -> Self {
        Self::new(xy_pad)
    }
}
