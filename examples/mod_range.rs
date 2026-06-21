mod style;
mod util;

use iced::widget::{checkbox, column, container, row, text};
use iced::{Alignment, Element, Length, Result, Size, application};

use iced_audio::{
    FloatRange, Gesture, HSlider, Knob, ModRangeInput, ModulationRange, NormalParam, VSlider,
};

use crate::util::info_text::info_text_f32;

fn main() -> Result {
    application(
        ModRangeExample::default,
        ModRangeExample::update,
        ModRangeExample::view,
    )
    .window_size(Size::new(600.0, 400.0))
    .run()
}

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum ModRangesID {}

#[derive(Debug, Clone)]
pub enum Message {
    RangeStart(Gesture),
    RangeEnd(Gesture),
    Knob1(Gesture),
    HSlider1(Gesture),
    VSlider1(Gesture),
    ModKnob1(Gesture),
    ModKnob2(Gesture),
    ModRangeInput1(Gesture),
    ModRangeInput2(Gesture),
    ToggleModRange(bool),
}

pub struct ModRangeExample {
    knob_start_param: NormalParam,
    knob_end_param: NormalParam,

    knob1_param: NormalParam,
    h_slider1_param: NormalParam,
    v_slider1_param: NormalParam,
    mod_range_1: ModulationRange,

    auto_input1_param: NormalParam,
    knob_auto1_param: NormalParam,
    knob_auto1_mod_range: ModulationRange,

    auto_input2_param: NormalParam,
    knob_auto2_param: NormalParam,
    knob_auto2_mod_range: ModulationRange,

    show_modulation: bool,

    output_text: String,
}

impl Default for ModRangeExample {
    fn default() -> Self {
        Self {
            // initialize the state of the Knob widget
            knob_start_param: FloatRange::NORMAL.default_param(),
            knob_end_param: FloatRange::NORMAL.default_param(),

            mod_range_1: ModulationRange::default(),

            knob1_param: FloatRange::NORMAL.default_param(),

            h_slider1_param: FloatRange::NORMAL.default_param(),

            v_slider1_param: FloatRange::NORMAL.default_param(),

            auto_input1_param: FloatRange::NORMAL_BIPOLAR.default_param(),

            knob_auto1_param: FloatRange::NORMAL.default_param(),
            auto_input2_param: FloatRange::NORMAL_BIPOLAR.default_param(),
            knob_auto1_mod_range: ModulationRange::default(),

            knob_auto2_param: FloatRange::NORMAL.default_param(),
            output_text: String::from("Move a widget"),
            knob_auto2_mod_range: ModulationRange::default(),

            show_modulation: true,
        }
    }
}

impl ModRangeExample {
    pub fn title(&self) -> &str {
        "Modulation Ranges"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RangeStart(Gesture::Gesturing(normal)) => {
                self.knob_start_param.set(normal);
                self.output_text = info_text_f32("RangeStart", normal, &FloatRange::NORMAL);

                self.mod_range_1.start = normal;
            }
            Message::RangeEnd(Gesture::Gesturing(normal)) => {
                self.knob_end_param.set(normal);
                self.output_text = info_text_f32("RangeEnd", normal, &FloatRange::NORMAL);

                self.mod_range_1.end = normal;
            }
            Message::Knob1(Gesture::Gesturing(normal)) => {
                self.knob1_param.set(normal);
                self.output_text = info_text_f32("Knob1", normal, &FloatRange::NORMAL);
            }
            Message::HSlider1(Gesture::Gesturing(normal)) => {
                self.h_slider1_param.set(normal);
                self.output_text = info_text_f32("HSlider1", normal, &FloatRange::NORMAL);
            }
            Message::VSlider1(Gesture::Gesturing(normal)) => {
                self.v_slider1_param.normal = normal;
                self.output_text = info_text_f32("VSlider1", normal, &FloatRange::NORMAL);
            }
            Message::ModKnob1(Gesture::Gesturing(normal)) => {
                self.knob_auto1_param.set(normal);
                self.output_text = info_text_f32("ModKnob1", normal, &FloatRange::NORMAL);

                let mod_value =
                    FloatRange::NORMAL_BIPOLAR.unmap_to_value(self.auto_input1_param.normal);

                self.knob_auto1_mod_range.start = normal;
                self.knob_auto1_mod_range
                    .end
                    .set(self.knob_auto1_mod_range.start.as_f32() + mod_value);
            }
            Message::ModRangeInput1(Gesture::Gesturing(normal)) => {
                self.auto_input1_param.set(normal);
                self.output_text =
                    info_text_f32("ModRangeInput1", normal, &FloatRange::NORMAL_BIPOLAR);

                let value = FloatRange::NORMAL_BIPOLAR.unmap_to_value(normal);
                self.knob_auto1_mod_range
                    .end
                    .set(self.knob_auto1_param.normal.as_f32() + value);
            }
            Message::ModRangeInput2(Gesture::Gesturing(normal)) => {
                self.auto_input2_param.set(normal);
                self.output_text =
                    info_text_f32("ModRangeInput2", normal, &FloatRange::NORMAL_BIPOLAR);

                let value = FloatRange::NORMAL_BIPOLAR.unmap_to_value(normal);
                self.knob_auto2_mod_range
                    .end
                    .set(self.knob_auto2_param.normal.as_f32() + value);
            }
            Message::ModKnob2(Gesture::Gesturing(normal)) => {
                self.knob_auto2_param.set(normal);
                self.output_text = info_text_f32("ModKnob2", normal, &FloatRange::NORMAL_BIPOLAR);

                let mod_value =
                    FloatRange::NORMAL_BIPOLAR.unmap_to_value(self.auto_input2_param.normal);

                self.knob_auto2_mod_range.start = normal;
                self.knob_auto2_mod_range
                    .end
                    .set(self.knob_auto2_mod_range.start.as_f32() + mod_value);
            }
            Message::ToggleModRange(toggle) => {
                self.show_modulation = toggle;

                self.mod_range_1.filled_visible = toggle;
                self.knob_auto1_mod_range.filled_visible = toggle;
                self.knob_auto2_mod_range.filled_visible = toggle;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_start = Knob::new(self.knob_start_param).on_gesture(Message::RangeStart);
        let knob_end = Knob::new(self.knob_end_param).on_gesture(Message::RangeEnd);

        let mod_range_1 = self.show_modulation.then_some(&self.mod_range_1);

        let knob1 = Knob::new(self.knob1_param)
            .on_gesture(Message::Knob1)
            .mod_range(mod_range_1)
            .style(style::knob::CustomArc);

        let h_slider1 = HSlider::new(self.h_slider1_param)
            .on_gesture(Message::HSlider1)
            .mod_range(mod_range_1)
            .style(style::h_slider::RectStyle);

        let v_slider1 = VSlider::new(self.v_slider1_param)
            .on_gesture(Message::VSlider1)
            .width(Length::Fixed(30.0))
            .mod_range(mod_range_1)
            .style(style::v_slider::RectStyle);

        let auto_input1 = ModRangeInput::new(self.auto_input1_param)
            .on_gesture(Message::ModRangeInput1)
            .size(Length::from(10))
            .style(style::mod_range_input::CustomStyle)
            .enabled(self.show_modulation);

        let knob_auto1 = Knob::new(self.knob_auto1_param)
            .on_gesture(Message::ModKnob1)
            .mod_range(self.show_modulation.then_some(&self.knob_auto1_mod_range))
            .style(style::knob::CustomStyleCircle);

        let auto_input2 = ModRangeInput::new(self.auto_input2_param)
            .on_gesture(Message::ModRangeInput2)
            .size(Length::from(15))
            .style(iced_audio::mod_range_input::InvisibleStyle)
            .enabled(self.show_modulation);

        let knob_auto2 = Knob::new(self.knob_auto2_param)
            .on_gesture(Message::ModKnob2)
            .mod_range(self.show_modulation.then_some(&self.knob_auto2_mod_range))
            .style(style::knob::CustomStyleCircle);

        // push the widgets into rows
        let knob_row = row![
            column![
                column![text("Range Start"), knob_start].spacing(8),
                column![text("Range End"), knob_end].spacing(8),
                checkbox(self.show_modulation)
                    .label("Show Modulation")
                    .on_toggle(Message::ToggleModRange),
            ]
            .max_width(130)
            .spacing(16),
            container(
                column![knob1, h_slider1, v_slider1]
                    .max_width(130)
                    .align_x(Alignment::Center)
                    .spacing(20)
            )
            .max_height(250),
            column![
                column![
                    text("Custom Style with ModRangeInput"),
                    column![auto_input1, knob_auto1]
                        .spacing(12)
                        .align_x(Alignment::Center)
                ]
                .width(Length::Fill)
                .spacing(8)
                .align_x(Alignment::Center),
                column![
                    text("Custom Style with invisible ModRangeInput"),
                    column![auto_input2, knob_auto2]
                        .spacing(0)
                        .align_x(Alignment::Center)
                ]
                .width(Length::Fill)
                .spacing(8)
                .align_x(Alignment::Center)
            ]
            .width(Length::Fill)
            .spacing(16),
        ]
        .spacing(16);

        column![knob_row, text(&self.output_text).size(16)]
            .spacing(20)
            .padding(20)
            .into()
    }
}
