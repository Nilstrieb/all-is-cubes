//! Rotations which exchange axes (thus not leaving the integer grid).
//! This module is private but reexported by its parent.
use std::ops::Mul;
use cgmath::{One, Vector3};
use crate::math::*;
/// Represents a discrete (grid-aligned) rotation, or exchange of axes.
///
/// Compared to a [`GridMatrix`], this cannot specify scale, translation, or skew;
/// it is used for identifying the rotations of blocks.
///
/// Each of the variant names specifies the three unit vectors which (*x*, *y*, *z*),
/// respectively, should be multiplied by to perform the rotation.
/// Lowercase refers to a negated unit vector.
///
/// See also:
///
/// * [`Face6`] is less general, in that it specifies a single axis but not
///   rotation about that axis.
/// * [`GridMatrix`] is more general, specifying an affine transformation.
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::exhaustive_enums)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub(crate) enum GridRotation {
    RXYZ,
    RXYz,
    RXyZ,
    RXyz,
    RxYZ,
    RxYz,
    RxyZ,
    Rxyz,
    RXZY,
    RXZy,
    RXzY,
    RXzy,
    RxZY,
    RxZy,
    RxzY,
    Rxzy,
    RYXZ,
    RYXz,
    RYxZ,
    RYxz,
    RyXZ,
    RyXz,
    RyxZ,
    Ryxz,
    RYZX,
    RYZx,
    RYzX,
    RYzx,
    RyZX,
    RyZx,
    RyzX,
    Ryzx,
    RZXY,
    RZXy,
    RZxY,
    RZxy,
    RzXY,
    RzXy,
    RzxY,
    Rzxy,
    RZYX,
    RZYx,
    RZyX,
    RZyx,
    RzYX,
    RzYx,
    RzyX,
    Rzyx,
}
