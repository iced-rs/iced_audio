//! Display an interactive dot that controls an [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use crate::core::{
    NormalParam,
    virtual_slider::{self, Gesture, VirtualSlider},
};
use iced_core::{
    Border, Clipboard, Element, Event, Layout, Length, Rectangle, Shadow, Shell, Size, Widget,
    border::Radius,
    layout, mouse,
    renderer::{Quad, Style},
    widget::{Tree, tree},
};

pub use crate::style::mod_range_input::{
    Appearance, CircleAppearance, InvisibleStyle, SquareAppearance, StyleSheet,
};

const DEFAULT_SIZE: f32 = 10.0;

/// An interactive dot that controls an [`NormalParam`]
///
/// [`NormalParam`]: ../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct ModRangeInput<'a, Message, Theme: StyleSheet> {
    virtual_slider: VirtualSlider<'a, Message>,
    size: Length,
    enabled: bool,
    bipolar: bool,
    style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme> ModRangeInput<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`ModRangeInput`].
    ///
    /// * `normal_param` - The normalized value of the parameter.
    pub fn new(normal_param: impl Into<NormalParam>) -> Self
    where
        <Theme as StyleSheet>::Style: Default,
    {
        ModRangeInput {
            virtual_slider: VirtualSlider::new(normal_param.into()),
            size: Length::Fixed(DEFAULT_SIZE),
            enabled: true,
            bipolar: true,
            style: Default::default(),
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

    /// If `true`, then the value will move half as fast to compensate for the
    /// extended bipolar range.
    ///
    /// The default is `true`.
    pub const fn bipolar(mut self, bipolar: bool) -> Self {
        self.bipolar = bipolar;
        self
    }

    /// Sets the diameter of the [`ModRangeInput`]. The default size is
    /// `Length::from(Length::Fixed(31))`.
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub const fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`ModRangeInput`].
    ///
    /// [`ModRangeInput`]: struct.ModRangeInput.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Enable/disable this widget.
    ///
    /// The default is `true`.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ModRangeInput<'a, Message, Theme>
where
    Theme: StyleSheet,
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<virtual_slider::State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(virtual_slider::State::new(
            self.virtual_slider.param().normal,
            self.enabled,
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
        layout::Node::new(limits.resolve(self.size, self.size, Size::ZERO))
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
        let state = tree.state.downcast_mut::<virtual_slider::State>();
        let cursor_is_over = cursor.is_over(layout.bounds());

        if self
            .virtual_slider
            .update(
                state,
                self.enabled,
                cursor_is_over,
                false,
                self.bipolar,
                event,
                cursor,
                shell,
            )
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
        let is_over = cursor.is_over(layout.bounds());

        let appearance = if !self.enabled {
            theme.disabled(&self.style)
        } else if state.is_gesturing() {
            theme.gesturing(&self.style)
        } else if is_over {
            theme.hovered(&self.style)
        } else {
            theme.idle(&self.style)
        };

        match appearance {
            Appearance::Circle(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                let radius = bounds_size / 2.0;

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds_x,
                            y: bounds_y,
                            width: bounds_size,
                            height: bounds_size,
                        },
                        border: Border {
                            color: style.border_color,
                            width: style.border_width,
                            radius: Radius::new(radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    style.color,
                );
            }
            Appearance::Square(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds_x,
                            y: bounds_y,
                            width: bounds_size,
                            height: bounds_size,
                        },
                        border: Border {
                            color: style.border_color,
                            width: style.border_width,
                            radius: Radius::new(style.border_radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    style.color,
                );
            }
            Appearance::Invisible => {}
        };
    }
}

impl<'a, Message, Theme, Renderer> From<ModRangeInput<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: iced_core::Renderer,
{
    fn from(mod_range_input: ModRangeInput<'a, Message, Theme>) -> Self {
        Self::new(mod_range_input)
    }
}
