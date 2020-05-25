//! Display an interactive vertical slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use std::fmt::Debug;

use iced_native::{
    input::{mouse, ButtonState, keyboard},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, Param, TickMarkGroup};

static DEFAULT_WIDTH: u16 = 14;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A vertical slider GUI widget that controls a [`Param`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`VSlider`]: struct.VSlider.html
#[allow(missing_debug_implementations)]
pub struct VSlider<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone
{
    state: &'a mut State,
    id: ID,
    normal: Normal,
    default_normal: Normal,
    on_change: Box<dyn Fn((ID, Normal)) -> Message>,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    width: Length,
    height: Length,
    style: Renderer::Style,
    tick_marks: Option<&'a TickMarkGroup>,
}

impl<'a, Message, Renderer: self::Renderer, ID>
    VSlider<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone
{
    /// Creates a new [`VSlider`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`VSlider`]
    ///   * a [`Param`] with the current and default values
    ///   * a function that will be called when the [`VSlider`] is dragged.
    ///   It receives the parameter's `ID` and the new [`Normal`] of the
    /// [`VSlider`].
    /// `ID` is a user supplied type. It can be an `enum`, `u32`, `i32`,
    /// `String`, etc. Each parameter must have a unique `ID` value!
    ///
    /// [`State`]: struct.State.html
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new<F>(
        state: &'a mut State,
        param: &impl Param<ID=ID>,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn((ID, Normal)) -> Message,
    {  
        VSlider {
            state,
            id: param.id(),
            normal: param.normal(),
            default_normal: param.default_normal(),
            on_change: Box::new(on_change),
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::Fill,
            style: Renderer::Style::default(),
            tick_marks: None,
        }
    }

    /// Sets the width of the [`VSlider`].
    /// The default width is `Length::from(Length::Units(16))`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`VSlider`].
    /// The default height is `Length::Fill`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`VSlider`].
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the modifier keys of the [`VSlider`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }
}

/// The local state of a [`VSlider`].
///
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Copy, Clone)]
pub struct State {
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl State {
    /// Creates a new [`VSlider`] state.
    ///
    /// It expects:
    /// * a [`Param`] with the initial value
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new<ID>(param: &impl Param<ID=ID>) -> Self {
        Self {
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: param.normal().value(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for VSlider<'a, Message, Renderer, ID>
where
    Renderer: self::Renderer,
    ID: Debug + Copy + Clone,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
            let limits = limits
            .width(self.width)
            .height(self.height);
        
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
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state,
            }) => match state {
                ButtonState::Pressed => {
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

                                messages.push((self.on_change)(
                                    (self.id, self.default_normal)
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal = self.normal.value();
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging {
                    let bounds_height = layout.bounds().height;
                    if bounds_height != 0.0 {
                        let mut movement_y =
                            (cursor_position.y - self.state.prev_drag_y)
                                / bounds_height;

                        if self.state.pressed_modifiers.matches(
                            self.modifier_keys) {
                            movement_y *= self.modifier_scalar;
                        }

                        let normal =
                            self.state.continuous_normal - movement_y;

                        self.state.continuous_normal = normal;
                        self.state.prev_drag_y = cursor_position.y;

                        messages.push((self.on_change)(
                            (self.id, normal.into())
                        ));
                    }
                }
            },
            Event::Keyboard(keyboard::Event::Input {
                modifiers,
                ..
            }) => {
                self.state.pressed_modifiers = modifiers;
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
            self.normal,
            self.state.is_dragging,
            self.tick_marks,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

/// The renderer of a [`VSlider`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`VSlider`] in your user interface.
///
/// [`VSlider`]: struct.VSlider.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`VSlider`].
    ///
    /// It receives:
    ///   * the bounds of the [`VSlider`]
    ///   * the current cursor position
    ///   * the current normal of the [`VSlider`]
    ///   * the local state of the [`VSlider`]
    ///   * the style of the [`VSlider`]
    ///
    /// [`VSlider`]: struct.VSlider.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        tick_marks: Option<&TickMarkGroup>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer, ID>
    From<VSlider<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        v_slider: VSlider<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(v_slider)
    }
}