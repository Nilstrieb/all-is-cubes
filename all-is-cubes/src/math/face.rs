//! Axis-aligned unit vectors: the [`Face6`] and [`Face7`] types.
//! This module is private but reexported by its parent.
use cgmath::{BaseNum, Vector3};
use crate::math::*;
/// Identifies a face of a cube or an orthogonal unit vector.
///
/// See also the similar type [`Face7`], which adds a “zero” or “within the cube”
/// variant. The two enums use the same discriminant numbering.
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::exhaustive_enums)]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    PartialEq,
    exhaust::Exhaust,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub(crate) enum Face6 {
    /// Negative X; the face whose normal vector is `(-1, 0, 0)`.
    NX = 1,
    /// Negative Y; the face whose normal vector is `(0, -1, 0)`; downward.
    NY = 2,
    /// Negative Z; the face whose normal vector is `(0, 0, -1)`.
    NZ = 3,
    /// Positive X; the face whose normal vector is `(1, 0, 0)`.
    PX = 4,
    /// Positive Y; the face whose normal vector is `(0, 1, 0)`; upward.
    PY = 5,
    /// Positive Z; the face whose normal vector is `(0, 0, 1)`.
    PZ = 6,
}
/// Identifies a face of a cube or an orthogonal unit vector, except for
/// [`Within`](Face7::Within) meaning “zero distance and undefined direction”.
///
/// This is essentially `Option<`[`Face6`]`>`, except with `Face`-specific methods
/// provided. The two enums use the same discriminant numbering.
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::exhaustive_enums)]
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    PartialEq,
    exhaust::Exhaust,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub(crate) enum Face7 {
    /// The interior volume of a cube, or an undefined direction. Corresponds to the vector `(0, 0, 0)`.
    Within = 0,
    /// Negative X; the face whose normal vector is `(-1, 0, 0)`.
    NX,
    /// Negative Y; the face whose normal vector is `(0, -1, 0)`; downward.
    NY,
    /// Negative Z; the face whose normal vector is `(0, 0, -1)`.
    NZ,
    /// Positive X; the face whose normal vector is `(1, 0, 0)`.
    PX,
    /// Positive Y; the face whose normal vector is `(0, 1, 0)`; upward.
    PY,
    /// Positive Z; the face whose normal vector is `(0, 0, 1)`.
    PZ,
}
/// Container for values keyed by [`Face6`]s. Always holds exactly six elements.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub(crate) struct FaceMap<V> {
    /// The value whose key is [`Face6::PY`].
    pub(crate) py: V,
}
/// The combination of a [`GridPoint`] identifying a unit cube and a [`Face7`] identifying
/// one face of it. This pattern recurs in selection and collision detection.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)]
#[allow(missing_docs)]
pub(crate) struct CubeFace {}
