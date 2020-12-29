use iced::{Column, Element, Length, Row, Text};

use iced_audio::{
    knob, text_marks, tick_marks, FloatRange, FreqRange, IntRange, Knob,
    LogDBRange, Normal,
};

use crate::{style, Step};

#[derive(Debug, Clone)]
pub enum Message {
    Float(Normal),
    Int(Normal),
    DB(Normal),
    Freq(Normal),
    Style1(Normal),
    Style2(Normal),
    Style3(Normal),
    Style4(Normal),
}

pub struct KnobStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    knob_float_state: knob::State,
    knob_int_state: knob::State,
    knob_db_state: knob::State,
    knob_freq_state: knob::State,
    knob_style1_state: knob::State,
    knob_style2_state: knob::State,
    knob_style3_state: knob::State,
    knob_style4_state: knob::State,

    float_tick_marks: tick_marks::Group,
    int_tick_marks: tick_marks::Group,
    db_tick_marks: tick_marks::Group,
    freq_tick_marks: tick_marks::Group,

    float_text_marks: text_marks::Group,
    int_text_marks: text_marks::Group,
    db_text_marks: text_marks::Group,
    freq_text_marks: text_marks::Group,

    output_text: String,
}

impl Default for KnobStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = LogDBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            int_range,
            db_range,
            freq_range,

            // initialize the state of the Knob widget
            knob_float_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            knob_int_state: knob::State::new(int_range.default_normal_param()),

            knob_db_state: knob::State::new(db_range.default_normal_param()),

            knob_freq_state: knob::State::new(
                freq_range.normal_param(1000.0, 1000.0),
            ),

            knob_style1_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            knob_style2_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            knob_style3_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            knob_style4_state: knob::State::new(
                float_range.default_normal_param(),
            ),

            float_tick_marks: tick_marks::Group::subdivided(
                1,
                1,
                1,
                Some(tick_marks::Tier::Two),
            ),

            int_tick_marks: tick_marks::Group::evenly_spaced(
                6,
                tick_marks::Tier::Two,
            ),

            db_tick_marks: vec![
                (db_range.map_to_normal(0.0), tick_marks::Tier::One),
                (db_range.map_to_normal(1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(12.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-12.0), tick_marks::Tier::Two),
            ]
            .into(),

            freq_tick_marks: vec![
                (freq_range.map_to_normal(20.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(50.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(100.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(200.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(400.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(1000.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(2000.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(5000.0), tick_marks::Tier::Two),
                (freq_range.map_to_normal(10000.0), tick_marks::Tier::One),
                (freq_range.map_to_normal(20000.0), tick_marks::Tier::Two),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center(
                "-1", "+1", "0",
            ),
            int_text_marks: text_marks::Group::evenly_spaced(&[
                "A", "B", "C", "D", "E", "F",
            ]),
            db_text_marks: text_marks::Group::min_max_and_center(
                "-12", "+12", "0",
            ),
            freq_text_marks: vec![
                (freq_range.map_to_normal(100.0), "100"),
                (freq_range.map_to_normal(1000.0), "1k"),
                (freq_range.map_to_normal(10000.0), "10k"),
            ]
            .into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl KnobStep {
    pub fn title(&self) -> &str {
        "Knobs"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Float(normal) => {
                self.output_text = crate::info_text_f32(
                    "KnobFloat",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.knob_int_state.snap_visible_to(&self.int_range);

                self.output_text = crate::info_text_i32(
                    "KnobInt",
                    self.int_range.unmap_to_value(normal),
                );
            }
            Message::DB(normal) => {
                self.output_text = crate::info_text_db(
                    "KnobDB",
                    self.db_range.unmap_to_value(normal),
                );
            }
            Message::Freq(normal) => {
                self.output_text = crate::info_text_freq(
                    "KnobFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::Style1(normal) => {
                self.output_text = crate::info_text_f32(
                    "KnobStyle1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style2(normal) => {
                self.output_text = crate::info_text_f32(
                    "KnobStyle2",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style3(normal) => {
                self.output_text = crate::info_text_f32(
                    "KnobStyle3",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style4(normal) => {
                self.output_text = crate::info_text_f32(
                    "KnobStyle4",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_float = Knob::new(&mut self.knob_float_state, Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let knob_int = Knob::new(&mut self.knob_int_state, Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let knob_db = Knob::new(&mut self.knob_db_state, Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let knob_freq = Knob::new(&mut self.knob_freq_state, Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let knob_style1 =
            Knob::new(&mut self.knob_style1_state, Message::Style1)
                .style(style::knob::CustomStyleCircle)
                .text_marks(&self.float_text_marks);

        let knob_style2 =
            Knob::new(&mut self.knob_style2_state, Message::Style2)
                .style(style::knob::CustomStyleLine);

        let knob_style3 =
            Knob::new(&mut self.knob_style3_state, Message::Style3)
                .style(style::knob::CustomArc);

        let knob_style4 =
            Knob::new(&mut self.knob_style4_state, Message::Style4)
                .style(style::knob::CustomArcBipolar);

        // push the widgets into rows
        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Float Range"))
                    .push(knob_float)
                    .push(Text::new("Log DB Range"))
                    .push(knob_db)
                    .push(Text::new("Custom Style 1"))
                    .push(knob_style1),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Int Range"))
                    .push(knob_int)
                    .push(Text::new("Freq Range"))
                    .push(knob_freq)
                    .push(Text::new("Custom Style 2"))
                    .push(knob_style2),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Custom Style 3"))
                    .push(knob_style3)
                    .push(Text::new("Custom Bipolar Style 4"))
                    .push(knob_style4),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Knobs").push(content).into()
    }
}
