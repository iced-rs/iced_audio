use iced::widget::{column, row, text};
use iced::{Element, Length};

use iced_audio::{
    text_marks, tick_marks, FloatRange, FreqRange, IntRange, Knob, LogDBRange,
    Normal, NormalParam,
};

use crate::{style, StepContainer};

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
    Style5(Normal),
}

pub struct KnobStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    knob_float_param: NormalParam,
    knob_int_param: NormalParam,
    knob_db_param: NormalParam,
    knob_freq_param: NormalParam,
    knob_style1_param: NormalParam,
    knob_style2_param: NormalParam,
    knob_style3_param: NormalParam,
    knob_style4_param: NormalParam,
    knob_style5_param: NormalParam,

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
            knob_float_param: float_range.default_normal_param(),
            knob_int_param: int_range.default_normal_param(),
            knob_db_param: db_range.default_normal_param(),
            knob_freq_param: freq_range.normal_param(1000.0, 1000.0),
            knob_style1_param: float_range.default_normal_param(),
            knob_style2_param: float_range.default_normal_param(),
            knob_style3_param: float_range.default_normal_param(),
            knob_style4_param: float_range.default_normal_param(),
            knob_style5_param: float_range.normal_param(-0.6, -0.6),

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
                self.knob_float_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobFloat",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Int(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.knob_int_param.update(self.int_range.snapped(normal));

                self.output_text = crate::info_text_i32(
                    "KnobInt",
                    self.int_range.unmap_to_value(normal),
                );
            }
            Message::DB(normal) => {
                self.knob_db_param.update(normal);

                self.output_text = crate::info_text_db(
                    "KnobDB",
                    self.db_range.unmap_to_value(normal),
                );
            }
            Message::Freq(normal) => {
                self.knob_freq_param.update(normal);

                self.output_text = crate::info_text_freq(
                    "KnobFreq",
                    self.freq_range.unmap_to_value(normal),
                );
            }
            Message::Style1(normal) => {
                self.knob_style1_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobStyle1",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style2(normal) => {
                self.knob_style2_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobStyle2",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style3(normal) => {
                self.knob_style3_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobStyle3",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style4(normal) => {
                self.knob_style4_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobStyle4",
                    self.float_range.unmap_to_value(normal),
                );
            }
            Message::Style5(normal) => {
                self.knob_style5_param.update(normal);

                self.output_text = crate::info_text_f32(
                    "KnobStyle5",
                    self.float_range.unmap_to_value(normal),
                );
            }
        }
    }

    pub fn view(&self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_float = Knob::new(self.knob_float_param, Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let knob_int = Knob::new(self.knob_int_param, Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let knob_db = Knob::new(self.knob_db_param, Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let knob_freq = Knob::new(self.knob_freq_param, Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let knob_style1 = Knob::new(self.knob_style1_param, Message::Style1)
            .style(style::knob::CustomStyleCircle)
            .text_marks(&self.float_text_marks);

        let knob_style2 = Knob::new(self.knob_style2_param, Message::Style2)
            .style(style::knob::CustomStyleLine);

        let knob_style3 = Knob::new(self.knob_style3_param, Message::Style3)
            .style(style::knob::CustomArc);

        let knob_style4 = Knob::new(self.knob_style4_param, Message::Style4)
            .style(style::knob::CustomArcBipolar);

        let knob_style5 = Knob::new(self.knob_style5_param, Message::Style5)
            .bipolar_center(Normal::new(0.2))
            .style(style::knob::CustomArcBipolar);

        // push the widgets into rows
        let knob_row = row![
            column![
                text("Float Range"),
                knob_float,
                text("Log DB Range"),
                knob_db,
                text("Custom Style 1"),
                knob_style1,
            ]
            .width(Length::Fill)
            .spacing(20),
            column![
                text("Int Range"),
                knob_int,
                text("Freq Range"),
                knob_freq,
                text("Custom Style 2"),
                knob_style2,
            ]
            .width(Length::Fill)
            .spacing(20),
            column![
                text("Custom Style 3"),
                knob_style3,
                text("Custom Bipolar Style 4"),
                knob_style4,
                text("Custom Bipolar Style 5 (Off-Center)"),
                knob_style5,
            ]
            .width(Length::Fill)
            .spacing(20)
        ]
        .spacing(20);

        let content = column![knob_row, text(&self.output_text).size(16)]
            .spacing(20)
            .padding(20);

        StepContainer::<Self, _, _>::new("Knobs")
            .push(content)
            .into()
    }
}
