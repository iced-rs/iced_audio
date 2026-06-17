//! Colors for the default styles

use iced::Color;

pub const BORDER: Color = Color::from_rgb(0.315, 0.315, 0.315);
pub const LIGHT_BACK: Color = Color::from_rgb(0.97, 0.97, 0.97);
pub const LIGHT_BACK_HOVER: Color = Color::from_rgb(0.93, 0.93, 0.93);
pub const LIGHT_BACK_DRAG: Color = Color::from_rgb(0.92, 0.92, 0.92);

pub const SLIDER_RAIL: (Color, Color) = (
    Color {
        r: 0.26,
        g: 0.26,
        b: 0.26,
        a: 0.75,
    },
    Color {
        r: 0.56,
        g: 0.56,
        b: 0.56,
        a: 0.75,
    },
);

pub const TICK_TIER_1: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.93,
};
pub const TICK_TIER_2: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.83,
};
pub const TICK_TIER_3: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.65,
};

pub const TEXT_MARK: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.65,
};

pub const KNOB_BACK_HOVER: Color = Color::from_rgb(0.96, 0.96, 0.96);

pub const RAMP_BACK_HOVER: Color = Color::from_rgb(0.95, 0.95, 0.95);

pub const XY_PAD_RAIL: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.9,
};
pub const XY_PAD_CENTER_LINE: Color = Color {
    r: 0.56,
    g: 0.56,
    b: 0.56,
    a: 0.5,
};

/*
pub const DB_METER_BACK: Color = Color::from_rgb(0.45, 0.45, 0.45);
pub const DB_METER_BORDER: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const DB_METER_LOW: Color = Color::from_rgb(0.435, 0.886, 0.11);
pub const DB_METER_MED: Color = Color::from_rgb(0.737, 1.0, 0.145);
pub const DB_METER_HIGH: Color = Color::from_rgb(1.0, 0.945, 0.0);
pub const DB_METER_CLIP: Color = Color::from_rgb(1.0, 0.071, 0.071);
pub const DB_METER_CLIP_MARKER: Color = Color {
    r: 0.78,
    g: 0.78,
    b: 0.78,
    a: 0.28,
};
pub const DB_METER_GAP: Color = Color::from_rgb(0.25, 0.25, 0.25);
pub const PHASE_METER_CENTER_LINE: Color = Color::from_rgb(0.92, 0.92, 0.92);
*/
