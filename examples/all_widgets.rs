extern crate iced;

use iced::{
    Column, Container, Element, Length, Sandbox, Text, Settings, Row, Color
};

use iced_audio::{Normal, FloatParam, IntParam, LogDBParam,
    OctaveParam, h_slider, HSlider
};

use iced_native::image;

pub fn main() {
    // run the application
    AllWidgets::run(Settings::default());
}

// the application
struct AllWidgets {
    h_slider_float_param: FloatParam,
    h_slider_float_state: h_slider::State,
    h_slider_float_label: String,

    h_slider_int_param: IntParam,
    h_slider_int_state: h_slider::State,
    h_slider_int_label: String,

    h_slider_log_param: LogDBParam,
    h_slider_log_state: h_slider::State,
    h_slider_log_label: String,

    h_slider_oct_param: OctaveParam,
    h_slider_oct_state: h_slider::State,
    h_slider_oct_label: String,

    h_slider_style_param: FloatParam,
    h_slider_style_state: h_slider::State,
    h_slider_style_label: String,

    h_slider_style_bp_param: FloatParam,
    h_slider_style_bp_state: h_slider::State,
    h_slider_style_bp_label: String,

    h_slider_texture_param: FloatParam,
    h_slider_texture_state: h_slider::State,
    h_slider_texture_label: String,

    h_slider_texture_handle: image::Handle,

    output_text: String,
}

impl Default for AllWidgets {
    fn default() -> Self {

        // initalize parameters

        let h_slider_float_param =
            FloatParam::new(0, -1.0, 1.0,
                              0.0, 0.0);

        let h_slider_int_param =
            IntParam::new(1, 0, 5,
                              0, 2);

        let h_slider_log_param =
            LogDBParam::new(2, -12.0, 12.0,
                              0.0, 0.0,
                              0.5.into());

        let h_slider_oct_param =
            OctaveParam::new(3, 20.0, 20_480.0,
                              1000.0, 1000.0);

        let h_slider_style_param =
            FloatParam::new(4, 0.0, 1.0,
                              0.0, 0.0);

        let h_slider_style_bp_param =
            FloatParam::new(5, -1.0, 1.0,
                              0.0, 0.0);

        let h_slider_texture_param =
            FloatParam::new(6, 0.0, 1.0,
                              0.0, 0.0);
        
        // create application
        
        Self {
            // add the parameter
            h_slider_float_param,
            // initialize the state of the HSlider widget
            h_slider_float_state: h_slider::State::new(
                &h_slider_float_param
            ),
            // initialize the label above the HSlider widget
            h_slider_float_label: String::from("Float Range"),


            h_slider_int_param,
            h_slider_int_state: h_slider::State::new(
                &h_slider_int_param
            ),
            h_slider_int_label: String::from("Int Range"),
            

            h_slider_log_param,
            h_slider_log_state: h_slider::State::new(
                &h_slider_log_param
            ),
            h_slider_log_label: String::from("Log dB Range"),
            

            h_slider_oct_param,
            h_slider_oct_state: h_slider::State::new(
                &h_slider_oct_param
            ),
            h_slider_oct_label: String::from("Octave Freq Range"),


            h_slider_style_param,
            h_slider_style_state: h_slider::State::new(
                &h_slider_style_param
            ),
            h_slider_style_label: String::from("Rect Style"),
            

            h_slider_style_bp_param,
            h_slider_style_bp_state: h_slider::State::new(
                &h_slider_style_bp_param
            ),
            h_slider_style_bp_label: String::from("Rect Bipolar Style"),


            h_slider_texture_param,
            h_slider_texture_state: h_slider::State::new(
                &h_slider_texture_param
            ),
            h_slider_texture_label: String::from("Texture Style"),

            h_slider_texture_handle: format!(
                "{}/examples/images/iced_h_slider.png",
                env!("CARGO_MANIFEST_DIR")
            ).into(),


            output_text: String::from("Move a widget"),
        }
    }
}


// generates the text for an output

fn info_text_f32(id: u32, value: f32) -> String {
    format!("ID {}  |  {:.3}", id, value)
}

fn info_text_i32(id: u32, value: i32) -> String {
    format!("ID {}  |  {}", id, value)
}

fn info_text_db(id: u32, value: f32) -> String {
    format!("ID {}  |  {:.3} dB", id, value)
}

fn info_text_octave(id: u32, value: f32) -> String {
    if value < 1000.0 {
        format!("ID {}  |  {:.2} Hz", id, value)
    } else {
        format!("ID {}  |  {:.2} kHz", id, value / 1000.0)
    }
}

#[derive(Debug, Clone)]
enum Message {
    // the message when an HSlider has changed
    HSliderChanged((u32, Normal)),
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
            Message::HSliderChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // HSlider widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    0 => {
                        self.h_slider_float_param.set_from_normal(normal);
                        self.output_text = info_text_f32(id,
                            self.h_slider_float_param.value());
                    },
                    1 => {
                        self.h_slider_int_param.set_from_normal(normal);
                        self.output_text = info_text_i32(id,
                            self.h_slider_int_param.value());
                    },
                    2 => {
                        self.h_slider_log_param.set_from_normal(normal);
                        self.output_text = info_text_db(id,
                            self.h_slider_log_param.value());
                    },
                    3 => {
                        self.h_slider_oct_param.set_from_normal(normal);
                        self.output_text = info_text_octave(id,
                            self.h_slider_oct_param.value());
                    },
                    4 => {
                        self.h_slider_style_param.set_from_normal(normal);
                        self.output_text = info_text_f32(id,
                            self.h_slider_style_param.value());
                    },
                    5 => {
                        self.h_slider_style_bp_param.set_from_normal(normal);
                        self.output_text = info_text_f32(id,
                            self.h_slider_style_bp_param.value());
                    },
                    6 => {
                        self.h_slider_texture_param.set_from_normal(normal);
                        self.output_text = info_text_f32(id,
                            self.h_slider_texture_param.value());
                    },
                    _ => (),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // create each of the HSlider widgets, passing in the value of
        // the corresponding parameter

        let h_slider_float = HSlider::new(
            &mut self.h_slider_float_state,
            &self.h_slider_float_param,
            Message::HSliderChanged,
        );

        let h_slider_int = HSlider::new(
            &mut self.h_slider_int_state,
            &self.h_slider_int_param,
            Message::HSliderChanged,
        );

        let h_slider_log = HSlider::new(
            &mut self.h_slider_log_state,
            &self.h_slider_log_param,
            Message::HSliderChanged,
        );

        let h_slider_oct = HSlider::new(
            &mut self.h_slider_oct_state,
            &self.h_slider_oct_param,
            Message::HSliderChanged,
        );

        let h_slider_style = HSlider::new(
            &mut self.h_slider_style_state,
            &self.h_slider_style_param,
            Message::HSliderChanged,
        )
        .style(HSliderCustomStyle);

        let h_slider_style_bp = HSlider::new(
            &mut self.h_slider_style_bp_state,
            &self.h_slider_style_bp_param,
            Message::HSliderChanged,
        )
        .style(HSliderCustomStyleBipolar);

        let h_slider_texture = HSlider::new(
            &mut self.h_slider_texture_state,
            &self.h_slider_texture_param,
            Message::HSliderChanged,
        )
        // clone the handle to the loaded texture
        .style(HSliderTextureStyle(self.h_slider_texture_handle.clone()));


        // push the sliders into rows

        let h_slider_row = Row::new()
            .spacing(20)

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_float_label))
                .push(h_slider_float)

                .push(Text::new(&self.h_slider_log_label))
                .push(h_slider_log)

                .push(Text::new(&self.h_slider_style_label))
                .push(h_slider_style)

                .push(Text::new(&self.h_slider_texture_label))
                .push(h_slider_texture)
            )

            .push(Column::new()
                .width(Length::FillPortion(1))
                .spacing(10)
                .push(Text::new(&self.h_slider_int_label))
                .push(h_slider_int)

                .push(Text::new(&self.h_slider_oct_label))
                .push(h_slider_oct)

                .push(Text::new(&self.h_slider_style_bp_label))
                .push(h_slider_style_bp)
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Text::new("Horizontal Sliders (HSlider)").size(42))
            .push(Text::new("Hold down Ctrl for fine adjustments. \
                Double-click to reset to the default value.").size(16))
            .push(h_slider_row)
            .push(Text::new(&self.output_text).size(16));
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

const EMPTY_COLOR: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x46 as f32 / 255.0,
    0x4D as f32 / 255.0,
);
const BORDER_COLOR: Color = Color::from_rgb(
    0x3C as f32 / 255.0,
    0x3F as f32 / 255.0,
    0x48 as f32 / 255.0,
);
const FILLED_COLOR: Color = Color::from_rgb(
    0x29 as f32 / 255.0,
    0x66 as f32 / 255.0,
    0xA3 as f32 / 255.0,
);
const FILLED_HOVER_COLOR: Color = Color::from_rgb(
    0x33 as f32 / 255.0,
    0x70 as f32 / 255.0,
    0xAD as f32 / 255.0,
);
const HANDLE_COLOR: Color = Color::from_rgb(
    0x75 as f32 / 255.0,
    0xC2 as f32 / 255.0,
    0xFF as f32 / 255.0,
);


// Custom style for the Rect HSlider

struct HSliderCustomStyle;
impl h_slider::StyleSheet for HSliderCustomStyle {
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

// Custom style for the Rect Bipolar HSlider

struct HSliderCustomStyleBipolar;
impl h_slider::StyleSheet for HSliderCustomStyleBipolar {
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

// Custom style for the Texture HSlider

struct HSliderTextureStyle(image::Handle);
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