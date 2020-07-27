//! Display a visualizer that displays average/peak decibel levels. It can be
//! either mono or stereo.

mod peak;
mod peak_rms;

pub use peak::*;
pub use peak_rms::*;

use std::fmt::Debug;

use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};

use std::hash::Hash;

use crate::core::{FloatRange, Normal, TextMarkGroup, TickMarkGroup};

static DEFAULT_WIDTH: u16 = 20;

static DEFAULT_PEAK_FALL_RATE: f32 = 0.7;
static DEFAULT_BAR_FALL_RATE: f32 = 0.475;
static DEFAULT_PEAK_HOLD_SEC: f32 = 1.75;

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
    text_marks: Option<&'a TextMarkGroup>,
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
            text_marks: None,
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

    /// Sets the [`TextMarkGroup`] to display. Note your [`StyleSheet`] must
    /// also implement `text_mark_style(&self) -> Option<TextMarkStyle>` for
    /// them to display (which the default style does).
    ///
    /// [`TextMarkGroup`]: ../../core/text_marks/struct.TextMarkGroup.html
    /// [`StyleSheet`]: ../../style/db_meter/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a TextMarkGroup) -> Self {
        self.text_marks = Some(text_marks);
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
    ///   * any text marks to display
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
        text_marks: Option<&TextMarkGroup>,
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

/// The output of a [`Detector`]
///
/// [`Detector`]: trait.Detector.html
#[derive(Debug, Copy, Clone)]
pub struct DetectorOutput {
    /// The value of the meter bar in decibels (usually represents rms/average value).
    /// Set this to `None` if there is no update.
    pub bar_db: Option<f32>,
    /// The value of the peak line in decibels (usually represents peak value).
    /// Set this to `None` if there is no update.
    pub peak_db: Option<f32>,
    /// The number of samples to discard from the ring buffer
    pub n_samples_to_discard: usize,
}

impl DetectorOutput {
    /// Returns an empty `DetectorOutput` with both values set to `None`
    pub fn empty() -> Self {
        Self {
            bar_db: None,
            peak_db: None,
            n_samples_to_discard: 0,
        }
    }
}

/// A DSP processor used to calculate the peak and rms/average levels of a stereo signal
pub trait Detector {
    /// Called when initialized and when the audio sample rate changes.
    fn update_sample_rate(&mut self, sample_rate: f32);

    /// Process new samples from the left/mono audio channel
    ///
    /// - `s1` and `s2` are slices of a lock-free ring buffer.
    /// They contain only the active readable data.
    /// `s1` is the first slice, and `s2` is the second consecutive slice when the data is wrapped around the ring buffer.
    /// - The length of `s2` may be `0` if all the readable data in the ring buffer is continous (does not wrap around).
    fn process_left(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput;

    /// Process new samples from the right audio channel
    ///
    /// - `s1` and `s2` are slices of a lock-free ring buffer.
    /// They contain only the active readable data.
    /// `s1` is the first slice, and `s2` is the second consecutive slice when the data is wrapped around the ring buffer.
    /// - The length of `s2` may be `0` if all the readable data in the ring buffer is continous (does not wrap around).
    fn process_right(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput;

    /// Clear any buffers / set to 0
    fn clear(&mut self);
}

/// Processes audio to animate a [`DBMeter`]
///
/// [`DBMeter`]: struct.DBMeter.html
#[allow(missing_debug_implementations)]
pub struct Animator {
    /// The rate at which the peak line will smoothly fall (in range [0, 1] per second)
    /// * default = 0.7
    pub peak_fall_rate: f32,
    /// The rate at which the bar will smoothly rise and fall (in range [0, 1] per second)
    /// * default = 0.475
    pub bar_fall_rate: f32,
    /// The time in seconds the peak line will hold before it falls down
    /// * default = 1.75
    pub peak_hold_sec: f32,

    sample_rate: f32,

    detector: Box<dyn Detector>,
    db_range: FloatRange,
    left_rb_rx: ringbuf::Consumer<f32>,
    right_rb_rx: Option<ringbuf::Consumer<f32>>,

    left_peak_normal: Normal,
    right_peak_normal: Normal,
    left_bar_normal: Normal,
    right_bar_normal: Normal,

    left_peak_held_time: f32,
    right_peak_held_time: f32,
}

impl Animator {
    /// Creates a new Animator for a [`DBMeter`]
    ///
    /// ## It expects:
    ///
    /// * `detector` - A [`Detector`] that detects peak and rms/average values
    /// * `db_range` - The same db_range that was used to create the [`State`] of the [`DBMeter`]. This is so the output of `detector` can be mapped correctly.
    /// * `left_rb_rx` - The consumer of the lock-free `RingBuffer` that reads the left channel audio data sent from the audio thread
    /// * `right_rb_rx` - The consumer of the lock-free `RingBuffer` that reads the right channel audio data sent from the audio thread.
    /// Set to `None` for no right audio channel (mono mode).
    ///
    /// [`State`]: struct.State.html
    /// [`DBMeter`]: struct.DBMeter.html
    /// [`Detector`]: trait.Detector.html
    pub fn new(
        detector: Box<dyn Detector>,
        db_range: FloatRange,
        left_rb_rx: ringbuf::Consumer<f32>,
        right_rb_rx: Option<ringbuf::Consumer<f32>>,
        sample_rate: f32,
    ) -> Self {
        let mut detector = detector;
        detector.update_sample_rate(sample_rate);

        Self {
            peak_fall_rate: DEFAULT_PEAK_FALL_RATE,
            bar_fall_rate: DEFAULT_BAR_FALL_RATE,
            peak_hold_sec: DEFAULT_PEAK_HOLD_SEC,
            sample_rate,
            detector,
            db_range,
            left_rb_rx,
            right_rb_rx,
            left_peak_normal: Normal::min(),
            right_peak_normal: Normal::min(),
            left_bar_normal: Normal::min(),
            right_bar_normal: Normal::min(),
            left_peak_held_time: 0.0,
            right_peak_held_time: 0.0,
        }
    }

    /// Sets the audio sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.detector.update_sample_rate(sample_rate);
    }

    /// Sets the [`Detector`] to use
    pub fn set_detector(&mut self, detector: Box<dyn Detector>) {
        self.detector = detector;

        self.detector.update_sample_rate(self.sample_rate);
    }

    /// Clears all values to 0
    pub fn clear(&mut self) {
        self.detector.clear();
        self.left_peak_normal = Normal::min();
        self.right_peak_normal = Normal::min();
        self.left_bar_normal = Normal::min();
        self.right_bar_normal = Normal::min();
        self.left_peak_held_time = 0.0;
        self.right_peak_held_time = 0.0;
    }

    fn peak_hold_and_fall(
        delta_fall: f32,
        normal: Normal,
        new_db: Option<f32>,
        db_range: &FloatRange,
        peak_held_time: &mut f32,
        peak_hold_sec: f32,
    ) -> Normal {
        if let Some(new_db) = new_db {
            let new_normal = db_range.to_normal(new_db);
            if new_normal.value() >= normal.value() {
                *peak_held_time = 0.0;
                new_normal
            } else if *peak_held_time >= peak_hold_sec {
                if normal.value() - delta_fall <= new_normal.value() {
                    new_normal
                } else {
                    (normal.value() - delta_fall).into()
                }
            } else {
                normal
            }
        } else if *peak_held_time >= peak_hold_sec {
            (normal.value() - delta_fall).into()
        } else {
            normal
        }
    }

    fn bar_fall(
        delta_fall: f32,
        normal: Normal,
        new_db: Option<f32>,
        db_range: &FloatRange,
    ) -> Normal {
        if let Some(db) = new_db {
            let new_normal = db_range.to_normal(db);

            if new_normal >= normal {
                new_normal
            } else if normal.value() - delta_fall <= new_normal.value() {
                new_normal
            } else {
                (normal.value() - delta_fall).into()
            }
        } else {
            (normal.value() - delta_fall).into()
        }
    }

    /// Updates to the next frame. This causes the `RingBuffer` to be polled for new inputs,
    /// and then the [`State`] of the [`DBMeter`] gets updated accordingly.
    ///
    /// * `delta_time` - the elapsed time since the last frame (since update() was last called)
    /// * `db_meter` - the [`State`] of the [`DBMeter`] to be animated
    ///
    /// [`State`]: struct.State.html
    /// [`DBMeter`]: struct.DBMeter.html
    pub fn update(&mut self, delta_time: f32, db_meter: &mut State) {
        let delta_peak_fall = self.peak_fall_rate * delta_time;
        let delta_bar_fall = self.bar_fall_rate * delta_time;

        self.left_peak_held_time += delta_time;
        self.right_peak_held_time += delta_time;

        let detector = &mut self.detector;

        let mut left_output = DetectorOutput::empty();
        self.left_rb_rx.access(|s1: &[f32], s2: &[f32]| {
            left_output = detector.process_left(s1, s2);
        });
        let _ = self.left_rb_rx.discard(left_output.n_samples_to_discard);

        self.left_peak_normal = Self::peak_hold_and_fall(
            delta_peak_fall,
            self.left_peak_normal,
            left_output.peak_db,
            &self.db_range,
            &mut self.left_peak_held_time,
            self.peak_hold_sec,
        );

        self.left_bar_normal = Self::bar_fall(
            delta_bar_fall,
            self.left_bar_normal,
            left_output.bar_db,
            &self.db_range,
        );

        db_meter.set_left(self.left_bar_normal);
        db_meter.set_left_peak(Some(self.left_peak_normal));

        if let Some(right_rb_rx) = &mut self.right_rb_rx {
            let mut right_output = DetectorOutput::empty();
            right_rb_rx.access(|s1: &[f32], s2: &[f32]| {
                right_output = detector.process_right(s1, s2);
            });
            let _ = right_rb_rx.discard(right_output.n_samples_to_discard);

            self.right_peak_normal = Self::peak_hold_and_fall(
                delta_peak_fall,
                self.right_peak_normal,
                right_output.peak_db,
                &self.db_range,
                &mut self.right_peak_held_time,
                self.peak_hold_sec,
            );

            self.right_bar_normal = Self::bar_fall(
                delta_bar_fall,
                self.right_bar_normal,
                right_output.bar_db,
                &self.db_range,
            );

            db_meter.set_right(self.right_bar_normal);
            db_meter.set_right_peak(self.right_peak_normal);
        }
    }
}
