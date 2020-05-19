extern crate iced;

mod tour_steps;
use tour_steps::*;

use iced::{
    button, Button, scrollable, Scrollable, Column, Row, Container, Element,
    HorizontalAlignment, Length, Space, Color, Sandbox, Settings, Text
};

static STARTING_STEP: usize = 3;

pub fn main() {
    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
    debug: bool,
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Tour {
        Tour {
            steps: Steps::default(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced Audio", self.steps.title())
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
        let Tour {
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
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    HSlidersMsg(step_h_sliders::Message),
    VSlidersMsg(step_v_sliders::Message),
    KnobsMsg(step_knobs::Message),
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
        }
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::HSliders(step) => step.title(),
            Step::VSliders(step) => step.title(),
            Step::Knobs(step) => step.title(),
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
        }
        .into()
    }

    pub fn container<Msg>(title: &str) -> Column<'a, Msg> {
        Column::new().spacing(20).push(Text::new(title).size(44))
    }

    fn welcome() -> Element<'a, StepMessage> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a simple tour meant to showcase a bunch of widgets \
                designed specifically for audio software applications such as \
                VST / LV2 plugins.",
            ))
            .push(Text::new(
                "Iced is a cross-platform GUI library for Rust focused on \
                simplicity and type-safety. It is heavily inspired by Elm. \
                Iced Audio is an extension for Iced.",
            ))
            .push(Text::new(
                "For most controls, holding down the Ctrl key will make fine \
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

pub mod style {
    use iced::{button, Background, Color, Vector, image};
    use iced_audio::{
        h_slider, v_slider
    };

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => BUTTON_PRIMARY_COLOR,
                    Button::Secondary => BUTTON_SECONDARY_COLOR,
                })),
                border_radius: 12,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }

    pub const BUTTON_PRIMARY_COLOR: Color = Color::from_rgb(
        0x32 as f32 / 255.0,
        0x80 as f32 / 255.0,
        0xC8 as f32 / 255.0,
    );

    pub const BUTTON_SECONDARY_COLOR: Color = Color::from_rgb(
        0x62 as f32 / 255.0,
        0x69 as f32 / 255.0,
        0x73 as f32 / 255.0,
    );

    pub const EMPTY_COLOR: Color = Color::from_rgb(
        0x42 as f32 / 255.0,
        0x46 as f32 / 255.0,
        0x4D as f32 / 255.0,
    );
    pub const BORDER_COLOR: Color = Color::from_rgb(
        0x3C as f32 / 255.0,
        0x3F as f32 / 255.0,
        0x48 as f32 / 255.0,
    );
    pub const FILLED_COLOR: Color = Color::from_rgb(
        0x29 as f32 / 255.0,
        0x66 as f32 / 255.0,
        0xA3 as f32 / 255.0,
    );
    pub const FILLED_HOVER_COLOR: Color = Color::from_rgb(
        0x33 as f32 / 255.0,
        0x70 as f32 / 255.0,
        0xAD as f32 / 255.0,
    );
    pub const HANDLE_COLOR: Color = Color::from_rgb(
        0x75 as f32 / 255.0,
        0xC2 as f32 / 255.0,
        0xFF as f32 / 255.0,
    );
    pub const HANDLE_HOVER_COLOR: Color = Color::from_rgb(
        0x7A as f32 / 255.0,
        0xC7 as f32 / 255.0,
        0xFF as f32 / 255.0,
    );

    // Custom style for the Rect HSlider

    pub struct HSliderRectStyle;
    impl h_slider::StyleSheet for HSliderRectStyle {
        fn active(&self) -> h_slider::Style {
            h_slider::Style::Rect(
            h_slider::RectStyle {
                back_empty_color: EMPTY_COLOR,
                back_filled_color: FILLED_COLOR,
                border_color: BORDER_COLOR,
                border_radius: 2,
                border_width: 1,
                handle_width: 4,
                handle_color: HANDLE_COLOR,
                handle_filled_gap: 1,
            })
        }
        
        fn hovered(&self) -> h_slider::Style {
            let active = self.active();
            if let h_slider::Style::Rect(active) = active {
                h_slider::Style::Rect(
                h_slider::RectStyle {
                    back_filled_color: FILLED_HOVER_COLOR,
                    handle_width: 5,
                    ..active
                })
            } else { active }
        }
        
        fn dragging(&self) -> h_slider::Style {
            self.hovered()
        }

        fn height(&self) -> u16 {
            24
        }
    }

    // Custom style for the Rect VSlider

    pub struct VSliderRectStyle;
    impl v_slider::StyleSheet for VSliderRectStyle {
        fn active(&self) -> v_slider::Style {
            v_slider::Style::Rect(
            v_slider::RectStyle {
                back_empty_color: EMPTY_COLOR,
                back_filled_color: FILLED_COLOR,
                border_color: BORDER_COLOR,
                border_radius: 2,
                border_width: 1,
                handle_height: 4,
                handle_color: HANDLE_COLOR,
                handle_filled_gap: 1,
            })
        }
        
        fn hovered(&self) -> v_slider::Style {
            let active = self.active();
            if let v_slider::Style::Rect(active) = active {
                v_slider::Style::Rect(
                v_slider::RectStyle {
                    back_filled_color: FILLED_HOVER_COLOR,
                    handle_height: 5,
                    ..active
                })
            } else { active }
        }
        
        fn dragging(&self) -> v_slider::Style {
            self.hovered()
        }

        fn width(&self) -> u16 {
            26
        }
    }

    // Custom style for the Rect Bipolar HSlider

    pub struct HSliderRectBipolarStyle;
    impl h_slider::StyleSheet for HSliderRectBipolarStyle {
        fn active(&self) -> h_slider::Style {
            h_slider::Style::RectBipolar(
            h_slider::RectBipolarStyle {
                back_left_empty_color: EMPTY_COLOR,
                back_left_filled_color: FILLED_COLOR,
                back_right_empty_color: EMPTY_COLOR,
                back_right_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
                border_color: BORDER_COLOR,
                border_radius: 2,
                border_width: 1,
                handle_width: 4,
                handle_left_color: HANDLE_COLOR,
                handle_right_color: Color::from_rgb(0.0, 0.9, 0.0),
                handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
                handle_filled_gap: 1,
            })
        }
        
        fn hovered(&self) -> h_slider::Style {
            let active = self.active();
            if let h_slider::Style::RectBipolar(active) = active {
                h_slider::Style::RectBipolar(
                h_slider::RectBipolarStyle {
                    back_left_filled_color: FILLED_HOVER_COLOR,
                    back_right_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
                    handle_width: 5,
                    ..active
                })
            } else { active }
        }
        
        fn dragging(&self) -> h_slider::Style {
            self.hovered()
        }

        fn height(&self) -> u16 {
            24
        }
    }

    // Custom style for the Rect Bipolar VSlider

    pub struct VSliderRectBipolarStyle;
    impl v_slider::StyleSheet for VSliderRectBipolarStyle {
        fn active(&self) -> v_slider::Style {
            v_slider::Style::RectBipolar(
            v_slider::RectBipolarStyle {
                back_bottom_empty_color: EMPTY_COLOR,
                back_bottom_filled_color: FILLED_COLOR,
                back_top_empty_color: EMPTY_COLOR,
                back_top_filled_color: Color::from_rgb(0.0, 0.605, 0.0),
                border_color: BORDER_COLOR,
                border_radius: 2,
                border_width: 1,
                handle_height: 4,
                handle_bottom_color: HANDLE_COLOR,
                handle_top_color: Color::from_rgb(0.0, 0.9, 0.0),
                handle_center_color: Color::from_rgb(0.7, 0.7, 0.7),
                handle_filled_gap: 1,
            })
        }
        
        fn hovered(&self) -> v_slider::Style {
            let active = self.active();
            if let v_slider::Style::RectBipolar(active) = active {
                v_slider::Style::RectBipolar(
                v_slider::RectBipolarStyle {
                    back_bottom_filled_color: FILLED_HOVER_COLOR,
                    back_top_filled_color: Color::from_rgb(0.0, 0.64, 0.0),
                    handle_height: 5,
                    ..active
                })
            } else { active }
        }
        
        fn dragging(&self) -> v_slider::Style {
            self.hovered()
        }

        fn width(&self) -> u16 {
            26
        }
    }

    // Custom style for the Texture HSlider

    pub struct HSliderTextureStyle(pub image::Handle);
    impl h_slider::StyleSheet for HSliderTextureStyle {
        fn active(&self) -> h_slider::Style {
            h_slider::Style::Texture(
            h_slider::TextureStyle {
                rail_colors: ([0.56, 0.56, 0.56, 0.75].into(), Color::WHITE),
                texture: self.0.clone(),
                handle_width: 38,
                handle_height: 20,
                texture_padding: None,
            })
        }
        
        fn hovered(&self) -> h_slider::Style {
            self.active()
        }
        
        fn dragging(&self) -> h_slider::Style {
            self.active()
        }

        fn height(&self) -> u16 {
            24
        }
    }

    // Custom style for the Texture VSlider

    pub struct VSliderTextureStyle(pub image::Handle);
    impl v_slider::StyleSheet for VSliderTextureStyle {
        fn active(&self) -> v_slider::Style {
            v_slider::Style::Texture(
            v_slider::TextureStyle {
                rail_colors: ([0.56, 0.56, 0.56, 0.75].into(), Color::WHITE),
                texture: self.0.clone(),
                handle_width: 20,
                handle_height: 38,
                texture_padding: None,
            })
        }
        
        fn hovered(&self) -> v_slider::Style {
            self.active()
        }
        
        fn dragging(&self) -> v_slider::Style {
            self.active()
        }

        fn width(&self) -> u16 {
            24
        }
    }

    // Custom style for the Vector Style Knob

    /*
    pub struct KnobVectorStyle;
    impl knob::StyleSheet for KnobVectorStyle {
        fn active(&self) -> knob::Style {
            knob::Style::Vector(
            knob::VectorStyle {
                knob_color: EMPTY_COLOR,
                knob_border_width: 1,
                knob_border_color: BORDER_COLOR,
                notch_color: HANDLE_COLOR,
                notch_width: 5,
                notch_height: 10,
                notch_offset: 1,
                inner_circle: Some(knob::InnerCircle {
                    scale: 0.5,
                    color: HANDLE_COLOR,
                    border_width: 1,
                    border_color: Color::BLACK,
                })
            })
        }

        fn hovered(&self) -> knob::Style {
            let active = self.active();
            if let knob::Style::Vector(active) = self.active() {

            knob::Style::Vector(
            knob::VectorStyle {
                notch_width: 3,
                inner_circle: Some(knob::InnerCircle {
                    scale: 0.47,
                    color: HANDLE_COLOR,
                    border_width: 1,
                    border_color: Color::BLACK,
                }),
                ..active
            })

            } else { active }
        }

        fn dragging(&self) -> knob::Style {
            self.hovered()
        }

        fn diameter(&self) -> u16 {
            33
        }
    }
    */

    /*
    // Custom style for the Texture Knob

    pub struct KnobTextureStyle(pub image::Handle);
    impl knob::StyleSheet for KnobTextureStyle {
        fn active(&self) -> knob::Style {
            knob::Style::Texture(
            knob::TextureStyle {
                texture: self.0.clone(),
                knob_width: 32,
                knob_height: 33,
                texture_padding: None,
            })
        }
        
        fn hovered(&self) -> knob::Style {
            self.active()
        }
        
        fn dragging(&self) -> knob::Style {
            self.active()
        }

        fn diameter(&self) -> u16 {
            33
        }
    }
    */
}