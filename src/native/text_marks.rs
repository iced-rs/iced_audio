//! Structs for constructing a group of text marks.

use std::fmt::Debug;

use crate::core::Normal;

/// A group of text marks.
#[derive(Debug, Clone)]
pub struct Group {
    /// The group of text marks.
    pub group: Vec<(Normal, String)>,
    hashed: u64,
}

impl Group {
    /// Constructs a new `Group` from a vector of [`TextMark`]s.
    ///
    /// [`Group`]: struct.Group.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn from_normalized(text_marks: &[(Normal, &str)]) -> Self {
        let mut group: Vec<(Normal, String)> =
            Vec::with_capacity(text_marks.len());
        for text_mark in text_marks {
            group.push((text_mark.0, String::from(text_mark.1)));
        }

        Self::from_string(group)
    }

    /// Constructs a new `Group` from a vector of [`TextMark`]s.
    ///
    /// [`Group`]: struct.Group.html
    /// [`TextMark`]: struct.TextMark.html
    fn from_string(group: Vec<(Normal, String)>) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = iced_native::Hasher::default();
        group.len().hash(&mut hasher);

        for text_mark in &group {
            text_mark.1.hash(&mut hasher);
            // Rust can't hash an f32 value.
            ((text_mark.0.as_f32() * 10000000.0) as u64).hash(&mut hasher);
        }

        Self {
            group,
            hashed: hasher.finish(),
        }
    }

    /// Returns a new [`Group`] with a single [`TextMark`] in
    /// the center position.
    ///
    /// * `text` - the text to display
    ///
    /// [`Group`]: struct.Group.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn center(text: &str) -> Self {
        vec![(Normal::center(), String::from(text))].into()
    }

    /// Returns a new [`Group`] with a [`TextMark`] in
    /// the min (`0.0`) position and max (`1.0`) position.
    ///
    /// * `min_text` - the text to display in the minimum position
    /// * `max_text` - the text to display in the maximum position
    ///
    /// [`Group`]: struct.Group.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn min_max(min_text: &str, max_text: &str) -> Self {
        vec![
            (Normal::min(), String::from(min_text)),
            (Normal::max(), String::from(max_text)),
        ]
        .into()
    }

    /// Returns a new [`Group`] with a [`TextMark`] in
    /// the min (`0.0`), the max (`1.0`), and center (`0.5`) positions.
    ///
    /// * `min_text` - the text to display in the minimum position
    /// * `max_text` - the text to display in the maximum position
    /// * `center_text` - the text to display in the center position
    ///
    /// [`Group`]: struct.Group.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn min_max_and_center(
        min_text: &str,
        max_text: &str,
        center_text: &str,
    ) -> Self {
        vec![
            (Normal::min(), String::from(min_text)),
            (Normal::center(), String::from(center_text)),
            (Normal::max(), String::from(max_text)),
        ]
        .into()
    }

    /// Creates a group of text marks by subdividing the range
    ///
    /// * `text` - a group of strings to be evenly spaced across the range (not including the minimum and maximum positions)
    /// * `min` - optional text to display at the minimum position
    /// * `max` - optional text to display at the maximum position
    pub fn subdivided(
        text: &[&str],
        min: Option<&str>,
        max: Option<&str>,
    ) -> Self {
        let mut vec: Vec<(Normal, String)> = Vec::with_capacity(text.len() + 2);

        let span = 1.0 / (text.len() + 1) as f32;

        for (i, text) in text.iter().enumerate() {
            let pos = (i as f32 * span) + span;

            vec.push((pos.into(), String::from(*text)));
        }

        if let Some(min_text) = min {
            vec.push((Normal::min(), String::from(min_text)));
        }

        if let Some(max_text) = max {
            vec.push((Normal::max(), String::from(max_text)));
        }

        vec.into()
    }

    /// Creates a group of evenly spaced text marks
    ///
    /// * `text` - a group of strings to be displayed
    pub fn evenly_spaced(text: &[&str]) -> Self {
        let mut vec: Vec<(Normal, String)> = Vec::with_capacity(text.len());

        if text.len() == 1 {
            vec.push((Normal::min(), String::from(text[0])));
        } else if text.len() != 0 {
            let len_min_1 = text.len() - 1;
            let span = 1.0 / len_min_1 as f32;

            for i in 0..len_min_1 {
                let pos = i as f32 * span;

                vec.push((pos.into(), String::from(text[i])));
            }

            vec.push((Normal::max(), String::from(text[len_min_1])));
        }

        vec.into()
    }

    /// Returns the hashed value of the internal data.
    pub(crate) fn hashed(&self) -> u64 {
        self.hashed
    }
}

impl From<&[(Normal, &str)]> for Group {
    fn from(slice: &[(Normal, &str)]) -> Self {
        Self::from_normalized(slice)
    }
}

impl From<&[(Normal, String)]> for Group {
    fn from(slice: &[(Normal, String)]) -> Self {
        slice.to_vec().into()
    }
}

impl From<Vec<(Normal, &str)>> for Group {
    fn from(vec: Vec<(Normal, &str)>) -> Self {
        Self::from_normalized(&vec)
    }
}

impl From<Vec<(Normal, String)>> for Group {
    fn from(vec: Vec<(Normal, String)>) -> Self {
        Self::from_string(vec)
    }
}
