mod style;
mod util;

use iced::{
    Element, Length, Result, Size, application,
    widget::{column, row, text},
};
use iced_audio::{
    DBRange, FloatRange, FreqRange, Gesture, IntRange, Knob, Normal, NormalParam, text_marks,
    tick_marks,
};

use crate::util::info_text::{info_text_db, info_text_f32, info_text_freq, info_text_i32};

const INT_RANGE: IntRange = IntRange::new(0, 5);

fn main() -> Result {
    application(KnobExample::default, KnobExample::update, KnobExample::view)
        .window_size(Size::new(600.0, 400.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Float(Gesture),
    Int(Gesture),
    DB(Gesture),
    Freq(Gesture),
    Style1(Gesture),
    Style2(Gesture),
    Style3(Gesture),
    Style4(Gesture),
    Style5(Gesture),
}

pub struct KnobExample {
    float_param: NormalParam,
    int_param: NormalParam,
    db_param: NormalParam,
    freq_param: NormalParam,
    style1_param: NormalParam,
    style2_param: NormalParam,
    style3_param: NormalParam,
    style4_param: NormalParam,
    style5_param: NormalParam,

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

impl Default for KnobExample {
    fn default() -> Self {
        Self {
            // initialize the state of the Knob widget
            float_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            int_param: INT_RANGE.default_param(),
            db_param: DBRange::NEG_12_TO_12.default_param(),
            freq_param: FreqRange::HZ_20_TO_20K.param(1000.0, 1000.0),
            style1_param: FloatRange::NORMAL.default_param(),
            style2_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            style3_param: FloatRange::NORMAL.default_param(),
            style4_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            style5_param: FloatRange::NORMAL.param(-0.6, -0.6),

            float_tick_marks: tick_marks::Group::subdivided(1, 1, 1, Some(tick_marks::Tier::Two)),
            int_tick_marks: tick_marks::Group::evenly_spaced(6, tick_marks::Tier::Two),
            db_tick_marks: vec![
                (DBRange::NEG_12_TO_12.map_db(0.0), tick_marks::Tier::One),
                (DBRange::NEG_12_TO_12.map_db(1.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(3.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(6.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(9.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(12.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-1.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-3.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-6.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-9.0), tick_marks::Tier::Two),
                (DBRange::NEG_12_TO_12.map_db(-12.0), tick_marks::Tier::Two),
            ]
            .into(),
            freq_tick_marks: vec![
                (
                    FreqRange::HZ_20_TO_20K.map_freq(20.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(50.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(100.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(200.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(400.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(1000.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(2000.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(5000.0),
                    tick_marks::Tier::Two,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(10000.0),
                    tick_marks::Tier::One,
                ),
                (
                    FreqRange::HZ_20_TO_20K.map_freq(20000.0),
                    tick_marks::Tier::Two,
                ),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center("-1", "+1", "0"),
            int_text_marks: text_marks::Group::evenly_spaced(&["A", "B", "C", "D", "E", "F"]),
            db_text_marks: text_marks::Group::min_max_and_center("-12", "+12", "0"),
            freq_text_marks: vec![
                (FreqRange::HZ_20_TO_20K.map_freq(100.0), "100"),
                (FreqRange::HZ_20_TO_20K.map_freq(1000.0), "1k"),
                (FreqRange::HZ_20_TO_20K.map_freq(10000.0), "10k"),
            ]
            .into(),

            output_text: String::new(),
        }
    }
}

impl KnobExample {
    fn update(&mut self, message: Message) {
        dbg!(&message);

        match message {
            Message::Float(Gesture::Gesturing(normal)) => {
                self.float_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::Int(Gesture::Gesturing(normal)) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.int_param.set(INT_RANGE.snap(normal));
                self.output_text = info_text_i32("KnobInt", normal, &INT_RANGE);
            }
            Message::DB(Gesture::Gesturing(normal)) => {
                self.db_param.set(normal);
                self.output_text = info_text_db("KnobInt", normal, &DBRange::NEG_12_TO_12);
            }
            Message::Freq(Gesture::Gesturing(normal)) => {
                self.freq_param.set(normal);
                self.output_text = info_text_freq("KnobInt", normal, &FreqRange::HZ_20_TO_20K);
            }
            Message::Style1(Gesture::Gesturing(normal)) => {
                self.style1_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL);
            }
            Message::Style2(Gesture::Gesturing(normal)) => {
                self.style2_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::Style3(Gesture::Gesturing(normal)) => {
                self.style3_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL);
            }
            Message::Style4(Gesture::Gesturing(normal)) => {
                self.style4_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL_BIPOLAR);
            }
            Message::Style5(Gesture::Gesturing(normal)) => {
                self.style5_param.set(normal);
                self.output_text = info_text_f32("KnobFloat", normal, &FloatRange::NORMAL);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_float = Knob::new(self.float_param)
            .on_gesture(Message::Float)
            .tick_marks(&self.float_tick_marks)
            .text_marks(&self.float_text_marks);

        let knob_int = Knob::new(self.int_param)
            .on_gesture(Message::Int)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let knob_db = Knob::new(self.db_param)
            .on_gesture(Message::DB)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let knob_freq = Knob::new(self.freq_param)
            .on_gesture(Message::Freq)
            .tick_marks(&self.freq_tick_marks)
            .text_marks(&self.freq_text_marks);

        let knob_style1 = Knob::new(self.style1_param)
            .on_gesture(Message::Style1)
            .style(style::knob::CustomStyleCircle)
            .text_marks(&self.float_text_marks);

        let knob_style2 = Knob::new(self.style2_param)
            .on_gesture(Message::Style2)
            .style(style::knob::CustomStyleLine);

        let knob_style3 = Knob::new(self.style3_param)
            .on_gesture(Message::Style3)
            .style(style::knob::CustomArc);

        let knob_style4 = Knob::new(self.style4_param)
            .on_gesture(Message::Style4)
            .style(style::knob::CustomArcBipolar);

        let knob_style5 = Knob::new(self.style5_param)
            .on_gesture(Message::Style5)
            .bipolar_center(Normal::new(0.2))
            .style(style::knob::CustomArcBipolar);

        // push the widgets into rows
        let knob_row = row![
            column![
                column![text("Float Range"), knob_float].spacing(28),
                column![text("Log DB Range"), knob_db].spacing(28),
                column![text("Custom Style 1"), knob_style1].spacing(28),
            ]
            .width(Length::Fill)
            .spacing(32),
            column![
                column![text("Int Range"), knob_int].spacing(28),
                column![text("Freq Range"), knob_freq].spacing(28),
                column![text("Custom Style 2"), knob_style2].spacing(28),
            ]
            .width(Length::Fill)
            .spacing(32),
            column![
                column![text("Custom Style 3"), knob_style3].spacing(28),
                column![text("Custom BP Style 4"), knob_style4].spacing(28),
                column![text("Custom BP Style 5"), knob_style5].spacing(28),
            ]
            .width(Length::Fill)
            .spacing(32)
        ]
        .spacing(20);

        column![knob_row, text(&self.output_text).size(16)]
            .spacing(20)
            .padding(31)
            .into()
    }
}
