//! Display a ramp control that controls a [`NormalParam`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use crate::core::{
    NormalParam,
    virtual_slider::{self, Gesture, VirtualSlider},
};
use iced_core::{
    Border, Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shadow, Shell, Size,
    Vector, Widget,
    border::Radius,
    layout, mouse,
    renderer::{Quad, Style},
    widget::{Tree, tree},
};
use iced_graphics::geometry::{self, Frame, LineCap, Path, Stroke};

pub use crate::style::ramp::{Appearance, StyleSheet};

const DEFAULT_WIDTH: f32 = 40.0;
const DEFAULT_HEIGHT: f32 = 20.0;

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
pub struct Ramp<'a, Message, Theme: StyleSheet> {
    virtual_slider: VirtualSlider<'a, Message>,
    enabled: bool,
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
    direction: RampDirection,
}

impl<'a, Message, Theme: StyleSheet> Ramp<'a, Message, Theme> {
    /// Creates a new [`Ramp`].
    ///
    /// It expects:
    ///   * `normal_param` - The normalized value of the parameter.
    ///   * the [`RampDirection`] of the [`Ramp`], which tells if the ramp line
    ///     should point `Up` (from `bottom-left` to `top-right`), or `Down` (from
    ///     `top-left` to `bottom-right`)
    ///
    /// [`RampDirection`]: enum.RampDirection.html
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Ramp`]: struct.Ramp.html
    pub fn new(normal_param: impl Into<NormalParam>, direction: RampDirection) -> Self
    where
        <Theme as StyleSheet>::Style: Default,
    {
        Ramp {
            virtual_slider: VirtualSlider::new(normal_param.into()),
            enabled: true,
            width: Length::Fixed(DEFAULT_WIDTH),
            height: Length::Fixed(DEFAULT_HEIGHT),
            style: Default::default(),
            direction,
        }
    }

    /// Sets the message to emit when the user gestures this widget.
    pub fn on_gesture(mut self, on_gesture: impl 'a + FnMut(Gesture) -> Message) -> Self {
        self.virtual_slider.set_on_gesture(on_gesture);
        self
    }

    /// Set a custom configuration to use for this virtual slider.
    pub fn config(mut self, config: &virtual_slider::Config) -> Self {
        self.virtual_slider.config = *config;
        self
    }

    /// Enable/disable this widget.
    ///
    /// The default is `true`.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Ramp<'a, Message, Theme>
where
    Theme: StyleSheet,
    Renderer: iced_core::Renderer + iced_graphics::geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<virtual_slider::State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(virtual_slider::State::new(
            self.virtual_slider.param().normal,
        ))
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
        if !self.enabled {
            return;
        }

        let state = tree.state.downcast_mut::<virtual_slider::State>();
        let cursor_is_over = cursor.is_over(layout.bounds());

        if self
            .virtual_slider
            .update(state, cursor_is_over, false, false, event, cursor, shell)
            .should_redraw()
        {
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
        let state = state.state.downcast_ref::<virtual_slider::State>();

        let bounds = layout.bounds();
        let cursor_is_over = cursor.is_over(layout.bounds());
        let normal_val = self.virtual_slider.param().normal;

        let appearance = if state.is_gesturing() {
            theme.gesturing(&self.style)
        } else if cursor_is_over {
            theme.hovered(&self.style)
        } else {
            theme.idle(&self.style)
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

        match self.direction {
            RampDirection::Up => {
                if normal_val.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_down_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control =
                        Point::new(range_width * (1.0 - (normal_val.as_f32() * 2.0)), 0.0);
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else if normal_val.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * (1.0 - ((normal_val.as_f32() - 0.5) * 2.0)),
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_center_color),
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                }
            }
            RampDirection::Down => {
                if normal_val.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_down_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(range_width * (normal_val.as_f32() * 2.0), 0.0);
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else if normal_val.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * ((normal_val.as_f32() - 0.5) * 2.0),
                        -range_height,
                    );
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                } else {
                    let stroke = Stroke {
                        width: appearance.line_width,
                        style: geometry::Style::Solid(appearance.line_center_color),
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
                            // clippy gets confused when default iced features are disabled
                            #[allow(clippy::unit_arg)]
                            renderer.draw_geometry(frame.into_geometry());
                        },
                    );
                }
            }
        };
    }
}

impl<'a, Message, Theme, Renderer> From<Ramp<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: iced_core::Renderer + iced_graphics::geometry::Renderer,
{
    fn from(ramp: Ramp<'a, Message, Theme>) -> Self {
        Self::new(ramp)
    }
}
