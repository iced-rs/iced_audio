//! Display an interactive horizontal slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use std::fmt::Debug;

use iced_native::{
    input::{keyboard, mouse, ButtonState},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{
    ModulationRange, Normal, Param, TextMarkGroup, TickMarkGroup,
};

static DEFAULT_HEIGHT: u16 = 14;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A horizontal slider GUI widget that controls a [`Param`]
///
/// An [`HSlider`] will try to fill the horizontal space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`HSlider`]: struct.HSlider.html
#[allow(missing_debug_implementations)]
pub struct HSlider<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    state: &'a mut State<ID>,
    on_change: Box<dyn Fn(ID) -> Message>,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    width: Length,
    height: Length,
    style: Renderer::Style,
    tick_marks: Option<&'a TickMarkGroup>,
    text_marks: Option<&'a TextMarkGroup>,
}

impl<'a, Message, Renderer: self::Renderer, ID>
    HSlider<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    /// Creates a new [`HSlider`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`HSlider`]
    ///   * a function that will be called when the [`HSlider`] is dragged.
    ///
    /// [`State`]: struct.State.html
    /// [`HSlider`]: struct.HSlider.html
    pub fn new<F>(state: &'a mut State<ID>, on_change: F) -> Self
    where
        F: 'static + Fn(ID) -> Message,
    {
        HSlider {
            state,
            on_change: Box::new(on_change),
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            width: Length::Fill,
            height: Length::from(Length::Units(DEFAULT_HEIGHT)),
            style: Renderer::Style::default(),
            tick_marks: None,
            text_marks: None,
        }
    }

    /// Sets the width of the [`HSlider`].
    /// The default width is `Length::Fill`.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`HSlider`].
    /// The default height is `Length::from(Length::Units(16))`.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`HSlider`].
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the modifier keys of the [`HSlider`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`HSlider`]: struct.HSlider.html
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
    /// [`HSlider`]: struct.HSlider.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/h_slider/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the [`TextMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `text_mark_style(&self) -> Option<TextMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`StyleSheet`]: ../../style/h_slider/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a TextMarkGroup) -> Self {
        self.text_marks = Some(text_marks);
        self
    }
}

/// The local state of an [`HSlider`].
///
/// [`HSlider`]: struct.HSlider.html
#[derive(Debug, Copy, Clone)]
pub struct State<ID: Debug + Copy + Clone> {
    /// The [`Param`] assigned to this widget
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    pub param: Param<ID>,
    /// An optional [`ModulationRange`] to assign to this widget
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    pub modulation_range: Option<ModulationRange>,
    is_dragging: bool,
    prev_drag_x: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl<ID: Debug + Copy + Clone> State<ID> {
    /// Creates a new [`HSlider`] state.
    ///
    /// It expects:
    /// * a [`Param`] to assign to this widget
    ///
    /// [`Param`]: ../../core/param/trait.Param.html
    /// [`HSlider`]: struct.HSlider.html
    pub fn new(param: Param<ID>) -> Self {
        Self {
            param,
            modulation_range: None,
            is_dragging: false,
            prev_drag_x: 0.0,
            continuous_normal: param.normal.value(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }

    /// Assigns an [`ModulationRange`] to this widget
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    pub fn modulation_range(
        mut self,
        modulation_range: ModulationRange,
    ) -> Self {
        self.modulation_range = Some(modulation_range);
        self
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for HSlider<'a, Message, Renderer, ID>
where
    Renderer: self::Renderer,
    ID: Debug + Copy + Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

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
                                self.state.prev_drag_x = cursor_position.x;
                            }
                            _ => {
                                self.state.is_dragging = false;

                                self.state.param.normal =
                                    self.state.param.default_normal;

                                messages.push((self.on_change)(
                                    self.state.param.id,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.param.normal.value();
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging {
                    let bounds_width = layout.bounds().width;
                    if bounds_width != 0.0 {
                        let mut movement_x = (cursor_position.x
                            - self.state.prev_drag_x)
                            / bounds_width;

                        if self
                            .state
                            .pressed_modifiers
                            .matches(self.modifier_keys)
                        {
                            movement_x *= self.modifier_scalar;
                        }

                        let normal = self.state.continuous_normal + movement_x;

                        self.state.continuous_normal = normal;
                        self.state.prev_drag_x = cursor_position.x;

                        self.state.param.normal = normal.into();

                        messages.push((self.on_change)(self.state.param.id));
                    }
                }
            }
            Event::Keyboard(keyboard::Event::Input { modifiers, .. }) => {
                self.state.pressed_modifiers = modifiers;
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
            self.state.param.normal,
            self.state.is_dragging,
            self.state.modulation_range,
            self.tick_marks,
            self.text_marks,
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

/// The renderer of an [`HSlider`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`HSlider`] in your user interface.
///
/// [`HSlider`]: struct.HSlider.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`HSlider`].
    ///
    /// It receives:
    ///   * the bounds of the [`HSlider`]
    ///   * the current cursor position
    ///   * the current normal of the [`HSlider`]
    ///   * whether the slider is currently being dragged
    ///   * any tick marks to display
    ///   * any text marks to display
    ///   * the style of the [`HSlider`]
    ///
    /// [`HSlider`]: struct.HSlider.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        modulation_range: Option<ModulationRange>,
        tick_marks: Option<&TickMarkGroup>,
        text_marks: Option<&TextMarkGroup>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer, ID> From<HSlider<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        h_slider: HSlider<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(h_slider)
    }
}
