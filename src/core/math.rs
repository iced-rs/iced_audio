//! Math helper functions

/// pi / 180.0
pub static PI_OVER_180: f32 = std::f32::consts::PI / 180.0;
/// 2.0 * pi
pub static TWO_PI: f32 = std::f32::consts::PI * 2.0;
/// pi * (3.0 / 2.0)
pub static THREE_HALVES_PI: f32 = std::f32::consts::PI * 2.0;

static ONE_OVER_20_F32: f32 = 1.0 / 20.0;
static ONE_OVER_20_F64: f64 = 1.0 / 20.0;

/// Converts decibels to amplitude
#[inline]
pub fn db_to_amplitdue_f32(db: f32) -> f32 {
    10.0f32.powf(db * ONE_OVER_20_F32)
}
/// Converts decibels to amplitude
#[inline]
pub fn db_to_amplitdue_f64(db: f64) -> f64 {
    10.0f64.powf(db * ONE_OVER_20_F64)
}

/// Converts amplitude to decibels
#[inline]
pub fn amplitude_to_db_f32(amp: f32) -> f32 {
    20.0f32 * amp.log10()
}
/// Converts amplitude to decibels
#[inline]
pub fn amplitdue_to_db_f64(amp: f64) -> f64 {
    20.0f64 * amp.log10()
}
