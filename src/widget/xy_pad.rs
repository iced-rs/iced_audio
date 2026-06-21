//! Display an interactive 2D XY Pad that controls two [`NormalParam`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use crate::{
    core::{Normal, NormalParam},
    virtual_slider::Gesture,
};
use iced_core::{
    Border, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shadow, Shell,
    Size, Widget,
    border::Radius,
    layout, mouse,
    renderer::{Quad, Style},
    touch,
    widget::{Tree, tree},
    window,
};

pub use crate::style::xy_pad::{Appearance, HandleCircle, HandleShape, HandleSquare, StyleSheet};

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
    on_gesture_x: Option<Box<dyn 'a + FnMut(Gesture) -> Message>>,
    on_gesture_y: Option<Box<dyn 'a + FnMut(Gesture) -> Message>>,
    enabled: bool,
    param_x: Option<NormalParam>,
    param_y: Option<NormalParam>,
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
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`XYPad`]: struct.XYPad.html
    pub fn new(
        param_x: Option<impl Into<NormalParam>>,
        param_y: Option<impl Into<NormalParam>>,
    ) -> Self
    where
        <Theme as StyleSheet>::Style: Default,
    {
        XYPad {
            param_x: param_x.map(|p| p.into()),
            param_y: param_y.map(|p| p.into()),
            enabled: true,
            on_gesture_x: None,
            on_gesture_y: None,
            size: Length::Fill,
            style: Default::default(),
        }
    }

    /// Sets the message to emit when the user gestures the X axis of this widget.
    pub fn on_gesture_x(
        mut self,
        on_gesture_x: Option<impl 'a + FnMut(Gesture) -> Message>,
    ) -> Self {
        if let Some(f) = on_gesture_x {
            self.on_gesture_x = Some(Box::new(f));
        }
        self
    }

    /// Sets the message to emit when the user gestures the Y axis of this widget.
    pub fn on_gesture_y(
        mut self,
        on_gesture_y: Option<impl 'a + FnMut(Gesture) -> Message>,
    ) -> Self {
        if let Some(f) = on_gesture_y {
            self.on_gesture_y = Some(Box::new(f));
        }
        self
    }

    /// Enable/disable this widget.
    ///
    /// The default is `true`.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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

    /// Returns `true` if any param value has changed.
    fn set_param_values(
        &mut self,
        value_x: f32,
        value_y: f32,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
    ) -> bool {
        let prev_value_x = self
            .param_x
            .map(|p| p.normal.as_f32())
            .unwrap_or(state.continuous_normal_x);
        let prev_value_y = self
            .param_y
            .map(|p| p.normal.as_f32())
            .unwrap_or(state.continuous_normal_y);

        let new_value_x = Normal::new(value_x);
        let new_value_y = Normal::new(value_y);

        let x_changed = (new_value_x.as_f32() - prev_value_x).abs() > f32::EPSILON;
        let y_changed = (new_value_y.as_f32() - prev_value_y).abs() > f32::EPSILON;

        if !(x_changed || y_changed) {
            return false;
        }

        if x_changed && let Some(param_x) = &mut self.param_x {
            param_x.normal = new_value_x;

            if let Gesture::GestureEnd = &state.last_sent_gesture_x {
                // Send a GestureStart message first.
                if let Some(on_gesture_x) = &mut self.on_gesture_x {
                    shell.publish((on_gesture_x)(Gesture::GestureStart));
                }
                state.last_sent_gesture_x = Gesture::GestureStart;
            }

            if let Some(on_gesture_x) = &mut self.on_gesture_x {
                shell.publish((on_gesture_x)(Gesture::Gesturing(new_value_x)));
            }
            state.last_sent_gesture_x = Gesture::Gesturing(new_value_x);
        }

        if y_changed && let Some(param_y) = &mut self.param_y {
            param_y.normal = new_value_y;

            if let Gesture::GestureEnd = &state.last_sent_gesture_y {
                // Send a GestureStart message first.
                if let Some(on_gesture_y) = &mut self.on_gesture_y {
                    shell.publish((on_gesture_y)(Gesture::GestureStart));
                }
                state.last_sent_gesture_y = Gesture::GestureStart;
            }

            if let Some(on_gesture_y) = &mut self.on_gesture_y {
                shell.publish((on_gesture_y)(Gesture::Gesturing(new_value_y)));
            }
            state.last_sent_gesture_y = Gesture::Gesturing(new_value_y);
        }

        true
    }

    fn end_gesture(&mut self, state: &mut State, shell: &mut Shell<'_, Message>) {
        state.is_dragging = false;

        if state.last_sent_gesture_x != Gesture::GestureEnd && self.param_x.is_some() {
            if let Some(on_gesture_x) = &mut self.on_gesture_x {
                shell.publish((on_gesture_x)(Gesture::GestureEnd));
            }
            state.last_sent_gesture_x = Gesture::GestureEnd;
        }

        if state.last_sent_gesture_y != Gesture::GestureEnd && self.param_y.is_some() {
            if let Some(on_gesture_y) = &mut self.on_gesture_y {
                shell.publish((on_gesture_y)(Gesture::GestureEnd));
            }
            state.last_sent_gesture_y = Gesture::GestureEnd;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    is_dragging: bool,
    continuous_normal_x: f32,
    continuous_normal_y: f32,
    last_sent_gesture_x: Gesture,
    last_sent_gesture_y: Gesture,
    last_click: Option<mouse::Click>,
}

impl State {
    fn new(param_x: Option<NormalParam>, param_y: Option<NormalParam>) -> Self {
        Self {
            is_dragging: false,
            continuous_normal_x: param_x.map(|p| p.normal.as_f32()).unwrap_or(0.5),
            continuous_normal_y: param_y.map(|p| p.normal.as_f32()).unwrap_or(0.5),
            last_sent_gesture_x: Gesture::GestureEnd,
            last_sent_gesture_y: Gesture::GestureEnd,
            last_click: None,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for XYPad<'a, Message, Theme>
where
    Theme: StyleSheet,
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.param_x, self.param_y))
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
        if !self.enabled {
            return;
        }

        let state = tree.state.downcast_mut::<State>();
        let cursor_is_over = cursor.is_over(layout.bounds());

        // Update state if the value was modified outside of the widget.
        if !state.is_dragging
            && let Some(param_x) = &self.param_x
            && state.continuous_normal_x != param_x.normal.as_f32()
        {
            state.continuous_normal_x = param_x.normal.as_f32();
        }
        if !state.is_dragging
            && let Some(param_y) = &self.param_y
            && state.continuous_normal_y != param_y.normal.as_f32()
        {
            state.continuous_normal_y = param_y.normal.as_f32();
        }

        let mut capture_event = false;
        let mut param_changed = false;
        let mut hover_state_changed = false;

        let values_from_cursor_pos = |position: Point| -> (f32, f32) {
            (
                if layout.bounds().width > 0.0 {
                    (position.x - layout.bounds().x) / layout.bounds().width
                } else {
                    0.0
                },
                if layout.bounds().height > 0.0 {
                    1.0 - ((position.y - layout.bounds().y) / layout.bounds().height)
                } else {
                    0.0
                },
            )
        };

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.is_dragging {
                    let (value_x, value_y) = values_from_cursor_pos(*position);

                    param_changed = self.set_param_values(value_x, value_y, state, shell);

                    capture_event = true;
                } else if cursor_is_over {
                    capture_event = true;
                }
            }
            Event::Mouse(mouse::Event::CursorEntered) | Event::Mouse(mouse::Event::CursorLeft) => {
                capture_event = true;
                hover_state_changed = true;
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor_is_over && let Some(cursor_position) = cursor.position() {
                    let click =
                        mouse::Click::new(cursor_position, mouse::Button::Left, state.last_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            state.is_dragging = true;

                            let (value_x, value_y) = values_from_cursor_pos(cursor_position);

                            param_changed = self.set_param_values(value_x, value_y, state, shell);
                        }
                        _ => {
                            // Reset to default

                            let value_x = self.param_x.map(|p| p.default.as_f32()).unwrap_or(0.5);
                            let value_y = self.param_y.map(|p| p.default.as_f32()).unwrap_or(0.5);

                            param_changed = self.set_param_values(value_x, value_y, state, shell);
                            self.end_gesture(state, shell);
                        }
                    }

                    state.last_click = Some(click);

                    capture_event = true;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                self.end_gesture(state, shell);
                capture_event = true;
            }
            Event::Window(window::Event::Unfocused) => {
                self.end_gesture(state, shell);
            }
            _ => {}
        }

        if capture_event {
            shell.capture_event();
        }

        if param_changed || hover_state_changed {
            shell.request_redraw();
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
        let cursor_is_over = cursor.is_over(layout.bounds());

        let appearance = if state.is_dragging {
            theme.gesturing(&self.style)
        } else if cursor_is_over {
            theme.hovered(&self.style)
        } else {
            theme.idle(&self.style)
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

        let normal_x = self
            .param_x
            .as_ref()
            .map(|p| p.normal.as_f32())
            .unwrap_or(state.continuous_normal_x);
        let normal_y = self
            .param_y
            .as_ref()
            .map(|p| p.normal.as_f32())
            .unwrap_or(state.continuous_normal_y);

        let handle_x = (bounds_x + (bounds_size * normal_x)).floor();
        let handle_y = (bounds_y + (bounds_size * (1.0 - normal_y))).floor();

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

impl<'a, Message, Theme, Renderer> From<XYPad<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: iced_core::Renderer,
{
    fn from(xy_pad: XYPad<'a, Message, Theme>) -> Self {
        Self::new(xy_pad)
    }
}
