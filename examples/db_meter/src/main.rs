use iced::{
    executor, time, Application, Color, Column, Command, Container, Element,
    Length, Row, Settings, Subscription, Text,
};

use iced_audio::{
    bar_text_marks, bar_tick_marks, db_meter, knob, DBMeter, FloatRange, Knob,
    TextMark, TextMarkGroup, TickMark, TickMarkGroup, TickMarkTier,
};

use std::time::Instant;

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    LeftMainDB,
    LeftPeakDB,
    RightMainDB,
    RightPeakDB,
}

pub fn main() {
    DBMeterApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    ParamMoved(ParamID),
}

struct DBMeterApp {
    #[allow(dead_code)]
    // Not to be confused with `LogDBRange` for use with controls.
    // Decibel meters usually display decibels linearly.
    linear_db_range: FloatRange,

    left_main_db_knob_state: knob::State<ParamID>,
    left_peak_db_knob_state: knob::State<ParamID>,
    right_main_db_knob_state: knob::State<ParamID>,
    right_peak_db_knob_state: knob::State<ParamID>,

    db_meter_state: db_meter::State,
    db_meter_custom_state: db_meter::State,

    tick_marks: TickMarkGroup,
    text_marks: TextMarkGroup,

    current: Instant,
}

impl DBMeterApp {
    pub fn update(&mut self, now: Instant) {
        self.current = now;
    }
}

impl Application for DBMeterApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Not to be confused with `LogDBRange` for use with controls.
        // Decibel meters usually display decibels linearly.
        let linear_db_range = FloatRange::new(-50.0, 3.0);

        (
            DBMeterApp {
                linear_db_range,

                left_main_db_knob_state: knob::State::new(
                    linear_db_range.create_param(
                        ParamID::LeftMainDB,
                        -50.0,
                        -50.0,
                    ),
                ),
                left_peak_db_knob_state: knob::State::new(
                    linear_db_range.create_param(
                        ParamID::LeftPeakDB,
                        -50.0,
                        -50.0,
                    ),
                ),
                right_main_db_knob_state: knob::State::new(
                    linear_db_range.create_param(
                        ParamID::RightMainDB,
                        -50.0,
                        -50.0,
                    ),
                ),
                right_peak_db_knob_state: knob::State::new(
                    linear_db_range.create_param(
                        ParamID::RightPeakDB,
                        -50.0,
                        -50.0,
                    ),
                ),

                db_meter_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    Some(db_meter::BarState::default()),
                    db_meter::TierPositions {
                        clipping: linear_db_range.to_normal(0.0),
                        med: Some(linear_db_range.to_normal(-18.0)),
                        high: Some(linear_db_range.to_normal(-6.0)),
                    },
                ),

                db_meter_custom_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    None,
                    db_meter::TierPositions {
                        clipping: linear_db_range.to_normal(0.0),
                        med: None,
                        high: Some(linear_db_range.to_normal(-12.0)),
                    },
                ),

                tick_marks: TickMarkGroup::new(vec![
                    TickMark::new(
                        linear_db_range.to_normal(0.0),
                        TickMarkTier::One,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-3.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-6.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-9.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-12.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-15.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-18.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-21.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-24.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-30.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-36.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-42.0),
                        TickMarkTier::Two,
                    ),
                    TickMark::new(
                        linear_db_range.to_normal(-48.0),
                        TickMarkTier::Two,
                    ),
                ]),

                text_marks: TextMarkGroup::new(vec![
                    TextMark::new("0", linear_db_range.to_normal(0.0)),
                    TextMark::new("-6", linear_db_range.to_normal(-6.0)),
                    TextMark::new("-12", linear_db_range.to_normal(-12.0)),
                    TextMark::new("-18", linear_db_range.to_normal(-18.0)),
                    TextMark::new("-24", linear_db_range.to_normal(-24.0)),
                    TextMark::new("-30", linear_db_range.to_normal(-30.0)),
                    TextMark::new("-36", linear_db_range.to_normal(-36.0)),
                    TextMark::new("-42", linear_db_range.to_normal(-42.0)),
                    TextMark::new("-48", linear_db_range.to_normal(-48.0)),
                ]),

                current: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DB Meter - Iced Audio")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.update(instant);

                // Normally you would animate the DBMeter here, but basic
                // knobs are used instead for demonstration.
            }
            Message::ParamMoved(id) => match id {
                ParamID::LeftMainDB => {
                    let normal = self.left_main_db_knob_state.param.normal;
                    self.db_meter_state.set_left(normal);
                    self.db_meter_custom_state.set_left(normal);
                }
                ParamID::LeftPeakDB => {
                    let normal = self.left_peak_db_knob_state.param.normal;
                    self.db_meter_state.set_left_peak(Some(normal));
                    self.db_meter_custom_state.set_left_peak(Some(normal));
                }
                ParamID::RightMainDB => {
                    self.db_meter_state
                        .set_right(self.right_main_db_knob_state.param.normal);
                }
                ParamID::RightPeakDB => {
                    self.db_meter_state.set_right_peak(
                        self.right_peak_db_knob_state.param.normal,
                    );
                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(10))
            .map(|instant| Message::Tick(instant))
    }

    fn view(&mut self) -> Element<Message> {
        let left_main_knob =
            Knob::new(&mut self.left_main_db_knob_state, Message::ParamMoved);
        let left_peak_knob =
            Knob::new(&mut self.left_peak_db_knob_state, Message::ParamMoved);

        let right_main_knob =
            Knob::new(&mut self.right_main_db_knob_state, Message::ParamMoved);
        let right_peak_knob =
            Knob::new(&mut self.right_peak_db_knob_state, Message::ParamMoved);

        let db_meter = DBMeter::new(&mut self.db_meter_state)
            .tick_marks(&self.tick_marks)
            .text_marks(&self.text_marks);

        let db_meter_custom = DBMeter::new(&mut self.db_meter_custom_state)
            .orientation(db_meter::Orientation::Horizontal)
            .height(Length::from(Length::Units(24)))
            .tick_marks(&self.tick_marks)
            .text_marks(&self.text_marks)
            .style(CustomDBMeterStyle);

        let row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            .push(
                Column::new()
                    .max_width(60)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(left_main_knob)
                    .push(Text::new("Left DB")),
            )
            .push(
                Column::new()
                    .max_width(60)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(left_peak_knob)
                    .push(Text::new("Left Peak DB")),
            )
            .push(
                Column::new()
                    .max_width(60)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(right_main_knob)
                    .push(Text::new("Right DB")),
            )
            .push(
                Column::new()
                    .max_width(60)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(right_peak_knob)
                    .push(Text::new("Right Peak DB")),
            )
            .push(db_meter)
            .push(db_meter_custom);

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub const BACK_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
pub const BORDER_COLOR: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0x33 as f32 / 255.0,
    0x3C as f32 / 255.0,
);
pub const LOW_COLOR: Color = Color::from_rgb(
    0x99 as f32 / 255.0,
    0x91 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const HIGH_COLOR: Color = Color::from_rgb(
    0x91 as f32 / 255.0,
    0xBD as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const CLIP_COLOR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x94 as f32 / 255.0,
    0x91 as f32 / 255.0,
);

struct CustomDBMeterStyle;
impl db_meter::StyleSheet for CustomDBMeterStyle {
    fn style(&self) -> db_meter::Style {
        db_meter::Style {
            back_color: BACK_COLOR,
            back_border_width: 2,
            back_border_color: BORDER_COLOR,
            low_color: LOW_COLOR,
            med_color: Color::TRANSPARENT,
            high_color: HIGH_COLOR,
            clip_color: CLIP_COLOR,
            peak_line_color: Some([1.0, 1.0, 1.0, 0.8].into()),
            peak_line_width: 2,
            color_all_clip_color: true,
            clip_marker_width: 2,
            clip_marker_color: Color::from_rgba8(0x96, 0x9E, 0xAA, 0.4),
            inner_gap: 2,
            inner_gap_color: Color::TRANSPARENT,
        }
    }

    fn tick_mark_style(&self) -> Option<bar_tick_marks::Style> {
        Some(bar_tick_marks::Style {
            length_tier_1: 4,
            length_tier_2: 3,
            length_tier_3: 2,

            width_tier_1: 2,
            width_tier_2: 2,
            width_tier_3: 1,

            color_tier_1: BACK_COLOR,
            color_tier_2: BACK_COLOR,
            color_tier_3: BACK_COLOR,

            offset: 2,

            placement: bar_tick_marks::Placement::RightOrBottom,
        })
    }

    fn text_mark_style(&self) -> Option<bar_text_marks::Style> {
        Some(bar_text_marks::Style {
            color: BORDER_COLOR,
            offset: 8,
            text_size: 12,
            font: Default::default(),
            bounds_width: 30,
            bounds_height: 14,
            placement: bar_text_marks::Placement::RightOrBottom,
        })
    }
}
