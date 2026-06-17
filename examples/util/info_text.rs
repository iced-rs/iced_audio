#![allow(unused)]

pub fn info_text_f32<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {id:?}  |  value: {value:.3}")
}

pub fn info_text_i32<ID: std::fmt::Debug>(id: ID, value: i32) -> String {
    format!("id: {id:?}  |  value: {value}")
}

pub fn info_text_db<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {id:?}  |  value: {value:.3} dB")
}

pub fn info_text_freq<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    if value < 1000.0 {
        format!("id: {id:?}  |  value: {value:.2} Hz")
    } else {
        format!("id: {id:?}  |  value: {:.2} kHz", value / 1000.0)
    }
}
