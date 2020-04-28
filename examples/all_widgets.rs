extern crate iced;

use iced::{
    scrollable, Align, Column, Container, Element, Length, Row, Sandbox,
    Settings, Scrollable, Space,
};

use iced_audio::{h_slider, HSlider};

pub fn main() {
    AllWidgets::run(Settings::default());
}

#[derive(Default)]
struct AllWidgets {
    h_slider: h_slider::State,
    h_slider_value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    HSliderChanged(f32),
}

impl Sandbox for AllWidgets {
    type Message = Message;

    fn new() -> Self {
        AllWidgets::default()
    }

    fn title(&self) -> String {
        String::from("All Widgets - Iced Audio")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HSliderChanged(value) => {
                self.h_slider_value = value;
                println!("h_slider value: {}", value);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let h_slider = HSlider::new(
            &mut self.h_slider,
            self.h_slider_value,
            Message::HSliderChanged,
        );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(h_slider);
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}