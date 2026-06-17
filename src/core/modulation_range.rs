//! The state of a modulation range

use super::normal::Normal;

/// The state of a modulation range
#[derive(Debug, Clone)]
pub struct ModulationRange {
    /// Where the modulation range starts.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub start: Normal,
    /// Where the modulation range ends.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub end: Normal,
    /// Whether the filled portion of the modulation range is visible or not, while keeping
    /// the empty portion visible.
    pub filled_visible: bool,
}

impl ModulationRange {
    /// Creates a new `ModulationRange`
    ///
    /// * start - Where the modulation range starts.
    ///   `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    /// * ends - Where the modulation range ends.
    ///   `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub const fn new(start: Normal, end: Normal) -> Self {
        Self {
            start,
            end,
            filled_visible: true,
        }
    }
}

impl Default for ModulationRange {
    fn default() -> Self {
        Self {
            start: Normal::MIN,
            end: Normal::MIN,
            filled_visible: true,
        }
    }
}
