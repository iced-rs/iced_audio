//! Display an interactive vertical slider that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced_native::{
    keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{ModulationRange, Normal, NormalParam};
use crate::native::{text_marks, tick_marks};

static DEFAULT_WIDTH: u16 = 14;
static DEFAULT_SCALAR: f32 = 0.98;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A vertical slider GUI widget that controls a [`NormalParam`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`VSlider`]: struct.VSlider.html
#[allow(missing_debug_implementations)]
pub struct VSlider<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    on_change: Box<dyn Fn(Normal) -> Message>,
    scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::ModifiersState,
    width: Length,
    height: Length,
    style: Renderer::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Renderer: self::Renderer> VSlider<'a, Message, Renderer> {
    /// Creates a new [`VSlider`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`VSlider`]
    ///   * a function that will be called when the [`VSlider`] is dragged.
    ///
    /// [`State`]: struct.State.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new<F>(state: &'a mut State, on_change: F) -> Self
    where
        F: 'static + Fn(Normal) -> Message,
    {
        VSlider {
            state,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::ModifiersState {
                control: true,
                ..Default::default()
            },
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::Fill,
            style: Renderer::Style::default(),
            tick_marks: None,
            text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
        }
    }

    /// Sets the width of the [`VSlider`].
    /// The default width is `Length::Units(14)`.
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

    /// Sets the scalar to use when the user drags the slider per pixel.
    ///
    /// For example, a scalar of `0.5` will cause the slider to move half a
    /// pixel for every pixel the mouse moves.
    ///
    /// The default scalar is `0.98`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
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
    /// [`VSlider`]: struct.VSlider.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a text_marks::Group) -> Self {
        self.text_marks = Some(text_marks);
        self
    }

    /// Sets a [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }

    /// Sets a second [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style_2(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range_2(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }
}

/// The local state of a [`VSlider`].
///
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone)]
pub struct State {
    /// The [`NormalParam`] assigned to this widget
    ///
    /// [`NormalParam`]: ../../core/normal_param/struct.Param.html
    pub normal_param: NormalParam,
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
    /// * a [`NormalParam`] to assign to this widget
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn new(normal_param: NormalParam) -> Self {
        Self {
            normal_param,
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normal_param.value.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for VSlider<'a, Message, Renderer>
where
    Renderer: self::Renderer,
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
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => {
                    if self.state.is_dragging {
                        let bounds_height = layout.bounds().height;

                        if bounds_height > 0.0 {
                            let mut movement_y = (cursor_position.y
                                - self.state.prev_drag_y)
                                / bounds_height;

                            if self
                                .state
                                .pressed_modifiers
                                .matches(self.modifier_keys)
                            {
                                movement_y *= self.modifier_scalar;
                            } else {
                                movement_y *= self.scalar;
                            }

                            let normal =
                                self.state.continuous_normal - movement_y;

                            self.state.continuous_normal = normal;
                            self.state.prev_drag_y = cursor_position.y;

                            self.state.normal_param.value = normal.into();

                            messages.push((self.on_change)(
                                self.state.normal_param.value,
                            ));
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
                                self.state.prev_drag_y = cursor_position.y;
                            }
                            _ => {
                                self.state.is_dragging = false;

                                self.state.normal_param.value =
                                    self.state.normal_param.default;

                                messages.push((self.on_change)(
                                    self.state.normal_param.value,
                                ));
                            }
                        }

                        self.state.last_click = Some(click);
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.state.is_dragging = false;
                    self.state.continuous_normal =
                        self.state.normal_param.value.as_f32();
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
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.state.normal_param.value,
            self.state.is_dragging,
            self.mod_range_1,
            self.mod_range_2,
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
    ///   * the height of the handle in pixels
    ///   * whether the slider is currently being dragged
    ///   * any tick marks to display
    ///   * any text marks to display
    ///   * the style of the [`VSlider`]
    ///
    /// [`VSlider`]: struct.VSlider.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        mod_range_1: Option<&ModulationRange>,
        mod_range_2: Option<&ModulationRange>,
        tick_marks: Option<&tick_marks::Group>,
        text_marks: Option<&text_marks::Group>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<VSlider<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        v_slider: VSlider<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(v_slider)
    }
}
