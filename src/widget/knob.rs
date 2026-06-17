//! Display an interactive rotating knob that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

mod bipolar_state;
mod draw;
mod knob_info;
mod state;
mod value_markers;

use crate::{
    core::{ModulationRange, Normal, NormalParam, SliderStatus},
    text_marks, tick_marks,
};
use iced::{
    advanced::{
        graphics::core::{keyboard, touch},
        layout, mouse,
        renderer::Style,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    Element, Event, Length, Rectangle, Renderer, Size,
};
use knob_info::KnobInfo;
use state::State;
use value_markers::ValueMarkers;

pub use crate::style::knob::{
    Appearance, ArcAppearance, ArcBipolarAppearance, CircleAppearance, CircleNotch, LineCap,
    LineNotch, ModRangeArcAppearance, NotchShape, StyleLength, StyleSheet, TextMarksAppearance,
    TickMarksAppearance, ValueArcAppearance,
};

static DEFAULT_SIZE: f32 = 30.0;
static DEFAULT_SCALAR: f32 = 0.00385;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A rotating knob GUI widget that controls a [`NormalParam`]
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    size: Length,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    bipolar_center: Option<Normal>,
    style: <Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Theme> Knob<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`Knob`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`Knob`]
    ///   * a function that will be called when the [`Knob`] is turned.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal) -> Message,
        <Theme as StyleSheet>::Style: Default,
    {
        Knob {
            normal_param,
            size: Length::Fixed(DEFAULT_SIZE),
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            bipolar_center: None,
            style: Default::default(),
            tick_marks: None,
            text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
        }
    }

    /// Sets the grab message of the [`Knob`].
    /// This is called when the mouse grabs from the knob.
    ///
    /// Typically, the user's interaction with the knob starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(mut self, on_grab: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`Knob`].
    /// This is called when the mouse is released from the knob.
    ///
    /// Typically, the user's interaction with the knob is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the knob's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: impl 'a + FnMut() -> Option<Message>) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the diameter of the [`Knob`]. The default size is
    /// `Length::from(Length::Fixed(31))`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.00385`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the modifier keys of the [`Knob`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the knobs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Knob::scalar()` (which the default is `0.00385`).
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

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
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
    pub fn mod_range(mut self, mod_range: Option<&'a ModulationRange>) -> Self {
        self.mod_range_1 = mod_range;
        self
    }

    /// Sets a second [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style_2(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range_2(mut self, mod_range: Option<&'a ModulationRange>) -> Self {
        self.mod_range_2 = mod_range;
        self
    }

    /// Sets the value to be considered the center of the [`Knob`]. Only has
    /// an effect when using [`ArcBipolarStyle`].
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`ArcBipolarStyle`]: ../../style/knob/struct.ArcBipolarStyle.html
    pub fn bipolar_center(mut self, bipolar_center: Normal) -> Self {
        self.bipolar_center = Some(bipolar_center);
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

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for Knob<'a, Message, Theme>
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

                shell.capture_event();
                shell.request_redraw();
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

                        shell.capture_event();
                        shell.request_redraw();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if is_over {
                    let click = mouse::Click::new(
                        cursor.position().unwrap(),
                        mouse::Button::Left,
                        state.last_click,
                    );

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            self.maybe_fire_on_grab(shell);

                            state.dragging_status = Some(Default::default());
                            state.prev_drag_y = cursor.position().unwrap().y;
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

                    shell.capture_event();
                    shell.request_redraw();
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
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = *modifiers;
                    shell.capture_event();
                    shell.request_redraw();
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

        let is_over = cursor.is_over(bounds);

        let angle_range = theme.angle_range(&self.style);

        let appearance = if state.dragging_status.is_some() {
            theme.dragging(&self.style)
        } else if is_over {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        let value_markers = ValueMarkers {
            tick_marks: self.tick_marks,
            text_marks: self.text_marks,
            mod_range_1: self.mod_range_1,
            mod_range_2: self.mod_range_2,
            tick_marks_style: theme.tick_marks_appearance(&self.style),
            text_marks_style: theme.text_marks_appearance(&self.style),
            value_arc_style: theme.value_arc_appearance(&self.style),
            mod_range_style_1: theme.mod_range_arc_appearance(&self.style),
            mod_range_style_2: theme.mod_range_arc_appearance_2(&self.style),
        };

        let bounds = {
            let bounds = Rectangle {
                x: bounds.x.round(),
                y: bounds.y.round(),
                width: bounds.width.round(),
                height: bounds.height.round(),
            };

            if bounds.width == bounds.height {
                bounds
            } else if bounds.width > bounds.height {
                Rectangle {
                    x: (bounds.x + (bounds.width - bounds.height) / 2.0).round(),
                    y: bounds.y,
                    width: bounds.height,
                    height: bounds.height,
                }
            } else {
                Rectangle {
                    x: bounds.x,
                    y: (bounds.y + (bounds.height - bounds.width) / 2.0).round(),
                    width: bounds.width,
                    height: bounds.width,
                }
            }
        };

        let radius = bounds.width / 2.0;

        let start_angle = if angle_range.min() >= crate::core::math::THREE_HALVES_PI {
            angle_range.min() - crate::core::math::THREE_HALVES_PI
        } else {
            angle_range.min() + std::f32::consts::FRAC_PI_2
        };
        let angle_span = angle_range.max() - angle_range.min();
        let value_angle = start_angle + (self.normal_param.value.scale(angle_span));

        let knob_info = KnobInfo {
            bounds,
            start_angle,
            angle_span,
            radius,
            value: self.normal_param.value,
            bipolar_center: self.bipolar_center,
            value_angle,
        };

        match appearance {
            Appearance::Circle(style) => draw::circle_style(
                renderer,
                &knob_info,
                style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
            Appearance::Arc(style) => draw::arc_style(
                renderer,
                &knob_info,
                style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),

            Appearance::ArcBipolar(style) => draw::arc_bipolar_style(
                renderer,
                &knob_info,
                style,
                &value_markers,
                //tick_marks_cache,
                //text_marks_cache,
            ),
        }
    }
}

impl<'a, Message, Theme> From<Knob<'a, Message, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(knob: Knob<'a, Message, Theme>) -> Self {
        Self::new(knob)
    }
}
