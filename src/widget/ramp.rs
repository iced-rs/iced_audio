//! Display a ramp control that controls a [`NormalParam`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use crate::core::{Normal, NormalParam, SliderStatus};
use iced::{
    advanced::{
        graphics::{
            core::{keyboard, touch},
            geometry::Renderer as _,
        },
        layout, mouse,
        renderer::{Quad, Style},
        widget::{tree, Tree},
        Clipboard, Layout, Renderer as _, Shell, Widget,
    },
    border::Radius,
    widget::canvas::{self, Frame, LineCap, Path, Stroke},
    Border, Element, Event, Length, Point, Rectangle, Renderer, Shadow, Size, Vector,
};

pub use crate::style::ramp::{Appearance, StyleSheet};

static DEFAULT_WIDTH: f32 = 40.0;
static DEFAULT_HEIGHT: f32 = 20.0;
static DEFAULT_SCALAR: f32 = 0.00385;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// The direction of a [`Ramp`] widget.
#[derive(Debug, Copy, Clone, Default)]
pub enum RampDirection {
    /// The line points upwards from `bottom-left` to `top-right`.
    #[default]
    Up,
    /// The line points downwards from `top-left` to `bottom-right`.
    Down,
}

/// A ramp GUI widget that controls a [`NormalParam`]. It is usually used to
/// represent the easing of a parameter between two points in time.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`Ramp`]: struct.Ramp.html
#[allow(missing_debug_implementations)]
pub struct Ramp<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
    direction: RampDirection,
}

impl<'a, Message, Theme> Ramp<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`Ramp`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`Ramp`]
    ///   * a function that will be called when the [`Ramp`] is dragged.
    ///   * the [`RampDirection`] of the [`Ramp`], which tells if the ramp line
    ///     should point `Up` (from `bottom-left` to `top-right`), or `Down` (from
    ///     `top-left` to `bottom-right`)
    ///
    /// [`RampDirection`]: enum.RampDirection.html
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new<F>(normal_param: NormalParam, on_change: F, direction: RampDirection) -> Self
    where
        F: 'static + Fn(Normal) -> Message,
        <Theme as StyleSheet>::Style: Default,
    {
        Ramp {
            normal_param,
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            width: Length::Fixed(DEFAULT_WIDTH),
            height: Length::Fixed(DEFAULT_HEIGHT),
            style: Default::default(),
            direction,
        }
    }

    /// Sets the grab message of the [`Ramp`].
    /// This is called when the mouse grabs from the ramp.
    ///
    /// Typically, the user's interaction with the ramp starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`Ramp`].
    /// This is called when the mouse is released from the ramp.
    ///
    /// Typically, the user's interaction with the ramp is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the ramp's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the width of the [`Ramp`].
    /// The default width is `Length::from(Length::Fixed(30))`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Ramp`].
    /// The default height is `Length::from(Length::Fixed(20))`.
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`Ramp`].
    ///
    /// [`Ramp`]: struct.Ramp.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
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

/// The local state of a [`Ramp`].
///
/// [`Ramp`]: struct.Ramp.html
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
    /// Creates a new [`Ramp`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`Ramp`].
    ///   A [`Normal`] value of `0.5` represents a straight line,
    ///   `0.0` is curved downward all the way,
    ///   and `1.0` is curved upward all the way.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Ramp`]: struct.Ramp.html
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

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for Ramp<'a, Message, Theme>
where
    Message: 'a + Clone,
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
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(self.width, self.height, Size::ZERO))
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
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = *modifiers;
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

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y: bounds_y,
                    width: bounds_width,
                    height: bounds_height,
                },
                border: Border {
                    color: appearance.back_border_color,
                    width: appearance.back_border_width,
                    radius: Radius::new(0.0),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            appearance.back_color,
        );

        let border_width = appearance.back_border_width;
        let twice_border_width = border_width * 2.0;

        let range_width = bounds_width - twice_border_width;
        let range_height = bounds_height - twice_border_width;

        let normal = self.normal_param.value;

        match self.direction {
            RampDirection::Up => {
                if normal.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_down_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(range_width * (1.0 - (normal.as_f32() * 2.0)), 0.0);
                    let to = Point::new(range_width, -range_height);

                    let path = Path::new(|p| {
                        p.move_to(to);
                        p.quadratic_curve_to(control, Point::ORIGIN)
                    });

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else if normal.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * (1.0 - ((normal.as_f32() - 0.5) * 2.0)),
                        -range_height,
                    );
                    let to = Point::new(range_width, -range_height);

                    let path = Path::new(|p| {
                        p.move_to(to);
                        p.quadratic_curve_to(control, Point::ORIGIN)
                    });

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_center_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let path =
                        Path::line(Point::new(0.0, 0.0), Point::new(range_width, -range_height));

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                }
            }
            RampDirection::Down => {
                if normal.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_down_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(range_width * (normal.as_f32() * 2.0), 0.0);
                    let from = Point::new(0.0, -range_height);
                    let to = Point::new(range_width, 0.0);

                    let path = Path::new(|p| {
                        p.move_to(from);
                        p.quadratic_curve_to(control, to)
                    });

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else if normal.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control =
                        Point::new(range_width * ((normal.as_f32() - 0.5) * 2.0), -range_height);
                    let from = Point::new(0.0, -range_height);
                    let to = Point::new(range_width, 0.0);

                    let path = Path::new(|p| {
                        p.move_to(to);
                        p.quadratic_curve_to(control, from)
                    });

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: canvas::Style::Solid(appearance.line_center_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let path =
                        Path::line(Point::new(0.0, -range_height), Point::new(range_width, 0.0));

                    let mut frame = Frame::new(renderer, Size::new(range_width, range_height));

                    frame.translate(Vector::new(0.0, range_height));

                    frame.stroke(&path, stroke);

                    renderer.with_translation(
                        Vector::new(bounds_x + border_width, bounds_y + border_width),
                        |renderer| {
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                }
            }
        };
    }
}

impl<'a, Message, Theme> From<Ramp<'a, Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(ramp: Ramp<'a, Message, Theme>) -> Self {
        Self::new(ramp)
    }
}
