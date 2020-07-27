//! A DSP [`Detector`] that calculates the peak levels of a stereo signal
///
/// [`Detector`]: ../db_meter/trait.Detector.html
use crate::native::db_meter::{Detector, DetectorOutput};

/// A DSP [`Detector`] that calculates the peak levels of a stereo signal
///
/// [`Detector`]: ../db_meter/trait.Detector.html
#[allow(missing_debug_implementations)]
#[derive(Default, Copy, Clone)]
pub struct PeakDetector;

impl PeakDetector {
    /// Creates a new `PeakDetector`
    pub fn new() -> Self {
        Self {}
    }

    fn peak_db(s1: &[f32], s2: &[f32]) -> f32 {
        let mut max_peak: f32 = 0.0;

        for smp in s1.iter() {
            let abs_smp = smp.abs();
            if abs_smp > max_peak {
                max_peak = abs_smp;
            }
        }

        for smp in s2.iter() {
            let abs_smp = smp.abs();
            if abs_smp > max_peak {
                max_peak = abs_smp;
            }
        }

        crate::core::math::amplitude_to_db_f32(max_peak)
    }
}

impl Detector for PeakDetector {
    fn update_sample_rate(&mut self, _sample_rate: f32) {}

    fn process_left(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput {
        let total_len = s1.len() + s2.len();

        let peak_db = if total_len > 0 {
            Some(Self::peak_db(s1, s2))
        } else {
            None
        };

        DetectorOutput {
            peak_db,
            bar_db: peak_db,
            n_samples_to_discard: total_len,
        }
    }

    fn process_right(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput {
        let total_len = s1.len() + s2.len();

        let peak_db = if total_len > 0 {
            Some(Self::peak_db(s1, s2))
        } else {
            None
        };

        DetectorOutput {
            peak_db,
            bar_db: peak_db,
            n_samples_to_discard: total_len,
        }
    }

    fn clear(&mut self) {}
}
