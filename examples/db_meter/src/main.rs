use iced::{
    executor, Application, Color, Column, Command, Container, Element, Length,
    Row, Settings, Subscription, Text, time,
};

use iced_audio::{
    bar_tick_marks, db_meter, knob, DBMeter, DBRange, Knob, TickMark,
    TickMarkGroup, TickMarkTier,
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
    db_range: DBRange,

    left_main_db_knob_state: knob::State<ParamID>,
    left_peak_db_knob_state: knob::State<ParamID>,
    right_main_db_knob_state: knob::State<ParamID>,
    right_peak_db_knob_state: knob::State<ParamID>,

    db_meter_state: db_meter::State,
    db_meter_custom_state: db_meter::State,

    tick_marks: TickMarkGroup,

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
        let db_range = DBRange::new(-64.0, 3.0, 0.9.into());

        (
            DBMeterApp {
                db_range,

                left_main_db_knob_state: knob::State::new(
                    db_range.create_param(ParamID::LeftMainDB, -64.0, -64.0),
                ),
                left_peak_db_knob_state: knob::State::new(
                    db_range.create_param(ParamID::LeftPeakDB, -64.0, -64.0),
                ),
                right_main_db_knob_state: knob::State::new(
                    db_range.create_param(ParamID::RightMainDB, -64.0, -64.0),
                ),
                right_peak_db_knob_state: knob::State::new(
                    db_range.create_param(ParamID::RightPeakDB, -64.0, -64.0),
                ),

                db_meter_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    Some(db_meter::BarState::default()),
                    db_meter::TierPositions {
                        clipping: db_range.to_normal(0.0),
                        med: Some(db_range.to_normal(-12.0)),
                        high: Some(db_range.to_normal(-3.0)),
                    },
                ),

                db_meter_custom_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    None,
                    db_meter::TierPositions {
                        clipping: db_range.to_normal(0.0),
                        med: None,
                        high: Some(db_range.to_normal(-6.0)),
                    },
                ),

                tick_marks: TickMarkGroup::new(vec![
                    TickMark {
                        position: db_range.to_normal(0.0),
                        tier: TickMarkTier::One,
                    },
                    TickMark {
                        position: db_range.to_normal(-3.0),
                        tier: TickMarkTier::Two,
                    },
                    TickMark {
                        position: db_range.to_normal(-6.0),
                        tier: TickMarkTier::Two,
                    },
                    TickMark {
                        position: db_range.to_normal(-9.0),
                        tier: TickMarkTier::Two,
                    },
                    TickMark {
                        position: db_range.to_normal(-12.0),
                        tier: TickMarkTier::Two,
                    },
                    TickMark {
                        position: db_range.to_normal(-24.0),
                        tier: TickMarkTier::Two,
                    },
                    TickMark {
                        position: db_range.to_normal(-48.0),
                        tier: TickMarkTier::Two,
                    },
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

        let db_meter =
            DBMeter::new(&mut self.db_meter_state).tick_marks(&self.tick_marks);

        let db_meter_custom = DBMeter::new(&mut self.db_meter_custom_state)
            .orientation(db_meter::Orientation::Horizontal)
            .height(Length::from(Length::Units(24)))
            .tick_marks(&self.tick_marks)
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
}
