//! A DSP [`Detector`] that calculates the peak and rms levels of a stereo signal
///
/// [`Detector`]: ../db_meter/trait.Detector.html
use crate::native::db_meter::{Detector, DetectorOutput};

static RMS_WINDOW_SIZE_SEC: f32 = 0.3;
static RMS_BLOCK_SIZE: usize = 512;

#[allow(missing_debug_implementations)]
struct RingBufCache {
    pub prod: ringbuf::Producer<f32>,
    pub cons: ringbuf::Consumer<f32>,
    pub smps_not_cached: usize,
}

/// A DSP [`Detector`] that calculates the peak and rms levels of a stereo signal
///
/// This algorithm caches blocks of 512 samples for effeciency. The RMS window size
/// will always be a multiple of 512, so it may be a bit off the target of 300ms. For a
/// basic decibel meter, this slight innacurracy is worth the better performance.
///
/// [`Detector`]: ../db_meter/trait.Detector.html
#[allow(missing_debug_implementations)]
pub struct PeakRmsDetector {
    rms_window_size: usize,
    one_over_rms_window_size: f32,
    rms_block_size: usize,
    sample_rate: f32,
    left_rms_cache: Option<RingBufCache>,
    right_rms_cache: Option<RingBufCache>,
}

impl PeakRmsDetector {
    /// Creates a new `PeakRmsDetector`
    pub fn new() -> Self {
        Self {
            rms_window_size: 0,
            one_over_rms_window_size: 0.0,
            sample_rate: 0.0,
            rms_block_size: 0,
            left_rms_cache: None,
            right_rms_cache: None,
        }
    }

    fn peak_db(s1: &[f32], s2: &[f32], n_smps_not_cached: usize) -> f32 {
        let n_new_smps = s1.len() + s2.len() - n_smps_not_cached;
        let mut max_peak: f32 = 0.0;

        // all new samples are in s2
        if s2.len() >= n_new_smps {
            let s2 = &s2[(s2.len() - n_new_smps)..];
            for smp in s2.iter() {
                let abs_smp = smp.abs();
                if abs_smp > max_peak {
                    max_peak = abs_smp;
                }
            }
        }
        // all new samples are in s1
        else if s2.len() == 0 {
            let s1 = &s1[(s1.len() - n_new_smps)..];
            for smp in s1.iter() {
                let abs_smp = smp.abs();
                if abs_smp > max_peak {
                    max_peak = abs_smp;
                }
            }
        }
        // all new samples are in both
        else {
            let s1_start = s1.len() - (n_new_smps - s2.len());
            let s1 = &s1[s1_start..];
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
        }

        // convert to decibels
        crate::core::math::amplitude_to_db_f32(max_peak)
    }

    fn rms_db(
        s1: &[f32],
        s2: &[f32],
        cache_rb_prod: &mut ringbuf::Producer<f32>,
        cache_rb_cons: &mut ringbuf::Consumer<f32>,
        one_over_rms_window_size: f32,
    ) -> (Option<f32>, usize, usize) {
        let mut rms_db: Option<f32> = None;
        let mut smps_not_cached = s1.len() + s2.len();
        let mut smps_to_discard: usize = 0;

        // calculate and cache new blocks
        if smps_not_cached >= RMS_BLOCK_SIZE {
            let mut s1 = &s1[..];
            let mut s2 = &s2[..];

            while smps_not_cached >= RMS_BLOCK_SIZE {
                let mut sum: f32 = 0.0;

                // all uncached samples are in s2
                if s2.len() >= RMS_BLOCK_SIZE {
                    let s2_part_start = s2.len() - RMS_BLOCK_SIZE;
                    let s2_part = &s2[s2_part_start..];
                    for smp in s2_part.iter() {
                        sum += (*smp) * (*smp);
                    }
                    s2 = &s2[..s2_part_start];
                }
                // all uncached samples are in s1
                else if s2.len() == 0 {
                    let s1_part_start = s1.len() - RMS_BLOCK_SIZE;
                    let s1_part = &s1[s1_part_start..];
                    for smp in s1_part.iter() {
                        sum += (*smp) * (*smp);
                    }
                    s1 = &s1[..s1_part_start];
                }
                // all uncached samples are in both
                else {
                    let s1_part_start = s1.len() - (RMS_BLOCK_SIZE - s2.len());
                    let s1_part = &s1[s1_part_start..];
                    for smp in s1_part.iter() {
                        sum += (*smp) * (*smp);
                    }
                    for smp in s2.iter() {
                        sum += (*smp) * (*smp);
                    }
                    s2 = &s2[0..0];
                    s1 = &s1[..s1_part_start];
                }

                if cache_rb_cons.is_full() {
                    let _ = cache_rb_cons.discard(1);
                }
                cache_rb_prod.push(sum).unwrap();

                smps_not_cached -= RMS_BLOCK_SIZE;
                smps_to_discard += RMS_BLOCK_SIZE;
            }

            if cache_rb_cons.is_full() {
                let mut sum = 0.0;
                cache_rb_cons.for_each(|cached_sum: &f32| {
                    sum += *cached_sum;
                });

                rms_db = Some(crate::core::math::amplitude_to_db_f32(
                    (sum * one_over_rms_window_size).sqrt(),
                ));
            }
        }

        (rms_db, smps_not_cached, smps_to_discard)
    }
}

impl Default for PeakRmsDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl Detector for PeakRmsDetector {
    fn update_sample_rate(&mut self, sample_rate: f32) {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;

            let rms_window_size = (RMS_WINDOW_SIZE_SEC * sample_rate).round();
            self.rms_block_size =
                (rms_window_size / RMS_BLOCK_SIZE as f32).round() as usize;
            self.rms_window_size = self.rms_block_size * RMS_BLOCK_SIZE;
            self.one_over_rms_window_size = 1.0 / self.rms_window_size as f32;

            let left_rms_cache_rb =
                ringbuf::RingBuffer::<f32>::new(self.rms_block_size);
            let (left_rms_cache_prod, left_rms_cache_cons) =
                left_rms_cache_rb.split();

            let right_rms_cache_rb =
                ringbuf::RingBuffer::<f32>::new(self.rms_block_size);
            let (right_rms_cache_prod, right_rms_cache_cons) =
                right_rms_cache_rb.split();

            self.left_rms_cache = Some(RingBufCache {
                prod: left_rms_cache_prod,
                cons: left_rms_cache_cons,
                smps_not_cached: 0,
            });

            self.right_rms_cache = Some(RingBufCache {
                prod: right_rms_cache_prod,
                cons: right_rms_cache_cons,
                smps_not_cached: 0,
            });
        }
    }

    fn process_left(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput {
        let mut peak_db: Option<f32> = None;
        let mut rms_db: Option<f32> = None;
        let mut n_samples_to_discard: usize = 0;

        if let Some(left_rms_cache) = &mut self.left_rms_cache {
            if s1.len() + s2.len() > 0 {
                // calculate peak
                peak_db =
                    Some(Self::peak_db(s1, s2, left_rms_cache.smps_not_cached));

                // calculate RMS
                let (rms, not_cached, discard) = Self::rms_db(
                    s1,
                    s2,
                    &mut left_rms_cache.prod,
                    &mut left_rms_cache.cons,
                    self.one_over_rms_window_size,
                );
                rms_db = rms;
                left_rms_cache.smps_not_cached = not_cached;
                n_samples_to_discard = discard;
            }
        }

        DetectorOutput {
            peak_db,
            bar_db: rms_db,
            n_samples_to_discard,
        }
    }

    fn process_right(&mut self, s1: &[f32], s2: &[f32]) -> DetectorOutput {
        let mut peak_db: Option<f32> = None;
        let mut rms_db: Option<f32> = None;
        let mut n_samples_to_discard: usize = 0;

        if let Some(right_rms_cache) = &mut self.right_rms_cache {
            if s1.len() + s2.len() > 0 {
                // calculate peak
                peak_db = Some(Self::peak_db(
                    s1,
                    s2,
                    right_rms_cache.smps_not_cached,
                ));

                // calculate RMS
                let (rms, not_cached, discard) = Self::rms_db(
                    s1,
                    s2,
                    &mut right_rms_cache.prod,
                    &mut right_rms_cache.cons,
                    self.one_over_rms_window_size,
                );
                rms_db = rms;
                right_rms_cache.smps_not_cached = not_cached;
                n_samples_to_discard = discard;
            }
        }

        DetectorOutput {
            peak_db,
            bar_db: rms_db,
            n_samples_to_discard,
        }
    }

    fn clear(&mut self) {
        if let Some(left_rms_cache) = &mut self.left_rms_cache {
            left_rms_cache.smps_not_cached = 0;
            let _ = left_rms_cache.cons.discard(left_rms_cache.cons.capacity());
        }

        if let Some(right_rms_cache) = &mut self.right_rms_cache {
            right_rms_cache.smps_not_cached = 0;
            let _ = right_rms_cache
                .cons
                .discard(right_rms_cache.cons.capacity());
        }
    }
}
