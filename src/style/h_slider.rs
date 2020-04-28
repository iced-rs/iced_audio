
use iced::Color;

/// The appearance of an HSlider.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub rail_colors: (Color, Color),
    pub handle: Handle,
}

/// The appearance of the handle of an HSlider.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    pub color: Color,
    pub width: u16,
    pub height: u16,
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: Color,
}

/// A set of rules that dictate the style of an HSlider.
pub trait StyleSheet {
    /// Produces the style of an active HSlider.
    fn active(&self) -> Style;

    /// Produces the style of a hovered HSlider.
    fn hovered(&self) -> Style;

    /// Produces the style of an HSlider that is being dragged.
    fn dragging(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            rail_colors: ([0.6, 0.6, 0.6, 0.5].into(), Color::WHITE),
            handle: Handle {
                color: Color::from_rgb(0.95, 0.95, 0.95),
                width: 25,
                height: 14,
                border_radius: 2,
                border_color: Color::from_rgb(0.6, 0.6, 0.6),
                border_width: 1,
            },
        }
    }

    fn hovered(&self) -> Style {
        let active = self.active();

        Style {
            handle: Handle {
                color: Color::from_rgb(0.90, 0.90, 0.90),
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> Style {
        let active = self.active();

        Style {
            handle: Handle {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..active.handle
            },
            ..active
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}