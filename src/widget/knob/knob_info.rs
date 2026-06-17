use crate::Normal;
use iced::Rectangle;

pub struct KnobInfo {
    pub bounds: Rectangle,
    pub start_angle: f32,
    pub angle_span: f32,
    pub radius: f32,
    pub value: Normal,
    pub bipolar_center: Option<Normal>,
    pub value_angle: f32,
}
