//! An animated solar system.
//!
//! This example showcases how to use a `Canvas` widget with transforms to draw
//! using different coordinate systems.
//!
//! Inspired by the example found in the MDN docs[1].
//!
//! [1]: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Basic_animations#An_animated_solar_system
use iced::{
    executor, Application, Column, Command, Container, Element, Length, Row,
    Settings, Subscription, Color, Text
};

use iced_audio::{db_meter, knob, DBMeter, Knob, LogDBParam, Normal,
TickMarkGroup, TickMarkTier, TickMark};

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
    ParamChanged((ParamID, Normal)),
}

struct DBMeterApp {
    left_main_db_parm: LogDBParam<ParamID>,
    left_peak_db_param: LogDBParam<ParamID>,
    right_main_db_parm: LogDBParam<ParamID>,
    right_peak_db_param: LogDBParam<ParamID>,

    left_main_db_knob_state: knob::State,
    left_peak_db_knob_state: knob::State,
    right_main_db_knob_state: knob::State,
    right_peak_db_knob_state: knob::State,

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
        let left_main_db_parm = LogDBParam::<ParamID>::new(
            ParamID::LeftMainDB,
            -64.0,
            3.0,
            -64.0,
            0.0,
            0.9.into(),
        );
        let left_peak_db_param = LogDBParam::<ParamID>::new(
            ParamID::LeftPeakDB,
            -64.0,
            3.0,
            -64.0,
            0.0,
            0.9.into(),
        );

        let right_main_db_parm = LogDBParam::<ParamID>::new(
            ParamID::RightMainDB,
            -64.0,
            3.0,
            -64.0,
            0.0,
            0.9.into(),
        );
        let right_peak_db_param = LogDBParam::<ParamID>::new(
            ParamID::RightPeakDB,
            -64.0,
            3.0,
            -64.0,
            0.0,
            0.9.into(),
        );

        (
            DBMeterApp {
                left_main_db_parm,
                left_peak_db_param,
                right_main_db_parm,
                right_peak_db_param,

                left_main_db_knob_state: knob::State::new(&left_main_db_parm),
                left_peak_db_knob_state: knob::State::new(&left_peak_db_param),
                right_main_db_knob_state: knob::State::new(&right_main_db_parm),
                right_peak_db_knob_state: knob::State::new(
                    &right_peak_db_param,
                ),

                db_meter_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    Some(db_meter::BarState::default()),
                    db_meter::TierPositions {
                        clipping: left_main_db_parm.value_to_normal(0.0),
                        med: Some(left_main_db_parm.value_to_normal(-12.0)),
                        high: Some(left_main_db_parm.value_to_normal(-3.0)),
                    },
                ),

                db_meter_custom_state: db_meter::State::new(
                    db_meter::BarState::default(),
                    None,
                    db_meter::TierPositions {
                        clipping: left_main_db_parm.value_to_normal(0.0),
                        med: None,
                        high: Some(left_main_db_parm.value_to_normal(-6.0)),
                    },
                ),

                tick_marks: TickMarkGroup::from_vec(
                    vec![
                        TickMark {
                            position: left_main_db_parm.value_to_normal(0.0),
                            tier: TickMarkTier::One,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-3.0),
                            tier: TickMarkTier::Two,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-6.0),
                            tier: TickMarkTier::Two,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-9.0),
                            tier: TickMarkTier::Two,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-12.0),
                            tier: TickMarkTier::Two,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-24.0),
                            tier: TickMarkTier::Two,
                        },
                        TickMark {
                            position: left_main_db_parm.value_to_normal(-48.0),
                            tier: TickMarkTier::Two,
                        },
                    ]
                ),

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
            }
            Message::ParamChanged((id, normal)) => {
                // Update each parameter with the `Normal` output value from
                // the corresponding parameter widget.
                //
                // Now do something useful with that value!
                //
                match id {
                    ParamID::LeftMainDB => {
                        self.left_main_db_parm.set_from_normal(normal);
                        self.db_meter_state.set_left(normal);
                        self.db_meter_custom_state.set_left(normal);
                    }
                    ParamID::LeftPeakDB => {
                        self.left_peak_db_param.set_from_normal(normal);
                        self.db_meter_state.set_left_peak(normal);
                        self.db_meter_custom_state.set_left_peak(normal);
                    }
                    ParamID::RightMainDB => {
                        self.right_main_db_parm.set_from_normal(normal);
                        self.db_meter_state.set_right(normal);
                    }
                    ParamID::RightPeakDB => {
                        self.right_peak_db_param.set_from_normal(normal);
                        self.db_meter_state.set_right_peak(normal);
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(10))
            .map(|instant| Message::Tick(instant))
    }

    fn view(&mut self) -> Element<Message> {
        let left_main_knob = Knob::new(
            &mut self.left_main_db_knob_state,
            &self.left_main_db_parm,
            Message::ParamChanged,
        );
        let left_peak_knob = Knob::new(
            &mut self.left_peak_db_knob_state,
            &self.left_peak_db_param,
            Message::ParamChanged,
        );

        let right_main_knob = Knob::new(
            &mut self.right_main_db_knob_state,
            &self.right_main_db_parm,
            Message::ParamChanged,
        );
        let right_peak_knob = Knob::new(
            &mut self.right_peak_db_knob_state,
            &self.right_peak_db_param,
            Message::ParamChanged,
        );

        let db_meter = DBMeter::new(
            &mut self.db_meter_state,
            db_meter::Orientation::Vertical,
        )
        .tick_marks(&self.tick_marks);

        let db_meter_custom = DBMeter::new(
            &mut self.db_meter_custom_state,
            db_meter::Orientation::Horizontal,
        )
        .height(Length::from(Length::Units(24)))
        .tick_marks(&self.tick_marks)
        .style(CustomDBMeterStyle);

        let row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            
            .push(Column::new()
                .max_width(60)
                .height(Length::Fill)
                .spacing(20)
                .push(left_main_knob)
                .push(Text::new("Left DB"))
            )

            .push(Column::new()
                .max_width(60)
                .height(Length::Fill)
                .spacing(20)
                .push(left_peak_knob)
                .push(Text::new("Left Peak DB"))
            )

            .push(Column::new()
                .max_width(60)
                .height(Length::Fill)
                .spacing(20)
                .push(right_main_knob)
                .push(Text::new("Right DB"))
            )

            .push(Column::new()
                .max_width(60)
                .height(Length::Fill)
                .spacing(20)
                .push(right_peak_knob)
                .push(Text::new("Right Peak DB"))
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

mod time {
    use iced::futures;
    use std::time::Instant;

    pub fn every(duration: std::time::Duration) -> iced::Subscription<Instant> {
        iced::Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H, I> for Every
    where
        H: std::hash::Hasher,
    {
        type Output = Instant;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: futures::stream::BoxStream<'static, I>,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| Instant::now())
                .boxed()
        }
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

    fn tick_mark_style(&self) -> Option<db_meter::TickMarkStyle> {
        Some(db_meter::TickMarkStyle {
            size_tier_1: 4,
            size_tier_2: 3,
            size_tier_3: 2,

            height_tier_1: 2,
            height_tier_2: 2,
            height_tier_3: 1,

            color_tier_1: BACK_COLOR,
            color_tier_2: BACK_COLOR,
            color_tier_3: BACK_COLOR,

            offset: 2,

            position: db_meter::TickMarkPosition::Right,
        })
    }
}