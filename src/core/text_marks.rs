//! Structs for constructing a group of [`TextMark`]s.
//!
//! [`TextMark`]: struct.TextMark.html

use std::fmt::Debug;

use crate::core::Normal;

/// A group of [`TextMark`]s.
///
/// [`TextMark`]: struct.TextMark.html
#[derive(Debug, Clone)]
pub struct TextMarkGroup {
    /// The group of [`TextMark`]s.
    ///
    /// [`TextMark`]: struct.TextMark.html
    pub group: Vec<TextMark>,
}

impl TextMarkGroup {
    /// Constructs a new `TextMarkGroup` from a vector of [`TextMark`]s.
    ///
    /// [`TextMarkGroup`]: struct.TextMarkGroup.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn new(text_marks: Vec<TextMark>) -> Self {
        Self { group: text_marks }
    }

    /// Returns a new [`TextMarkGroup`] with a single [`TextMark`] in
    /// the center position.
    ///
    /// * `text` - the text to display
    ///
    /// [`TextMarkGroup`]: struct.TextMarkGroup.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn center(text: &str) -> Self {
        let text_marks = vec![TextMark::center(text)];
        Self::new(text_marks)
    }

    /// Returns a new [`TextMarkGroup`] with a [`TextMark`] in
    /// the min (`0.0`) position and max (`1.0`) position.
    ///
    /// * `min_text` - the text to display in the minimum position
    /// * `max_text` - the text to display in the maximum position
    ///
    /// [`TextMarkGroup`]: struct.TextMarkGroup.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn min_max(min_text: &str, max_text: &str) -> Self {
        let text_marks = vec![TextMark::min(min_text), TextMark::max(max_text)];
        Self::new(text_marks)
    }

    /// Returns a new [`TextMarkGroup`] with a [`TextMark`] in
    /// the min (`0.0`), the max (`1.0`), and center (`0.5`) positions.
    ///
    /// * `min_text` - the text to display in the minimum position
    /// * `max_text` - the text to display in the maximum position
    /// * `center_text` - the text to display in the center position
    ///
    /// [`TextMarkGroup`]: struct.TextMarkGroup.html
    /// [`TextMark`]: struct.TextMark.html
    pub fn min_max_and_center(
        min_text: &str,
        max_text: &str,
        center_text: &str,
    ) -> Self {
        let text_marks = vec![
            TextMark::min(min_text),
            TextMark::max(max_text),
            TextMark::center(center_text),
        ];
        Self::new(text_marks)
    }

    /// Creates a group of text marks by subdividing the range
    ///
    /// * `text` - a group of strings to be evenly spaced across the range (not including the minimum and maximum positions)
    /// * `min` - optional text to display at the minimum position
    /// * `max` - optional text to display at the maximum position
    pub fn subdivided(
        text: Vec<&str>,
        min: Option<&str>,
        max: Option<&str>,
    ) -> Self {
        let mut vec: Vec<TextMark> = Vec::new();
        vec.reserve_exact(text.len() + 2);

        let ranges = text.len() + 1;

        let span = 1.0 / ranges as f32;

        for i in 0..ranges {
            let pos = (i as f32 * span) + span;

            vec.push(TextMark {
                position: pos.into(),
                text: String::from(text[i]),
            });
        }

        if let Some(min_text) = min {
            vec.push(TextMark::min(min_text));
        }

        if let Some(max_text) = max {
            vec.push(TextMark::max(max_text));
        }

        Self::new(vec)
    }
}

/// Data of a text mark
#[derive(Debug, Clone)]
pub struct TextMark {
    /// a [`Normal`] value that represents the position of the text
    /// mark. For example, a value of `0.0` is at the minimum position, `1.0` is
    /// at the maximum position, and `0.5` is at the center position. The
    /// default is `0.5`.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub position: Normal,

    /// The text of the mark
    pub text: String,
}

impl Default for TextMark {
    fn default() -> Self {
        TextMark::center("0")
    }
}

impl TextMark {
    /// Returns a new text mark
    ///
    /// * `text` - the text to display
    /// * `position` - a [`Normal`] value that represents the position of the text
    /// mark. For example, a value of `0.0` is at the minimum position, `1.0` is
    /// at the maximum position, and `0.5` is at the center position.
    ///
    /// [`Normal`]: ../struct.
    pub fn new(text: &str, position: Normal) -> Self {
        Self {
            position,
            text: String::from(text),
        }
    }

    /// Returns a new text mark at the center (`0.5`) position
    ///
    /// * `text` - the text to display
    pub fn center(text: &str) -> Self {
        Self {
            position: 0.5.into(),
            text: String::from(text),
        }
    }

    /// Returns a new text mark at the minimum (`0.0`) position
    ///
    /// * `text` - the text to display
    pub fn min(text: &str) -> Self {
        Self {
            position: 0.0.into(),
            text: String::from(text),
        }
    }

    /// Returns a new text mark at the maximum (`1.0`) position
    ///
    /// * `text` - the text to display
    pub fn max(text: &str) -> Self {
        Self {
            position: 1.0.into(),
            text: String::from(text),
        }
    }
}
