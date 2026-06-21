//! Display an interactive horizontal slider that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.Param.html

mod draw;
mod value_markers;

use crate::{
    core::{ModulationRange, NormalParam},
    text_marks, tick_marks,
    virtual_slider::{self, Gesture, VirtualSlider},
};
use iced_core::{
    Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Size, Widget,
    layout::{self, Limits},
    mouse,
    renderer::Style,
    widget::{Tree, tree},
};
use value_markers::ValueMarkers;

pub use crate::style::h_slider::{
    Appearance, ClassicAppearance, ClassicHandle, ClassicRail, ModRangeAppearance,
    ModRangePlacement, RectAppearance, RectBipolarAppearance, StyleSheet, TextMarksAppearance,
    TextureAppearance, TickMarksAppearance,
};

const DEFAULT_HEIGHT: f32 = 14.0;

/// A horizontal slider GUI widget that controls a [`NormalParam`]
///
/// an [`HSlider`] will try to fill the horizontal space of its container.
///
/// [`NormalParam`]: ../../core/normal_param/struct.Param.html
/// [`HSlider`]: struct.HSlider.html
#[allow(missing_debug_implementations)]
pub struct HSlider<'a, Message, Theme: StyleSheet> {
    virtual_slider: VirtualSlider<'a, Message>,
    drag_horizontally: bool,
    enabled: bool,
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Theme> HSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    /// Creates a new [`HSlider`].
    ///
    /// * `normal_param` - The normalized value of the parameter.
    pub fn new(normal_param: impl Into<NormalParam>) -> Self
    where
        <Theme as StyleSheet>::Style: Default,
    {
        HSlider {
            virtual_slider: VirtualSlider::new(normal_param.into()),
            drag_horizontally: true,
            enabled: true,
            width: Length::Fill,
            height: Length::Fixed(DEFAULT_HEIGHT),
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

    /// Set whether this slider is dragged horizontally (true) or vertically (false).
    ///
    /// The default value is `true`.
    pub fn drag_horizontally(mut self, drag_horizontally: bool) -> Self {
        self.drag_horizontally = drag_horizontally;
        self
    }

    /// Enable/disable this widget.
    ///
    /// The default is `true`.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the width of the [`HSlider`].
    ///
    /// The default height is `Length::Fill`.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`HSlider`].
    ///
    /// The default height is `Length::Fixed(14)`.
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`HSlider`].
    ///
    /// [`HSlider`]: struct.HSlider.html
    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/h_slider/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/h_slider/trait.StyleSheet.html
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
}

impl<'a, Message, Theme: StyleSheet, Renderer> Widget<Message, Theme, Renderer>
    for HSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
    Renderer: iced_core::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::image::Renderer<Handle = iced_core::image::Handle>,
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
            width: Length::Shrink,
            height: self.height,
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> layout::Node {
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
            .update(
                state,
                cursor_is_over,
                self.drag_horizontally,
                false,
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
        let cursor_is_over = cursor.is_over(bounds);
        let normal_val = self.virtual_slider.param().normal;

        let appearance = if state.is_gesturing() {
            theme.gesturing(&self.style)
        } else if cursor_is_over {
            theme.hovered(&self.style)
        } else {
            theme.idle(&self.style)
        };

        let bounds = Rectangle {
            x: bounds.x.round(),
            y: bounds.y.round(),
            width: bounds.width.round(),
            height: bounds.height.round(),
        };

        let value_markers = ValueMarkers {
            tick_marks: self.tick_marks,
            text_marks: self.text_marks,
            mod_range_1: self.mod_range_1,
            mod_range_2: self.mod_range_2,
            tick_marks_style: theme.tick_marks_appearance(&self.style),
            text_marks_style: theme.text_marks_appearance(&self.style),
            mod_range_style_1: theme.mod_range_appearance(&self.style),
            mod_range_style_2: theme.mod_range_appearance_2(&self.style),
        };

        match appearance {
            Appearance::Texture(style) => {
                draw::texture_style(renderer, normal_val, &bounds, style, &value_markers)
            }
            Appearance::Classic(style) => {
                draw::classic_style(renderer, normal_val, &bounds, &style, &value_markers)
            }
            Appearance::Rect(style) => {
                draw::rect_style(renderer, normal_val, &bounds, &style, &value_markers)
            }
            Appearance::RectBipolar(style) => {
                draw::rect_bipolar_style(renderer, normal_val, &bounds, &style, &value_markers)
            }
        };
    }
}

impl<'a, Message, Theme, Renderer> From<HSlider<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: iced_core::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::image::Renderer<Handle = iced_core::image::Handle>,
{
    fn from(h_slider: HSlider<'a, Message, Theme>) -> Self {
        Self::new(h_slider)
    }
}
