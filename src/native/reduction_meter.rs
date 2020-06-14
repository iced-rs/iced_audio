//! Display a visualizer that displays average/peak reduction levels.

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, TickMarkGroup};

static DEFAULT_WIDTH: u16 = 10;

/// The orientation of a [`ReductionMeter`]
///
/// [`ReductionMeter`]: struct.ReductionMeter.html
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    /// Vertical orientation
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

/// A visualizer that displays average/peak reduction levels.
///
/// A [`ReductionMeter`] will try to fill the length of its container.
///
/// [`ReductionMeter`]: struct.ReductionMeter.html
#[allow(missing_debug_implementations)]
pub struct ReductionMeter<'a, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    style: Renderer::Style,
    orientation: Orientation,
    tick_marks: Option<&'a TickMarkGroup>,
}

impl<'a, Renderer: self::Renderer> ReductionMeter<'a, Renderer> {
    /// Creates a new [`ReductionMeter`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`ReductionMeter`]
    ///
    /// [`State`]: struct.State.html
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn new(state: &'a mut State) -> Self {
        ReductionMeter {
            state,
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::Fill,
            style: Renderer::Style::default(),
            orientation: Orientation::Vertical,
            tick_marks: None,
        }
    }

    /// Sets the [`Orientation`] of the [`ReductionMeter`].
    ///
    /// [`Orientation`]: enum.Orientation.html
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        if self.orientation != orientation {
            self.orientation = orientation;
            let temp_height = self.height;
            self.height = self.width;
            self.width = temp_height;
        }
        self
    }

    /// Sets the width of the [`ReductionMeter`].
    ///
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`ReductionMeter`].
    ///
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`ReductionMeter`].
    ///
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/reduction_meter/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }
}

/// The local state of a [`ReductionMeter`].
///
/// [`ReductionMeter`]: struct.ReductionMeter.html
#[derive(Debug, Copy, Clone)]
pub struct State {
    /// The [`Normal`] position of the meter bar.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub bar_normal: Normal,
    /// The [`Normal`] position of the peak line. Set this to
    /// `None` for no peak line.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub peak_normal: Option<Normal>,
}

impl State {
    /// Creates a new [`ReductionMeter`] state.
    ///
    /// * `normal` - The current position of the meter bar.
    /// * `peak_normal` - The current position of the peak line of the meter.
    /// Set this to `None` for no peak line.
    ///
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    pub fn new(bar_normal: Normal, peak_normal: Option<Normal>) -> Self {
        Self {
            bar_normal,
            peak_normal,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for ReductionMeter<'a, Renderer>
where
    Renderer: self::Renderer,
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
        _event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            self.state.bar_normal,
            self.state.peak_normal,
            &self.orientation,
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

/// The renderer of a [`ReductionMeter`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`ReductionMeter`] in your user interface.
///
/// [`ReductionMeter`]: struct.ReductionMeter.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`ReductionMeter`].
    ///
    /// It receives:
    ///   * the bounds of the [`ReductionMeter`]
    ///   * the `Normal` position of the meter bar
    ///   * the `Normal` position of the peak line
    ///   * the orientation of the [`ReductionMeter`]
    ///   * any tick marks to display
    ///   * the style of the [`ReductionMeter`]
    ///
    /// [`ReductionMeter`]: struct.ReductionMeter.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        bar_normal: Normal,
        peak_normal: Option<Normal>,
        orientation: &Orientation,
        tick_marks: Option<&TickMarkGroup>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<ReductionMeter<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        reduction_meter: ReductionMeter<'a, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(reduction_meter)
    }
}
