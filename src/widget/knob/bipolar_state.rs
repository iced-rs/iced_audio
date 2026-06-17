use std::cmp::Ordering;

use crate::widget::knob::knob_info::KnobInfo;

pub enum BipolarState {
    Left,
    Right,
    Center,
}

impl BipolarState {
    pub fn from_knob_info(knob_info: &KnobInfo) -> Self {
        if let Some(center) = knob_info.bipolar_center {
            match knob_info.value.partial_cmp(&center) {
                Some(Ordering::Less) => BipolarState::Left,
                Some(Ordering::Equal) => BipolarState::Center,
                Some(Ordering::Greater) => BipolarState::Right,
                None => BipolarState::Center,
            }
        } else if knob_info.value.as_f32() < 0.499 {
            BipolarState::Left
        } else if knob_info.value.as_f32() > 0.501 {
            BipolarState::Right
        } else {
            BipolarState::Center
        }
    }
}
