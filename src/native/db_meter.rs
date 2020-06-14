//! Display a visualizer that displays average/peak decibel levels. It can be
//! either mono or stereo.

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, TickMarkGroup};

static DEFAULT_WIDTH: u16 = 20;

/// The orientation of a [`DBMeter`]
///
/// [`DBMeter`]: struct.DBMeter.html
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

/// A visualizer that displays average/peak decibel levels. It can be
/// either mono or stereo.
///
/// A [`DBMeter`] will try to fill the length of its container.
///
/// [`DBMeter`]: struct.DBMeter.html
#[allow(missing_debug_implementations)]
pub struct DBMeter<'a, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    style: Renderer::Style,
    orientation: Orientation,
    tick_marks: Option<&'a TickMarkGroup>,
}

impl<'a, Renderer: self::Renderer> DBMeter<'a, Renderer> {
    /// Creates a new [`DBMeter`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`DBMeter`]
    ///
    /// [`State`]: struct.State.html
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn new(state: &'a mut State) -> Self {
        DBMeter {
            state,
            width: Length::from(Length::Units(DEFAULT_WIDTH)),
            height: Length::Fill,
            style: Renderer::Style::default(),
            orientation: Orientation::Vertical,
            tick_marks: None,
        }
    }

    /// Sets the [`Orientation`] of the [`DBMeter`].
    ///
    /// [`Orientation`]: enum.Orientation.html
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        if self.orientation != orientation {
            self.orientation = orientation;
            let temp_height = self.height;
            self.height = self.width;
            self.width = temp_height;
        }
        self
    }

    /// Sets the width of the [`DBMeter`].
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`DBMeter`].
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`DBMeter`].
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the [`TickMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `tick_mark_style(&self) -> Option<TickMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`StyleSheet`]: ../../style/db_meter/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }
}

/// The [`Normal`] positions of each color tier of a [`DBMeter`]
///
/// [`Normal`]: ../../core/struct.Normal.html
/// [`DBMeter`]: struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct TierPositions {
    /// The [`Normal`] position where the `clipping` color starts.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub clipping: Normal,

    /// The [`Normal`] position where the `high` color starts. Set this
    /// to `None` for no `high` color tier.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub high: Option<Normal>,

    /// The [`Normal`] position where the `medium` color starts. Set this
    /// to `None` for no `medium` color tier. This will not be active
    /// if `high` is set to `None`.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub med: Option<Normal>,
}

/// The state of a single meter bar in a [`DBMeter`] [`State`].
///
/// [`DBMeter`]: struct.DBMeter.html
/// [`State`]: struct.State.html
#[derive(Debug, Copy, Clone)]
pub struct BarState {
    /// The [`Normal`] position of the bar.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub normal: Normal,

    /// The [`Normal`] position of the peak line. Set this to `None`
    /// for no peak line.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub peak_normal: Option<Normal>,
}

impl BarState {
    /// Creates a new [`BarState`] for a [`DBMeter`] [`State`]
    ///
    /// * `normal` - The [`Normal`] position of the bar.
    /// * `peak_normal` - The [`Normal`] position of the peak line.
    /// Set this to `None` for no peak line.
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    /// [`BarState`]: struct.BarState.html
    /// [`State`]: struct.State.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn new(normal: Normal, peak_normal: Option<Normal>) -> Self {
        Self {
            normal,
            peak_normal,
        }
    }
}

impl Default for BarState {
    fn default() -> Self {
        Self {
            normal: 0.0.into(),
            peak_normal: None,
        }
    }
}

/// The local state of a [`DBMeter`].
///
/// [`DBMeter`]: struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct State {
    left_bar: BarState,
    right_bar: Option<BarState>,
    tier_positions: TierPositions,
}

impl State {
    /// Creates a new [`DBMeter`] state.
    ///
    /// * `left_bar` - The state of the left/top bar.
    /// * `right_bar` - The state of the right/bottom bar. Set this
    /// to `None` for a mono [`DBMeter`].
    /// * `tier_positions` - The positions where each tier of color starts.
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn new(
        left_bar: BarState,
        right_bar: Option<BarState>,
        tier_positions: TierPositions,
    ) -> Self {
        Self {
            left_bar,
            right_bar,
            tier_positions,
        }
    }

    /// Sets the [`Normal`] position of the left bar.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn set_left(&mut self, normal: Normal) {
        self.left_bar.normal = normal;
    }
    /// Sets the [`Normal`] position of the left peak line. Set this to
    /// `None` for no peak line.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn set_left_peak(&mut self, normal: Option<Normal>) {
        self.left_bar.peak_normal = normal;
    }
    /// Sets the [`Normal`] position of the right bar. This will
    /// have no effect if `right` was set to `None` in `State::new()`.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn set_right(&mut self, normal: Normal) {
        if let Some(right_bar) = &mut self.right_bar {
            right_bar.normal = normal;
        }
    }
    /// Sets the [`Normal`] position of the right peak line. Set this to
    /// `None` for no peak line. This will have no effect if `right`
    /// was set to `None` in `State::new()`.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn set_right_peak(&mut self, normal: Normal) {
        if let Some(right_bar) = &mut self.right_bar {
            right_bar.peak_normal = Some(normal);
        }
    }

    /// Returns the [`Normal`] position of the left bar.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn left_normal(&self) -> Normal {
        self.left_bar.normal
    }

    /// Returns the [`Normal`] position of the left peak line.
    /// Returns `None` if none exists.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn left_peak_normal(&self) -> Option<Normal> {
        self.left_bar.peak_normal
    }

    /// Returns the [`Normal`] position of the right bar.
    /// Returns `None` if the meter is mono.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn right_normal(&self) -> Option<Normal> {
        if let Some(right_bar) = self.right_bar {
            Some(right_bar.normal)
        } else {
            None
        }
    }

    /// Returns the [`Normal`] position of the right peak line.
    /// Returns `None` if none exists or if the meter is mono.
    ///
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn right_peak_normal(&self) -> Option<Normal> {
        if let Some(right_bar) = self.right_bar {
            right_bar.peak_normal
        } else {
            None
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DBMeter<'a, Renderer>
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
            self.state.left_bar,
            self.state.right_bar,
            self.state.tier_positions,
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

/// The renderer of a [`DBMeter`].
///
/// Your renderer will need to implement this trait before being
/// able to use an [`DBMeter`] in your user interface.
///
/// [`DBMeter`]: struct.DBMeter.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`DBMeter`].
    ///
    /// It receives:
    ///   * the bounds of the [`DBMeter`]
    ///   * the state of the left bar
    ///   * the state of the right bar
    ///   * the positions of each tier of color
    ///   * the orientation of the [`DBMeter`]
    ///   * any tick marks to display
    ///   * the style of the [`DBMeter`]
    ///
    /// [`DBMeter`]: struct.DBMeter.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        left_bar: BarState,
        right_bar: Option<BarState>,
        tier_positions: TierPositions,
        orientation: &Orientation,
        tick_marks: Option<&TickMarkGroup>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<DBMeter<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(db_meter: DBMeter<'a, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(db_meter)
    }
}
