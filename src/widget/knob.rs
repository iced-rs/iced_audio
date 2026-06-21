//! Display an interactive rotating knob that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

mod bipolar_state;
mod draw;
mod knob_info;
mod value_markers;

use crate::{
    core::{ModulationRange, Normal, NormalParam},
    text_marks, tick_marks,
    virtual_slider::{self, Gesture, VirtualSlider},
};
use iced_core::{
    Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Size, Widget, layout, mouse,
    renderer::Style,
    widget::{Tree, tree},
};
use knob_info::KnobInfo;
use value_markers::ValueMarkers;

pub use crate::style::knob::{
    Appearance, ArcAppearance, ArcBipolarAppearance, CircleAppearance, CircleNotch, LineNotch,
    ModRangeArcAppearance, NotchShape, StyleLength, StyleSheet, TextMarksAppearance,
    TickMarksAppearance, ValueArcAppearance,
};

const DEFAULT_SIZE: f32 = 30.0;

/// A rotating knob GUI widget that controls a [`NormalParam`]
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Theme: StyleSheet> {
    virtual_slider: VirtualSlider<'a, Message>,
    enabled: bool,
    size: Length,
    bipolar_center: Option<Normal>,
    style: <Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Theme: StyleSheet> Knob<'a, Message, Theme> {
    /// Creates a new [`Knob`].
    ///
    /// * `normal_param` - The normalized value of the parameter.
    pub fn new(normal_param: impl Into<NormalParam>) -> Self
    where
        <Theme as StyleSheet>::Style: Default,
    {
        Knob {
            virtual_slider: VirtualSlider::new(normal_param.into()),
            enabled: true,
            size: Length::Fixed(DEFAULT_SIZE),
            bipolar_center: None,
            style: Default::default(),
            tick_marks: None,
            text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
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
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Knob<'a, Message, Theme>
where
    Theme: StyleSheet,
    Renderer: iced_core::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_graphics::geometry::Renderer,
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
        let cursor_is_over = cursor.is_over(bounds);
        let angle_range = theme.angle_range(&self.style);
        let normal_val = self.virtual_slider.param().normal;

        let appearance = if state.is_gesturing() {
            theme.gesturing(&self.style)
        } else if cursor_is_over {
            theme.hovered(&self.style)
        } else {
            theme.idle(&self.style)
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
        let value_angle = start_angle + (normal_val.scale(angle_span));

        let knob_info = KnobInfo {
            bounds,
            start_angle,
            angle_span,
            radius,
            value: normal_val,
            bipolar_center: self.bipolar_center,
            value_angle,
        };

        match appearance {
            Appearance::Circle(style) => {
                draw::circle_style(renderer, &knob_info, style, &value_markers)
            }
            Appearance::Arc(style) => draw::arc_style(renderer, &knob_info, style, &value_markers),

            Appearance::ArcBipolar(style) => {
                draw::arc_bipolar_style(renderer, &knob_info, style, &value_markers)
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Knob<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: iced_core::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_graphics::geometry::Renderer,
{
    fn from(knob: Knob<'a, Message, Theme>) -> Self {
        Self::new(knob)
    }
}
