//! Rotations which exchange axes (thus not leaving the integer grid).
//! This module is private but reexported by its parent.
use std::ops::Mul;
use cgmath::{One, Vector3, Zero as _};
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
impl GridRotation {
    /// All 48 possible rotations.
    ///
    /// Warning: TODO: The ordering of these rotations is not yet stable.
    /// The current ordering is based on the six axis permutations followed by rotations.
    #[rustfmt::skip]
    pub(crate) const ALL: [Self; 48] = {
        use GridRotation::*;
        [
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
        ]
    };
    /// All possible rotations that are not reflections.
    ///
    /// Warning: TODO: The ordering of these rotations is not yet stable.
    #[rustfmt::skip]
    pub(crate) const ALL_BUT_REFLECTIONS: [Self; 24] = {
        use GridRotation::*;
        [
            RXYZ,
            RXyz,
            RxYz,
            RxyZ,
            RXZy,
            RXzY,
            RxZY,
            Rxzy,
            RYXz,
            RYxZ,
            RyXZ,
            Ryxz,
            RYZX,
            RYzx,
            RyZx,
            RyzX,
            RZXY,
            RZxy,
            RzXy,
            RzxY,
            RZYx,
            RZyX,
            RzYX,
            Rzyx,
        ]
    };
    /// The identity rotation, also known as [`RXYZ`](Self::RXYZ).
    pub(crate) const IDENTITY: Self = Self::RXYZ;
    /// The rotation that is clockwise in our Y-up right-handed coordinate system.
    ///
    /// ```
    /// use all_is_cubes::math::{Face6::*, GridRotation};
    ///
    /// assert_eq!(GridRotation::CLOCKWISE.transform(PX), PZ);
    /// assert_eq!(GridRotation::CLOCKWISE.transform(PZ), NX);
    /// assert_eq!(GridRotation::CLOCKWISE.transform(NX), NZ);
    /// assert_eq!(GridRotation::CLOCKWISE.transform(NZ), PX);
    ///
    /// assert_eq!(GridRotation::CLOCKWISE.transform(PY), PY);
    /// ```
    pub(crate) const CLOCKWISE: Self = Self::RZYx;
    /// The rotation that is counterclockwise in our Y-up right-handed coordinate system.
    ///
    /// ```
    /// use all_is_cubes::math::{Face6::*, GridRotation};
    ///
    /// assert_eq!(GridRotation::COUNTERCLOCKWISE.transform(PX), NZ);
    /// assert_eq!(GridRotation::COUNTERCLOCKWISE.transform(NZ), NX);
    /// assert_eq!(GridRotation::COUNTERCLOCKWISE.transform(NX), PZ);
    /// assert_eq!(GridRotation::COUNTERCLOCKWISE.transform(PZ), PX);
    ///
    /// assert_eq!(GridRotation::COUNTERCLOCKWISE.transform(PY), PY);
    /// ```
    pub(crate) const COUNTERCLOCKWISE: Self = Self::RzYX;
    /// Constructs a rotation from a basis: that is, the returned rotation will
    /// rotate `PX` into `basis[0]`, `PY` into `basis[1]`, and `PZ` into `basis[2]`.
    ///
    /// Panics if the three provided axes are not mutually perpendicular.
    #[inline]
    pub(crate) fn from_basis(basis: impl Into<Vector3<Face6>>) -> Self {
        loop {}
    }
    fn from_basis_impl(basis: Vector3<Face6>) -> Self {
        loop {}
    }
    /// Find the rotation (without reflection) which rotates `source` to `destination`.
    /// and leaves `up` unaffected. (This might also be considered a “look at” operation).
    ///
    /// If it is not possible to leave `up` unaffected, returns [`None`]. (Trying two
    /// perpendicular `up` directions will always succeed.)
    pub(crate) fn from_to(source: Face6, destination: Face6, up: Face6) -> Option<Self> {
        loop {}
    }
    #[inline]
    #[rustfmt::skip]
    pub(crate) const fn to_basis(self) -> Vector3<Face6> {
        loop {}
    }
    /// Expresses this rotation as a matrix which rotates “in place” the
    /// points within the volume defined by coordinates in the range [0, size].
    ///
    /// That is, a [`GridAab`] of that volume will be unchanged by rotation:
    ///
    /// ```
    /// use all_is_cubes::block::Resolution;
    /// use all_is_cubes::math::{GridAab, GridRotation};
    ///
    /// let b = GridAab::for_block(Resolution::R8);
    /// let rotation = GridRotation::CLOCKWISE.to_positive_octant_matrix(8);
    /// assert_eq!(b.transform(rotation), Some(b));
    /// ```
    ///
    /// Such matrices are suitable for rotating the voxels of a block, provided
    /// that the coordinates are then transformed with [`GridMatrix::transform_cube`],
    /// *not* [`GridMatrix::transform_point`](cgmath::Transform::transform_point)
    /// (due to the lower-corner format of cube coordinates).
    /// ```
    /// # use all_is_cubes::math::{GridAab, GridPoint, GridRotation};
    /// let rotation = GridRotation::CLOCKWISE.to_positive_octant_matrix(4);
    /// assert_eq!(rotation.transform_cube(GridPoint::new(0, 0, 0)), GridPoint::new(3, 0, 0));
    /// assert_eq!(rotation.transform_cube(GridPoint::new(3, 0, 0)), GridPoint::new(3, 0, 3));
    /// assert_eq!(rotation.transform_cube(GridPoint::new(3, 0, 3)), GridPoint::new(0, 0, 3));
    /// assert_eq!(rotation.transform_cube(GridPoint::new(0, 0, 3)), GridPoint::new(0, 0, 0));
    /// ```
    ///
    pub(crate) fn to_positive_octant_matrix(self, size: GridCoordinate) -> GridMatrix {
        loop {}
    }
    /// Expresses this rotation as a matrix without any translation.
    pub(crate) fn to_rotation_matrix(self) -> GridMatrix {
        loop {}
    }
    /// Rotate the face by this rotation.
    #[inline]
    pub(crate) fn transform(self, face: Face6) -> Face6 {
        loop {}
    }
    /// Returns whether this is a reflection.
    ///
    /// ```
    /// use all_is_cubes::math::{GridRotation, Face6::*};
    ///
    /// assert!(!GridRotation::IDENTITY.is_reflection());
    /// assert!(!GridRotation::from_basis([PX, PZ, NY]).is_reflection());
    /// assert!(GridRotation::from_basis([PX, PZ, PY]).is_reflection());
    /// ```
    #[inline]
    pub(crate) const fn is_reflection(self) -> bool {
        loop {}
    }
    /// Returns the inverse of this rotation; the one which undoes this.
    ///
    /// ```
    /// use all_is_cubes::math::GridRotation;
    ///
    /// for &rotation in &GridRotation::ALL {
    ///     assert_eq!(rotation * rotation.inverse(), GridRotation::IDENTITY);
    /// }
    /// ```
    #[must_use]
    pub(crate) fn inverse(self) -> Self {
        loop {}
    }
    /// Generates the sequence of rotations that may be obtained by concatenating/multiplying
    /// this rotation with itself repeatedly.
    ///
    /// The first element of the iterator will always be the identity, i.e. this rotation
    /// applied zero times. The iterator ends when the sequence would repeat itself, i.e.
    /// just before it would produce the identity again.
    ///
    /// ```
    /// use all_is_cubes::math::Face6::*;
    /// use all_is_cubes::math::GridRotation;
    ///
    /// assert_eq!(
    ///     GridRotation::IDENTITY.iterate().collect::<Vec<_>>(),
    ///     vec![GridRotation::IDENTITY],
    /// );
    ///
    /// let x_reflection = GridRotation::from_basis([NX, PY, PZ]);
    /// assert_eq!(
    ///     x_reflection.iterate().collect::<Vec<_>>(),
    ///     vec![GridRotation::IDENTITY, x_reflection],
    /// );
    ///
    /// assert_eq!(
    ///     GridRotation::CLOCKWISE.iterate().collect::<Vec<_>>(),
    ///     vec![
    ///         GridRotation::IDENTITY,
    ///         GridRotation::CLOCKWISE,
    ///         GridRotation::CLOCKWISE * GridRotation::CLOCKWISE,
    ///         GridRotation::COUNTERCLOCKWISE,
    ///    ],
    /// );
    /// ```
    pub(crate) fn iterate(self) -> impl Iterator<Item = Self> {
        let mut item = Self::IDENTITY;
        std::iter::once(Self::IDENTITY)
            .chain(
                std::iter::from_fn(move || {
                    item = item * self;
                    if item == Self::IDENTITY { None } else { Some(item) }
                }),
            )
    }
}
impl Default for GridRotation {
    /// Returns the identity (no rotation).
    #[inline]
    fn default() -> Self {
        loop {}
    }
}
impl One for GridRotation {
    /// Returns the identity (no rotation).
    #[inline]
    fn one() -> Self {
        loop {}
    }
}
impl Mul<Self> for GridRotation {
    type Output = Self;
    /// Multiplication is concatenation: `self * rhs` is equivalent to
    /// applying `rhs` and then applying `self`.
    /// ```
    /// use all_is_cubes::math::{Face6, Face6::*, GridRotation, GridPoint};
    ///
    /// let transform_1 = GridRotation::from_basis([NY, PX, PZ]);
    /// let transform_2 = GridRotation::from_basis([PY, PZ, PX]);
    ///
    /// // Demonstrate the directionality of concatenation.
    /// for face in Face6::ALL {
    ///     assert_eq!(
    ///         (transform_1 * transform_2).transform(face),
    ///         transform_1.transform(transform_2.transform(face)),
    ///     );
    /// }
    /// ```
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        loop {}
    }
}
