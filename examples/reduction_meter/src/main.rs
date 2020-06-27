use iced::{
    executor, Application, Color, Column, Command, Container, Element, Length,
    Row, Settings, Subscription, Text, time,
};

use iced_audio::{
    bar_tick_marks, knob, reduction_meter, DBRange, Knob, ReductionMeter,
    TickMark, TickMarkGroup, TickMarkTier,
};

use std::time::Instant;

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    MainDB,
    PeakDB,
}

pub fn main() {
    ReductionMeterApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    ParamMoved(ParamID),
}

struct ReductionMeterApp {
    #[allow(dead_code)]
    db_range: DBRange,

    main_knob_state: knob::State<ParamID>,
    peak_knob_state: knob::State<ParamID>,

    r_meter_default_state: reduction_meter::State,
    r_meter_custom_state: reduction_meter::State,

    tick_marks: TickMarkGroup,

    current: Instant,
}

impl ReductionMeterApp {
    pub fn update(&mut self, now: Instant) {
        self.current = now;
    }
}

impl Application for ReductionMeterApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let db_range = DBRange::new(-64.0, 0.0, 1.0.into());

        (
            ReductionMeterApp {
                db_range,

                main_knob_state: knob::State::new(db_range.create_param(
                    ParamID::MainDB,
                    -64.0,
                    -64.0,
                )),
                peak_knob_state: knob::State::new(db_range.create_param(
                    ParamID::PeakDB,
                    -64.0,
                    -64.0,
                )),

                r_meter_default_state: reduction_meter::State::new(
                    0.0.into(),
                    Some(0.0.into()),
                ),

                r_meter_custom_state: reduction_meter::State::new(
                    0.0.into(),
                    Some(0.0.into()),
                ),

                tick_marks: TickMarkGroup::new(vec![
                    TickMark {
                        position: db_range.to_normal(-1.0),
                        tier: TickMarkTier::One,
                    },
                    TickMark {
                        position: db_range.to_normal(-2.0),
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
        String::from("Reduction Meter - Iced Audio")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.update(instant);

                // Normally you would animate the meter here, but basic
                // knobs are used instead for demonstration.
            }
            Message::ParamMoved(id) => match id {
                ParamID::MainDB => {
                    let normal = self.main_knob_state.param.normal;
                    self.r_meter_default_state.bar_normal = normal;
                    self.r_meter_custom_state.bar_normal = normal;
                }
                ParamID::PeakDB => {
                    let normal = self.peak_knob_state.param.normal;
                    self.r_meter_default_state.peak_normal = Some(normal);
                    self.r_meter_custom_state.peak_normal = Some(normal);
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
        let main_knob =
            Knob::new(&mut self.main_knob_state, Message::ParamMoved);
        let peak_knob =
            Knob::new(&mut self.peak_knob_state, Message::ParamMoved);

        let r_meter_default =
            ReductionMeter::new(&mut self.r_meter_default_state)
                .tick_marks(&self.tick_marks);

        let r_meter_custom =
            ReductionMeter::new(&mut self.r_meter_custom_state)
                .orientation(reduction_meter::Orientation::Horizontal)
                .style(CustomReductionMeterStyle)
                .tick_marks(&self.tick_marks);

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
                    .push(main_knob)
                    .push(Text::new("DB")),
            )
            .push(
                Column::new()
                    .max_width(60)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(peak_knob)
                    .push(Text::new("Peak DB")),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .max_width(100)
                    .spacing(20)
                    .push(r_meter_default),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .spacing(20)
                    .push(r_meter_custom),
            );

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
pub const COLOR: Color = Color::from_rgb(
    0x99 as f32 / 255.0,
    0x91 as f32 / 255.0,
    0xFF as f32 / 255.0,
);

struct CustomReductionMeterStyle;
impl reduction_meter::StyleSheet for CustomReductionMeterStyle {
    fn style(&self) -> reduction_meter::Style {
        reduction_meter::Style {
            back_color: BACK_COLOR,
            back_border_width: 2,
            back_border_radius: 2,
            back_border_color: BORDER_COLOR,
            color: COLOR,
            peak_line_color: COLOR,
            peak_line_width: 2,
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
