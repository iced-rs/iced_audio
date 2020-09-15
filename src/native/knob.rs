//! Display an interactive rotating knob that controls a [`Param`]
//!
//! [`Param`]: ../core/param/struct.Param.html

use std::fmt::Debug;

use iced_native::{
    keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{
    ModulationRange, Normal, Param, TextMarkGroup, TickMarkGroup,
};

static DEFAULT_SIZE: u16 = 30;
static DEFAULT_SCALAR: f32 = 0.005;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A rotating knob GUI widget that controls a [`Param`]
///
/// [`Param`]: ../../core/param/struct.Param.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Renderer: self::Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    state: &'a mut State<ID>,
    size: Length,
    on_change: Box<dyn Fn(ID) -> Message>,
    scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    style: Renderer::Style,
    tick_marks: Option<&'a TickMarkGroup>,
    text_marks: Option<&'a TextMarkGroup>,
}

impl<'a, Message, Renderer: self::Renderer, ID> Knob<'a, Message, Renderer, ID>
where
    ID: Debug + Copy + Clone,
{
    /// Creates a new [`Knob`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`Knob`]
    ///   * a function that will be called when the [`Knob`] is turned.
    ///
    /// [`State`]: struct.State.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<F>(state: &'a mut State<ID>, on_change: F) -> Self
    where
        F: 'static + Fn(ID) -> Message,
    {
        Knob {
            state,
            size: Length::from(Length::Units(DEFAULT_SIZE)),
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            style: Renderer::Style::default(),
            tick_marks: None,
            text_marks: None,
        }
    }

    /// Sets the diameter of the [`Knob`]. The default size is
    /// `Length::from(Length::Units(31))`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.005`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets the modifier keys of the [`Knob`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_keys(
        mut self,
        modifier_keys: keyboard::ModifiersState,
    ) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the knobs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Knob::scalar()` (which the default is `0.008`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the knob to turn
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the [`TextMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `text_mark_style(&self) -> Option<TextMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a TextMarkGroup) -> Self {
        self.text_marks = Some(text_marks);
        self
    }
}

/// The local state of a [`Knob`].
///
/// [`Knob`]: struct.Knob.html
#[derive(Debug, Copy, Clone)]
pub struct State<ID: Debug + Copy + Clone> {
    /// The [`Param`] assigned to this widget
    ///
    /// [`Param`]: ../../core/param/struct.Param.html
    pub param: Param<ID>,
    /// An optional [`ModulationRange`] to assign to this widget
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    pub modulation_range: Option<ModulationRange>,
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::ModifiersState,
    last_click: Option<mouse::Click>,
}

impl<ID: Debug + Copy + Clone> State<ID> {
    /// Creates a new [`Knob`] state.
    ///
    /// It expects:
    /// * a [`Param`] to assign to this widget
    ///
    /// [`Param`]: ../../core/param/struct.Param.html
    /// [`Knob`]: struct.Knob.html
    pub fn new(param: Param<ID>) -> Self {
        Self {
            param,
            modulation_range: None,
            is_dragging: false,
            prev_drag_y: 0.0,
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

    /// Returns the [`Normal`] value of the [`Param`]
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`Param`]: ../../core/param/struct.Param.html
    pub fn normal(&mut self) -> &mut Normal {
        &mut self.param.normal
    }
}

impl<'a, Message, Renderer, ID> Widget<Message, Renderer>
    for Knob<'a, Message, Renderer, ID>
where
    Renderer: self::Renderer,
    ID: Debug + Copy + Clone,
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
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => {
                    if self.state.is_dragging && cursor_position.y != -1.0 {
                        let mut movement_y = (cursor_position.y
                            - self.state.prev_drag_y)
                            * self.scalar;

                        if self
                            .state
                            .pressed_modifiers
                            .matches(self.modifier_keys)
                        {
                            movement_y *= self.modifier_scalar;
                        }

                        let mut normal =
                            self.state.continuous_normal - movement_y;

                        if normal < 0.0 {
                            normal = 0.0;
                        } else if normal > 1.0 {
                            normal = 1.0;
                        }

                        self.state.continuous_normal = normal;
                        self.state.prev_drag_y = cursor_position.y;

                        self.state.param.normal = normal.into();

                        messages.push((self.on_change)(self.state.param.id));
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
                                self.state.prev_drag_y = cursor_position.y;
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
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.param.normal.value();
                }
                _ => {}
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    self.state.pressed_modifiers = modifiers;
                }
                _ => {}
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

        self.size.hash(state);
    }
}

/// The renderer of a [`Knob`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Knob`] in your user interface.
///
/// [`Knob`]: struct.Knob.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Knob`].
    ///
    /// It receives:
    ///   * the bounds of the [`Knob`]
    ///   * the current cursor position
    ///   * the current normal of the [`Knob`]
    ///   * whether the knob is currently being dragged
    ///   * any tick marks to display
    ///   * any text marks to display
    ///   * the style of the [`Knob`]
    ///
    /// [`Knob`]: struct.Knob.html
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

impl<'a, Message, Renderer, ID> From<Knob<'a, Message, Renderer, ID>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
    ID: 'a + Debug + Copy + Clone,
{
    fn from(
        knob: Knob<'a, Message, Renderer, ID>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(knob)
    }
}
