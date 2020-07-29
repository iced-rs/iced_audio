//! Display a visualizer that displays the phase correlation of a stereo signal.

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, TextMarkGroup, TickMarkGroup};

static DEFAULT_HEIGHT: u16 = 8;

/// The orientation of a [`PhaseMeter`]
///
/// [`PhaseMeter`]: struct.PhaseMeter.html
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    /// Horizontal orientation
    Horizontal,
    /// Vertical orientation
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

/// A visualizer that displays the phase correlation of a stereo signal.
///
/// A [`PhaseMeter`] will try to fill the length of its container.
///
/// [`PhaseMeter`]: struct.PhaseMeter.html
#[allow(missing_debug_implementations)]
pub struct PhaseMeter<'a, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    style: Renderer::Style,
    orientation: Orientation,
    tick_marks: Option<&'a TickMarkGroup>,
    text_marks: Option<&'a TextMarkGroup>,
}

impl<'a, Renderer: self::Renderer> PhaseMeter<'a, Renderer> {
    /// Creates a new [`PhaseMeter`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`PhaseMeter`]
    ///
    /// [`State`]: struct.State.html
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn new(state: &'a mut State) -> Self {
        PhaseMeter {
            state,
            width: Length::Fill,
            height: Length::from(Length::Units(DEFAULT_HEIGHT)),
            style: Renderer::Style::default(),
            orientation: Orientation::Horizontal,
            tick_marks: None,
            text_marks: None,
        }
    }

    /// Sets the [`Orientation`] of the [`PhaseMeter`].
    ///
    /// [`Orientation`]: enum.Orientation.html
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        if self.orientation != orientation {
            self.orientation = orientation;
            let temp_height = self.height;
            self.height = self.width;
            self.width = temp_height;
        }
        self
    }

    /// Sets the width of the [`PhaseMeter`].
    ///
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`PhaseMeter`].
    ///
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`PhaseMeter`].
    ///
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/phase_meter/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the [`TextMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `text_mark_style(&self) -> Option<TextMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`StyleSheet`]: ../../style/phase_meter/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a TextMarkGroup) -> Self {
        self.text_marks = Some(text_marks);
        self
    }
}

/// The [`Normal`] positions of each color tier of a [`PhaseMeter`]
///
/// [`Normal`]: ../../core/struct.Normal.html
/// [`PhaseMeter`]: struct.PhaseMeter.html
#[derive(Debug, Copy, Clone)]
pub struct TierPositions {
    /// The [`Normal`] position (of the left half) where the `poor` color starts.
    /// Default is `0.55.into()`
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub poor: Normal,

    /// The [`Normal`] position (of the right half) where the `good` color starts.
    /// Default is `0.45.into()`
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub good: Normal,
}

impl Default for TierPositions {
    fn default() -> Self {
        Self {
            poor: 0.55.into(),
            good: 0.45.into(),
        }
    }
}

/// The local state of a [`PhaseMeter`].
///
/// [`PhaseMeter`]: struct.PhaseMeter.html
#[derive(Debug, Copy, Clone)]
pub struct State {
    /// The current [`Normal`] position of the phase meter.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub normal: Normal,
    tier_positions: TierPositions,
}

impl State {
    /// Creates a new [`PhaseMeter`] state.
    ///
    /// * `normal` - The current [`Normal`] position of the phase meter.
    /// * `tier_positions` - The positions where each tier of color starts.
    ///
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    pub fn new(normal: Normal, tier_positions: TierPositions) -> Self {
        Self {
            normal,
            tier_positions,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            normal: 0.5.into(),
            tier_positions: TierPositions::default(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for PhaseMeter<'a, Renderer>
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
            self.state.normal,
            self.state.tier_positions,
            &self.orientation,
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

/// The renderer of a [`PhaseMeter`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`PhaseMeter`] in your user interface.
///
/// [`PhaseMeter`]: struct.PhaseMeter.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`PhaseMeter`].
    ///
    /// It receives:
    ///   * the bounds of the [`PhaseMeter`]
    ///   * the current [`Normal`] position of the phase meter.
    ///   * the positions of each tier of color
    ///   * the orientation of the [`PhaseMeter`]
    ///   * any tick marks to display
    ///   * any text marks to display
    ///   * the style of the [`PhaseMeter`]
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    /// [`PhaseMeter`]: struct.PhaseMeter.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        normal: Normal,
        tier_positions: TierPositions,
        orientation: &Orientation,
        tick_marks: Option<&TickMarkGroup>,
        text_marks: Option<&TextMarkGroup>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<PhaseMeter<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        phase_meter: PhaseMeter<'a, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(phase_meter)
    }
}
