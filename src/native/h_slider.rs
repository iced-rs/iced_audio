//! Display an interactive horizontal slider

use iced_native::{
    input::{mouse, ButtonState},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};
use iced_wgpu::{Defaults, Primitive};

use std::{hash::Hash};

/// A horizontal slider
///
/// An [`HSlider`] will try to fill the horizontal space of its container.
///
/// [`HSlider`]: struct.HSlider.html
#[allow(missing_debug_implementations)]
pub struct HSlider<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message>,
    width: Length,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer> HSlider<'a, Message, Renderer> {
    /// Creates a new [`HSlider`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`HSlider`]
    ///   * the current value between 0 and 1
    ///   * a function that will be called when the [`HSlider`] is dragged.
    ///   It receives the new value of the [`HSlider`] and must produce a
    ///   `Message`.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn new<F>(
        state: &'a mut State,
        value: f32,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(f32) -> Message,
    {
        HSlider {
            state,
            value,
            on_change: Box::new(on_change),
            width: Length::Fill,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`HSlider`].
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the style of the [`HSlider`].
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

/// The local state of an [`HSlider`].
///
/// [`HSlider`]: struct.HSlider.html
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct State {
    is_dragging: bool,
    drag_offset_x: f32,
}

impl State {
    /// Creates a new [`HSlider`] state.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn new() -> State {
        State::default()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for HSlider<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .width(self.width)
            .height(Length::Units(renderer.height() as u16));
        
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
        let mut change = || {
            let bounds = layout.bounds();

            if self.state.is_dragging {
                let mut value = (
                                    (cursor_position.x - bounds.x)
                                    - self.state.drag_offset_x
                                    ) / bounds.width;
                
                if value < 0.0 { value = 0.0; }
                else if value > 1.0 { value = 1.0; }
                
                messages.push((self.on_change)(value));
            }
        };

        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state,
            }) => match state {
                ButtonState::Pressed => {
                    if layout.bounds().contains(cursor_position) {
                        change();

                        let bounds = layout.bounds();

                        self.state.is_dragging = true;
                        self.state.drag_offset_x =
                            cursor_position.x - 
                            ((self.value * bounds.width) + bounds.x);
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                    self.state.drag_offset_x = 0.0;
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging {
                    change();
                }
            }
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
            self.value,
            self.state.is_dragging,
            &self.style,
        )
    }

/// test
    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
    }
}

/// The renderer of an [`HSlider`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use an [`HSlider`] in your user interface.
///
/// [`HSlider`]: struct.HSlider.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;
    
    /// Returns the height of the [`HSlider`].
    ///
    /// [`HSlider`]: struct.HSlider.html
    fn height(&self) -> u32;

    /// Draws an [`HSlider`].
    ///
    /// It receives:
    ///   * the bounds of the [`HSlider`]
    ///   * the current cursor position
    ///   * the current value of the [`HSlider`]
    ///   * the local state of the [`HSlider`]
    ///
    /// [`HSlider`]: struct.HSlider.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        value: f32,
        is_dragging: bool,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<HSlider<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        h_slider: HSlider<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(h_slider)
    }
}