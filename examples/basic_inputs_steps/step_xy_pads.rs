use iced::{
    Column, Element, Length, Text, Row
};

use iced_audio::{Normal, FloatParam, xy_pad, XYPad};

use crate::{Step, style};

/// Unique identifier for each parameter. An XYPad widget needs two parameters
/// for the x and y coordinate. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum XYPadsID {
    DefaultX,
    DefaultY,
    CustomX,
    CustomY,
}

#[derive(Debug, Clone)]
pub enum Message {
    XYPadsChanged((XYPadsID, Normal)),
}

pub struct XYPadStep {
    default_x_param: FloatParam<XYPadsID>,
    default_y_param: FloatParam<XYPadsID>,
    xy_pad_default_state: xy_pad::State,
    xy_pad_default_label: String,

    custom_x_param: FloatParam<XYPadsID>,
    custom_y_param: FloatParam<XYPadsID>,
    xy_pad_custom_state: xy_pad::State,
    xy_pad_custom_label: String,

    output_text_x: String,
    output_text_y: String,
}

impl Default for XYPadStep {
    fn default() -> Self {
        // initalize parameters

        let default_x_param = FloatParam::<XYPadsID>::new(
            XYPadsID::DefaultX, -1.0, 1.0, 0.0, 0.0);
        let default_y_param = FloatParam::<XYPadsID>::new(
            XYPadsID::DefaultY, -1.0, 1.0, 0.0, 0.0);

        let custom_x_param = FloatParam::<XYPadsID>::new(
            XYPadsID::CustomX, -1.0, 1.0, 0.0, 0.0);
        let custom_y_param = FloatParam::<XYPadsID>::new(
            XYPadsID::CustomY, -1.0, 1.0, 0.0, 0.0);
        
        Self {
            // add the parameters
            default_x_param,
            default_y_param,
            // initialize the state of the XYPad widget
            xy_pad_default_state: xy_pad::State::new(
                &default_x_param, &default_y_param
            ),
            
            // initialize the label above the XYPad widget
            xy_pad_default_label: String::from("Default Style"),
            

            custom_x_param,
            custom_y_param,
            xy_pad_custom_state: xy_pad::State::new(
                &custom_x_param, &custom_y_param
            ),
            xy_pad_custom_label: String::from("Custom Style"),


            output_text_x: String::from("Move a widget"),
            output_text_y: String::from("Move a widget"),
        }
    }
}

impl XYPadStep {
    pub fn title(&self) -> &str {
        "XY Pads"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::XYPadsChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // XYPad widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    XYPadsID::DefaultX => {
                        self.default_x_param.set_from_normal(normal);
                        self.output_text_x = crate::info_text_f32(id,
                            self.default_x_param.value());
                    },
                    XYPadsID::DefaultY => {
                        self.default_y_param.set_from_normal(normal);
                        self.output_text_y = crate::info_text_f32(id,
                            self.default_y_param.value());
                    },

                    XYPadsID::CustomX => {
                        self.custom_x_param.set_from_normal(normal);
                        self.output_text_x = crate::info_text_f32(id,
                            self.custom_x_param.value());
                    },
                    XYPadsID::CustomY => {
                        self.custom_y_param.set_from_normal(normal);
                        self.output_text_y = crate::info_text_f32(id,
                            self.custom_y_param.value());
                    },
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the XYPad widgets, passing in the value of
        // the corresponding parameter

        let xy_pad_default = XYPad::new(
            &mut self.xy_pad_default_state,
            &self.default_x_param,
            &self.default_y_param,
            Message::XYPadsChanged,
        );

        let xy_pad_custom = XYPad::new(
            &mut self.xy_pad_custom_state,
            &self.custom_x_param,
            &self.custom_y_param,
            Message::XYPadsChanged,
        )
        .style(style::XYPadCustomStyle);

        // push the widgets into rows

        let xy_pad_row = Row::new()
            .spacing(20)

            .push(Column::new()
                .max_height(300)
                .width(Length::Fill)
                .push(Text::new(&self.xy_pad_default_label))
                .push(xy_pad_default)
            )
            .push(Column::new()
                .max_height(300)
                .width(Length::Fill)
                .push(Text::new(&self.xy_pad_custom_label))
                .push(xy_pad_custom)
            );
        
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(xy_pad_row)
            .push(Text::new(&self.output_text_x).size(16))
            .push(Text::new(&self.output_text_y).size(16));
        
        Step::container("XY Pads (XYPad)").push(content).into()
    }
}