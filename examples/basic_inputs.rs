extern crate iced;

mod basic_inputs_steps;
use basic_inputs_steps::*;

use iced::{
    button, Button, scrollable, Scrollable, Column, Row, Container, Element,
    HorizontalAlignment, Length, Space, Color, Sandbox, Settings, Text
};

pub use basic_inputs_steps::style;

static STARTING_STEP: usize = 0;

pub fn main() {
    BasicInputs::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

pub struct BasicInputs {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
    debug: bool,
}

impl Sandbox for BasicInputs {
    type Message = Message;

    fn new() -> BasicInputs {
        BasicInputs {
            steps: Steps::default(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced Audio Basic Inputs", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg, &mut self.debug);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let BasicInputs {
            steps,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(steps.view(self.debug).map(Message::StepMessage))
            .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable = Scrollable::new(scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Default for Steps {
    fn default() -> Self {
        Self {
            steps: vec![
                Step::Welcome,
                Step::HSliders(Default::default()),
                Step::VSliders(Default::default()),
                Step::Knobs(Default::default()),
                Step::XYPads(Default::default()),
                Step::Ramps(Default::default()),
            ],
            current: STARTING_STEP,
        }
    }
}

impl Steps {
    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        self.steps[self.current].update(msg, debug);
    }

    fn view(&mut self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

pub enum Step {
    Welcome,
    HSliders(step_h_sliders::HSliderStep),
    VSliders(step_v_sliders::VSliderStep),
    Knobs(step_knobs::KnobsStep),
    XYPads(step_xy_pads::XYPadStep),
    Ramps(step_ramps::RampStep),
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    HSlidersMsg(step_h_sliders::Message),
    VSlidersMsg(step_v_sliders::Message),
    KnobsMsg(step_knobs::Message),
    XYPadsMsg(step_xy_pads::Message),
    RampsMsg(step_ramps::Message),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage, _debug: &mut bool) {
        match msg {
            StepMessage::HSlidersMsg(msg) => {
                if let Step::HSliders(step) = self { step.update(msg); };
            },
            StepMessage::VSlidersMsg(msg) => {
                if let Step::VSliders(step) = self { step.update(msg); };
            },
            StepMessage::KnobsMsg(msg) => {
                if let Step::Knobs(step) = self { step.update(msg); };
            },
            StepMessage::XYPadsMsg(msg) => {
                if let Step::XYPads(step) = self { step.update(msg); };
            },
            StepMessage::RampsMsg(msg) => {
                if let Step::Ramps(step) = self { step.update(msg); };
            },
        }
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::HSliders(step) => step.title(),
            Step::VSliders(step) => step.title(),
            Step::Knobs(step) => step.title(),
            Step::XYPads(step) => step.title(),
            Step::Ramps(step) => step.title(),
        }
    }

    fn view(&mut self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::HSliders(step) => {
                step.view(debug).map(StepMessage::HSlidersMsg)
            },
            Step::VSliders(step) => {
                step.view(debug).map(StepMessage::VSlidersMsg)
            },
            Step::Knobs(step) => {
                step.view(debug).map(StepMessage::KnobsMsg)
            },
            Step::XYPads(step) => {
                step.view(debug).map(StepMessage::XYPadsMsg)
            },
            Step::Ramps(step) => {
                step.view(debug).map(StepMessage::RampsMsg)
            },
        }
        .into()
    }

    pub fn container<Msg>(title: &str) -> Column<'a, Msg> {
        Column::new().spacing(20).push(Text::new(title).size(44))
    }

    fn welcome() -> Element<'a, StepMessage> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a simple tour showcasing basic input widgets \
                designed specifically for audio software applications such as \
                VST / LV2 plugins.",
            ))
            .push(Text::new(
                "Iced is a cross-platform GUI library for Rust focused on \
                simplicity and type-safety. It is heavily inspired by Elm. \
                Iced Audio is an extension for Iced.",
            ))
            .push(Text::new(
                "For each control, holding down the Ctrl key will make fine \
                adjustments, and double-clicking will set the control to its \
                default value.",
            ))
        .into()
    }
}

fn button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

// generates the text for an output

pub fn info_text_f32<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {:?}  |  value: {:.3}", id, value)
}

pub fn info_text_i32<ID: std::fmt::Debug>(id: ID, value: i32) -> String {
    format!("id: {:?}  |  value: {}", id, value)
}

pub fn info_text_db<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {:?}  |  value: {:.3} dB", id, value)
}

pub fn info_text_octave<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    if value < 1000.0 {
        format!("id: {:?}  |  value: {:.2} Hz", id, value)
    } else {
        format!("id: {:?}  |  value: {:.2} kHz", id, value / 1000.0)
    }
}