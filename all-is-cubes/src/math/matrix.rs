//! Integer-coordinate matrices.
//! This module is private but reexported by its parent.



/// A 4×3 affine transformation matrix in [`GridCoordinate`]s, rather than floats as
/// [`cgmath::Matrix4`] requires.
///
/// TODO: The operators implemented for this are very incomplete.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GridMatrix {}
