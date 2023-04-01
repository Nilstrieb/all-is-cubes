//! Integer-coordinate matrices.
//! This module is private but reexported by its parent.
use std::ops::Mul;
use cgmath::{Matrix4, One, Transform, Vector3, Vector4};
use crate::math::{
    Face7, FreeCoordinate, GridCoordinate, GridPoint, GridRotation, GridVector,
};
/// A 4×3 affine transformation matrix in [`GridCoordinate`]s, rather than floats as
/// [`cgmath::Matrix4`] requires.
///
/// TODO: The operators implemented for this are very incomplete.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GridMatrix {
    /// First column
    pub(crate) x: Vector3<GridCoordinate>,
    /// Second column
    pub(crate) y: Vector3<GridCoordinate>,
    /// Third column
    pub(crate) z: Vector3<GridCoordinate>,
    /// Fourth column (translation)
    pub(crate) w: Vector3<GridCoordinate>,
}
