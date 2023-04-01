//! Data structures for light storage and algorithms.



/// Lighting within a [`Space`]; an [`Rgb`] value stored with reduced precision and range.
///
/// TODO: This now stores additional information. Rename to '`SpaceLight`' or some such.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct PackedLight {}
/// A priority queue for [`LightUpdateRequest`]s which contains cubes
/// at most once, even when added with different priorities.
pub(crate) struct LightUpdateQueue {}
