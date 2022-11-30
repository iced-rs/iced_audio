//! Display an interactive vertical slider that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced_native::widget::tree::{self, Tree};
use iced_native::{
    event, keyboard, layout, mouse, touch, Clipboard, Element, Event, Layout,
    Length, Point, Rectangle, Shell, Size, Widget,
};

use crate::core::{ModulationRange, Normal, NormalParam};
use crate::native::{text_marks, tick_marks};
use crate::style::v_slider::StyleSheet;

static DEFAULT_WIDTH: u16 = 14;
static DEFAULT_SCALAR: f32 = 0.9575;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A vertical slider GUI widget that controls a [`NormalParam`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
/// [`VSlider`]: struct.VSlider.html
#[allow(missing_debug_implementations)]
pub struct VSlider<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    normal_param: NormalParam,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    width: Length,
    height: Length,
    style: <Renderer::Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Renderer> VSlider<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`VSlider`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`VSlider`]
    ///   * a function that will be called when the [`VSlider`] is dragged.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`VSlider`]: struct.VSlider.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'static + Fn(Normal) -> Message,
    {
        VSlider {
            normal_param,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::Fill,
            style: Default::default(),
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
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the modifier keys of the [`VSlider`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the slider per pixel.
    ///
    /// For example, a scalar of `0.5` will cause the slider to move half a
    /// pixel for every pixel the mouse moves.
    ///
    /// The default scalar is `0.9575`.
    ///
    /// [`VSlider`]: struct.VSlider.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`VSlider`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`VSlider`]: struct.VSlider.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
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

    fn move_virtual_slider(
        &mut self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        mut normal_delta: f32,
    ) {
        if normal_delta.abs() < f32::EPSILON {
            return;
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        self.normal_param.value =
            Normal::new(state.continuous_normal - normal_delta);
        state.continuous_normal = self.normal_param.value.as_f32();

        shell.publish((self.on_change)(self.normal_param.value));
    }
}

/// The local state of a [`VSlider`].
///
/// [`VSlider`]: struct.VSlider.html
#[derive(Debug, Clone)]
struct State {
    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
    tick_marks_cache: crate::graphics::tick_marks::PrimitiveCache,
    text_marks_cache: crate::graphics::text_marks::PrimitiveCache,
}

impl State {
    /// Creates a new [`VSlider`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`VSlider`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`VSlider`]: struct.VSlider.html
    fn new(normal: Normal) -> Self {
        Self {
            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
            tick_marks_cache: Default::default(),
            text_marks_cache: Default::default(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for VSlider<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param.value))
    }

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
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let state = state.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.is_dragging {
                    let bounds = layout.bounds();
                    if bounds.height > 0.0 {
                        let normal_delta = (cursor_position.y
                            - state.prev_drag_y)
                            / bounds.height
                            * self.scalar;

                        state.prev_drag_y = if cursor_position.y <= bounds.y {
                            bounds.y
                        } else {
                            cursor_position.y.min(bounds.y + bounds.height)
                        };

                        self.move_virtual_slider(state, shell, normal_delta);

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.wheel_scalar == 0.0 {
                    return event::Status::Ignored;
                }

                if layout.bounds().contains(cursor_position) {
                    let lines = match delta {
                        iced_native::mouse::ScrollDelta::Lines {
                            y, ..
                        } => y,
                        iced_native::mouse::ScrollDelta::Pixels {
                            y, ..
                        } => {
                            if y > 0.0 {
                                1.0
                            } else if y < 0.0 {
                                -1.0
                            } else {
                                0.0
                            }
                        }
                    };

                    if lines != 0.0 {
                        let normal_delta = -lines * self.wheel_scalar;

                        self.move_virtual_slider(state, shell, normal_delta);

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    let click =
                        mouse::Click::new(cursor_position, state.last_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            state.is_dragging = true;
                            state.prev_drag_y = cursor_position.y;
                            state.continuous_normal =
                                self.normal_param.value.as_f32();
                        }
                        _ => {
                            state.is_dragging = false;

                            self.normal_param.value = self.normal_param.default;
                            state.continuous_normal =
                                self.normal_param.default.as_f32();

                            shell.publish((self.on_change)(
                                self.normal_param.value,
                            ));
                        }
                    }

                    state.last_click = Some(click);

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.is_dragging {
                    state.is_dragging = false;
                    state.continuous_normal = self.normal_param.value.as_f32();

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                _ => {}
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let state = state.state.downcast_ref::<State>();
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.normal_param.value,
            state.is_dragging,
            self.mod_range_1,
            self.mod_range_2,
            self.tick_marks,
            self.text_marks,
            theme,
            &self.style,
            &state.tick_marks_cache,
            &state.text_marks_cache,
        )
    }
}

/// The renderer of a [`VSlider`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`VSlider`] in your user interface.
///
/// [`VSlider`]: struct.VSlider.html
pub trait Renderer: iced_native::Renderer
where
    Self::Theme: StyleSheet,
{
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
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
        tick_marks_cache: &crate::tick_marks::PrimitiveCache,
        text_marks_cache: &crate::text_marks::PrimitiveCache,
    );
}

impl<'a, Message, Renderer> From<VSlider<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer,
    Renderer::Theme: 'a + StyleSheet,
{
    fn from(
        v_slider: VSlider<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(v_slider)
    }
}
