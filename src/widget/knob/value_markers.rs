use crate::{
    style::knob::{
        ModRangeArcAppearance, TextMarksAppearance, TickMarksAppearance, ValueArcAppearance,
    },
    text_marks, tick_marks, ModulationRange,
};

pub struct ValueMarkers<'a> {
    pub tick_marks: Option<&'a tick_marks::Group>,
    pub text_marks: Option<&'a text_marks::Group>,
    pub mod_range_1: Option<&'a ModulationRange>,
    pub mod_range_2: Option<&'a ModulationRange>,
    pub tick_marks_style: Option<TickMarksAppearance>,
    pub text_marks_style: Option<TextMarksAppearance>,
    pub value_arc_style: Option<ValueArcAppearance>,
    pub mod_range_style_1: Option<ModRangeArcAppearance>,
    pub mod_range_style_2: Option<ModRangeArcAppearance>,
}
