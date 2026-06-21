#![allow(unused)]

use iced_audio::{DBRange, FloatRange, FreqRange, IntRange, Normal};

pub fn info_text_f32<ID: std::fmt::Debug>(id: ID, normal: Normal, range: &FloatRange) -> String {
    let value = range.unmap_to_value(normal);
    format!("id: {id:?}  |  value: {value:.3}")
}

pub fn info_text_i32<ID: std::fmt::Debug>(id: ID, normal: Normal, range: &IntRange) -> String {
    let value = range.unmap_to_value(normal);
    format!("id: {id:?}  |  value: {value}")
}

pub fn info_text_db<ID: std::fmt::Debug>(id: ID, normal: Normal, range: &DBRange) -> String {
    let value = range.unmap_to_db(normal);
    format!("id: {id:?}  |  value: {value:.3} dB")
}

pub fn info_text_freq<ID: std::fmt::Debug>(id: ID, normal: Normal, range: &FreqRange) -> String {
    let value = range.unmap_to_freq(normal);
    if value < 1000.0 {
        format!("id: {id:?}  |  value: {value:.2} Hz")
    } else {
        format!("id: {id:?}  |  value: {:.2} kHz", value / 1000.0)
    }
}
