/// Tier of sizes for a tick mark.
///
/// * One - large-sized tick mark
/// * Two - medium-sized tick mark
/// * Small - small-sized tick mark
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, std::hash::Hash)]
pub enum Tier {
    /// large-sized tick mark
    #[default]
    One,
    /// medium-sized tick mark
    Two,
    /// small-sized tick mark
    Three,
}
