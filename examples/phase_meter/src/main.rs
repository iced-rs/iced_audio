use iced::{
    executor, Application, Color, Column, Command, Container, Element, Length,
    Row, Settings, Subscription, Text,
};

use iced_audio::{
    knob, phase_meter, FloatRange, Knob, PhaseMeter, TextMarkGroup,
    TickMarkGroup, TickMarkTier,
};

use std::time::Instant;

// Create a unique identifier for each parameter. Note you may also use any
// type you want such as u32, i32, Strings, etc.
#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    PhaseKnob,
}

pub fn main() {
    PhaseMeterApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    ParamMoved(ParamID),
}

struct PhaseMeterApp {
    #[allow(dead_code)]
    float_range: FloatRange,

    phase_knob_state: knob::State<ParamID>,

    phase_meter_default_state: phase_meter::State,
    phase_meter_custom_state: phase_meter::State,

    tick_marks: TickMarkGroup,
    text_marks: TextMarkGroup,

    current: Instant,
}

impl PhaseMeterApp {
    pub fn update(&mut self, now: Instant) {
        self.current = now;
    }
}

impl Application for PhaseMeterApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let float_range = FloatRange::default_bipolar();

        (
            PhaseMeterApp {
                float_range,

                phase_knob_state: knob::State::new(
                    float_range.create_param_default(ParamID::PhaseKnob),
                ),

                phase_meter_default_state: phase_meter::State::default(),

                phase_meter_custom_state: phase_meter::State::new(
                    0.5.into(),
                    phase_meter::TierPositions {
                        poor: 0.55.into(),
                        good: 0.45.into(),
                    },
                ),

                tick_marks: TickMarkGroup::subdivided(
                    1,
                    1,
                    0,
                    Some(TickMarkTier::Two),
                ),

                text_marks: TextMarkGroup::min_max_and_center("-1", "+1", "0"),

                current: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Phase Correlation Meter - Iced Audio")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.update(instant);

                // Normally you would animate the meter here, but basic
                // knobs are used instead for demonstration.
            }
            Message::ParamMoved(id) => match id {
                ParamID::PhaseKnob => {
                    let normal = self.phase_knob_state.param.normal;
                    self.phase_meter_default_state.normal = normal;
                    self.phase_meter_custom_state.normal = normal;
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
        let phase_knob =
            Knob::new(&mut self.phase_knob_state, Message::ParamMoved);

        let phase_meter_default =
            PhaseMeter::new(&mut self.phase_meter_default_state)
                .tick_marks(&self.tick_marks)
                .text_marks(&self.text_marks);

        let phase_meter_custom =
            PhaseMeter::new(&mut self.phase_meter_custom_state)
                .orientation(phase_meter::Orientation::Vertical)
                .style(CustomPhaseMeterStyle)
                .width(Length::from(Length::Units(14)));

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
                    .push(phase_knob)
                    .push(Text::new("Phase")),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(20)
                    .push(phase_meter_default),
            )
            .push(
                Column::new()
                    .max_width(100)
                    .max_height(400)
                    .spacing(20)
                    .push(phase_meter_custom),
            );

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
pub const BAD_COLOR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x94 as f32 / 255.0,
    0x91 as f32 / 255.0,
);
pub const POOR_COLOR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x94 as f32 / 255.0,
    0x91 as f32 / 255.0,
);
pub const OKAY_COLOR: Color = Color::from_rgb(
    0x99 as f32 / 255.0,
    0x91 as f32 / 255.0,
    0xFF as f32 / 255.0,
);
pub const GOOD_COLOR: Color = Color::from_rgb(
    0x91 as f32 / 255.0,
    0xBD as f32 / 255.0,
    0xFF as f32 / 255.0,
);

// Custom style for a `PhaseMeter`

struct CustomPhaseMeterStyle;
impl phase_meter::StyleSheet for CustomPhaseMeterStyle {
    fn style(&self) -> phase_meter::Style {
        phase_meter::Style {
            back_color: BACK_COLOR,
            back_border_width: 2,
            back_border_color: BORDER_COLOR,
            bad_color: BAD_COLOR,
            poor_color: POOR_COLOR,
            okay_color: OKAY_COLOR,
            good_color: GOOD_COLOR,
            center_line_width: 2,
            center_line_color: Color::WHITE,
        }
    }
}
