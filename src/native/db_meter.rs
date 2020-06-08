//!

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{Normal, TickMarkGroup};

static DEFAULT_WIDTH: u16 = 20;

///
#[allow(missing_debug_implementations)]
pub enum Orientation {
    ///
    Vertical,
    ///
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

///
///
/// A [`DBMeter`] will try to fill the vertical space of its container.
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
    ///
    pub fn new(state: &'a mut State, orientation: Orientation) -> Self {
        let (width, height) = match orientation {
            Orientation::Vertical => {
                (Length::from(Length::Units(DEFAULT_WIDTH)), Length::Fill)
            }
            Orientation::Horizontal => {
                (Length::Fill, Length::from(Length::Units(DEFAULT_WIDTH)))
            }
        };

        DBMeter {
            state,
            width,
            height,
            style: Renderer::Style::default(),
            orientation,
            tick_marks: None,
        }
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
    /// [`StyleSheet`]: ../../style/v_db_meter/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a TickMarkGroup) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }
}

///
///
/// [`DBMeter`]: struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct TierPositions {
    ///
    pub clipping: Normal,
    ///
    pub med: Option<Normal>,
    ///
    pub high: Option<Normal>,
}

///
///
/// [`DBMeter`]: struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct BarState {
    ///
    pub normal: Normal,
    ///
    pub peak_normal: Option<Normal>,
}

impl BarState {
    ///
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

    ///
    pub fn set_left(&mut self, normal: Normal) {
        self.left_bar.normal = normal;
    }
    ///
    pub fn set_left_peak(&mut self, normal: Normal) {
        self.left_bar.peak_normal = Some(normal);
    }
    ///
    pub fn set_right(&mut self, normal: Normal) {
        if let Some(right_bar) = &mut self.right_bar {
            right_bar.normal = normal;
        }
    }
    ///
    pub fn set_right_peak(&mut self, normal: Normal) {
        if let Some(right_bar) = &mut self.right_bar {
            right_bar.peak_normal = Some(normal);
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

/// The renderer of an [`DBMeter`].
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
    ///   * the current cursor position
    ///   * the local state of the [`DBMeter`]
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
