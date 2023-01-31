mod steps;
mod style;
use steps::*;

use iced::widget::{
    column, container, horizontal_space, row, scrollable, text, Button, Column,
};
use iced::{alignment, Color, Element, Length, Sandbox, Settings};
use iced_native::widget::tree::Tag;
use iced_native::widget::{Operation, Tree};
use iced_native::{
    event, layout, mouse, overlay, renderer, Clipboard, Event, Layout, Point,
    Rectangle, Shell, Widget,
};

use std::marker::PhantomData;

static STARTING_STEP: usize = 0;

pub fn main() {
    InputsTour::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

pub struct InputsTour {
    steps: Steps,
    debug: bool,
}

impl Sandbox for InputsTour {
    type Message = Message;

    fn new() -> InputsTour {
        InputsTour {
            steps: Steps::default(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced Audio Inputs Tour", self.steps.title())
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

    fn view(&self) -> Element<Message> {
        let InputsTour { steps, .. } = self;

        let mut controls = row![];

        if steps.has_previous() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary.into()),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button("Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary.into()),
            );
        }

        let content: Element<_> = column![
            steps.view(self.debug).map(Message::StepMessage),
            controls,
        ]
        .max_width(540)
        .spacing(20)
        .padding(20)
        .into();

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            })
            .width(Length::Fill)
            .center_x(),
        );

        container(scrollable).height(Length::Fill).center_y().into()
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
                Step::ModRanges(Default::default()),
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

    fn view(&self, debug: bool) -> Element<StepMessage> {
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
    Knobs(step_knobs::KnobStep),
    ModRanges(step_mod_ranges::ModRanges),
    XYPads(step_xy_pads::XYPadStep),
    Ramps(step_ramps::RampStep),
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    HSlidersMsg(step_h_sliders::Message),
    VSlidersMsg(step_v_sliders::Message),
    KnobsMsg(step_knobs::Message),
    ModRangesMsg(step_mod_ranges::Message),
    XYPadsMsg(step_xy_pads::Message),
    RampsMsg(step_ramps::Message),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage, _debug: &mut bool) {
        match msg {
            StepMessage::HSlidersMsg(msg) => {
                if let Step::HSliders(step) = self {
                    step.update(msg);
                };
            }
            StepMessage::VSlidersMsg(msg) => {
                if let Step::VSliders(step) = self {
                    step.update(msg);
                };
            }
            StepMessage::KnobsMsg(msg) => {
                if let Step::Knobs(step) = self {
                    step.update(msg);
                };
            }
            StepMessage::ModRangesMsg(msg) => {
                if let Step::ModRanges(step) = self {
                    step.update(msg);
                };
            }
            StepMessage::XYPadsMsg(msg) => {
                if let Step::XYPads(step) = self {
                    step.update(msg);
                };
            }
            StepMessage::RampsMsg(msg) => {
                if let Step::Ramps(step) = self {
                    step.update(msg);
                };
            }
        }
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::HSliders(step) => step.title(),
            Step::VSliders(step) => step.title(),
            Step::Knobs(step) => step.title(),
            Step::ModRanges(step) => step.title(),
            Step::XYPads(step) => step.title(),
            Step::Ramps(step) => step.title(),
        }
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::HSliders(step) => {
                step.view(debug).map(StepMessage::HSlidersMsg)
            }
            Step::VSliders(step) => {
                step.view(debug).map(StepMessage::VSlidersMsg)
            }
            Step::Knobs(step) => step.view(debug).map(StepMessage::KnobsMsg),
            Step::ModRanges(step) => {
                step.view(debug).map(StepMessage::ModRangesMsg)
            }
            Step::XYPads(step) => step.view(debug).map(StepMessage::XYPadsMsg),
            Step::Ramps(step) => step.view(debug).map(StepMessage::RampsMsg),
        }
    }

    fn welcome() -> Element<'a, StepMessage> {
        StepContainer::<Self, _, _>::new("Welcome!")
            .push(text(
                "This is a simple tour showcasing basic input widgets \
                designed specifically for audio software applications such as \
                VST / LV2 plugins.",
            ))
            .push(text(
                "Iced is a cross-platform GUI library for Rust focused on \
                simplicity and type-safety. It is heavily inspired by Elm. \
                Iced Audio is an extension for Iced.",
            ))
            .push(text(
                "For each control, holding down the Ctrl key will make fine \
                adjustments, and double-clicking will set the control to its \
                default value.",
            ))
            .into()
    }
}

/// A Container for `Step` implementations.
///
/// Due to `Tree` diff implementation, when switching `Step`s:
/// for all the container's children of the same type and
/// in the same position as in previous `Step` `Container`,
/// the `State` is not reset.
///
/// This `struct` uses the type of the `Step` implementation to
/// form a specific `Tag` which forces the tree to be rebuilt
/// when switching `Step`s.
pub struct StepContainer<'a, Step, Message, Renderer> {
    col: Column<'a, Message, Renderer>,
    step_type: PhantomData<Step>,
}

impl<'a, Step, Message, Renderer> StepContainer<'a, Step, Message, Renderer>
where
    Step: 'static,
    Renderer: 'a + iced_native::text::Renderer,
    Renderer::Theme: iced_native::widget::text::StyleSheet,
{
    /// Builds a new step container for the step implementation with the provided title.
    pub fn new(title: &str) -> Self {
        Self {
            col: column![text(title).size(44)].spacing(20),
            step_type: PhantomData,
        }
    }

    pub fn push(
        self,
        child: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        Self {
            col: self.col.push(child),
            step_type: PhantomData,
        }
    }
}

impl<'a, Step, Message, Renderer> Widget<Message, Renderer>
    for StepContainer<'a, Step, Message, Renderer>
where
    Step: 'static,
    Renderer: iced_native::Renderer,
{
    fn tag(&self) -> Tag {
        // Force the `Tag` to be `Step`-implementation dependent.
        Tag::of::<Step>()
    }

    // Defere the rest of the `Widget` implementation to the inner `Column`.
    fn children(&self) -> Vec<Tree> {
        self.col.children()
    }
    fn diff(&self, tree: &mut Tree) {
        Widget::diff(&self.col, tree)
    }
    fn width(&self) -> Length {
        Widget::width(&self.col)
    }
    fn height(&self) -> Length {
        Widget::height(&self.col)
    }
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        Widget::layout(&self.col, renderer, limits)
    }
    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        Widget::operate(&self.col, tree, layout, renderer, operation);
    }
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        Widget::on_event(
            &mut self.col,
            tree,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        Widget::mouse_interaction(
            &self.col,
            tree,
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        Widget::draw(
            &self.col,
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        Widget::overlay(&mut self.col, tree, layout, renderer)
    }
}

impl<'a, Step, Message, Renderer>
    From<StepContainer<'a, Step, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Step: 'static,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(
        step_container: StepContainer<'a, Step, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(step_container)
    }
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(100))
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

pub fn info_text_freq<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    if value < 1000.0 {
        format!("id: {:?}  |  value: {:.2} Hz", id, value)
    } else {
        format!("id: {:?}  |  value: {:.2} kHz", id, value / 1000.0)
    }
}
