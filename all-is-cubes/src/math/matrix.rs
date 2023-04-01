//! Integer-coordinate matrices.
//! This module is private but reexported by its parent.
use std::ops::Mul;
use cgmath::{
    EuclideanSpace as _, InnerSpace, Matrix4, One, Transform, Vector3, Vector4, Zero as _,
};
pub(crate) use ordered_float::{FloatIsNan, NotNan};
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
impl GridMatrix {
    pub(crate) const ZERO: Self = Self {
        x: Vector3::new(0, 0, 0),
        y: Vector3::new(0, 0, 0),
        z: Vector3::new(0, 0, 0),
        w: Vector3::new(0, 0, 0),
    };
    /// For Y-down drawing
    #[doc(hidden)]
    pub(crate) const FLIP_Y: Self = Self {
        x: Vector3::new(1, 0, 0),
        y: Vector3::new(0, -1, 0),
        z: Vector3::new(0, 0, 1),
        w: Vector3::new(0, 0, 0),
    };
    /// Note: This takes the same column-major ordering as [`cgmath`], so the argument order
    /// is transposed relative to a conventional textual display of a matrix.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub(crate) const fn new(
        x0: GridCoordinate,
        x1: GridCoordinate,
        x2: GridCoordinate,
        y0: GridCoordinate,
        y1: GridCoordinate,
        y2: GridCoordinate,
        z0: GridCoordinate,
        z1: GridCoordinate,
        z2: GridCoordinate,
        w0: GridCoordinate,
        w1: GridCoordinate,
        w2: GridCoordinate,
    ) -> Self {
        loop {}
    }
    /// Construct a translation matrix.
    #[inline]
    pub(crate) fn from_translation(offset: impl Into<GridVector>) -> Self {
        loop {}
    }
    /// Construct a uniform scaling matrix.
    ///
    /// Note that since this is an integer matrix, there is no possibility  of scaling less
    /// than 1 other than 0!
    #[inline]
    pub(crate) fn from_scale(scale: GridCoordinate) -> Self {
        loop {}
    }
    /// Construct a transformation to a translated and rotated coordinate system from
    /// an origin in the target coordinate system and basis vectors expressed as [`Face7`]s.
    ///
    /// Skews or scaling cannot be performed using this constructor.
    ///
    /// ```
    /// use cgmath::Transform as _;  // trait method Transform::transform_point
    /// use all_is_cubes::math::{Face7::*, GridMatrix, GridPoint};
    ///
    /// let transform = GridMatrix::from_origin([10, 10, 10], PX, PZ, NY);
    /// assert_eq!(
    ///     transform.transform_point(GridPoint::new(1, 2, 3)),
    ///     GridPoint::new(11, 7, 12),
    /// );
    /// ```
    #[inline]
    pub(crate) fn from_origin(
        origin: impl Into<GridPoint>,
        x: Face7,
        y: Face7,
        z: Face7,
    ) -> Self {
        loop {}
    }
    /// Convert this integer-valued matrix to an equivalent float-valued matrix.
    #[inline]
    pub(crate) fn to_free(self) -> Matrix4<FreeCoordinate> {
        loop {}
    }
    /// Returns row `r` of the matrix.
    ///
    /// Panics if `r >= 3`.
    #[inline(always)]
    pub(crate) fn row(&self, r: usize) -> Vector4<GridCoordinate> {
        loop {}
    }
    /// Equivalent to temporarily applying an offset of `[0.5, 0.5, 0.5]` while
    /// transforming `cube` as per [`GridMatrix::transform_point`], despite the fact that
    /// integer arithmetic is being used.
    ///
    /// This operation thus transforms the standard positive-octant unit cube identified
    /// by its most negative corner the same way as the [`GridAab::single_cube`] containing
    /// that cube.
    ///
    /// ```
    /// use all_is_cubes::math::{Face7::*, GridMatrix, GridPoint};
    /// use cgmath::Transform; // for transform_point
    ///
    /// // Translation without rotation has the usual definition.
    /// let matrix = GridMatrix::from_translation([10, 0, 0]);
    /// assert_eq!(matrix.transform_cube(GridPoint::new(1, 1, 1)), GridPoint::new(11, 1, 1));
    ///
    /// // With a rotation or reflection, the results are different.
    /// // TODO: Come up with a better example and explanation.
    /// let reflected = GridMatrix::from_origin([10, 0, 0], NX, PY, PZ);
    /// assert_eq!(reflected.transform_point(GridPoint::new(1, 5, 5)), GridPoint::new(9, 5, 5));
    /// assert_eq!(reflected.transform_cube(GridPoint::new(1, 5, 5)), GridPoint::new(8, 5, 5));
    /// ```
    ///
    /// [`GridAab::single_cube`]: crate::math::GridAab::single_cube
    #[inline]
    pub(crate) fn transform_cube(&self, cube: GridPoint) -> GridPoint {
        loop {}
    }
    /// Decomposes a matrix into its rotation and translation components.
    /// Returns `None` if the matrix has any scaling or skew.
    ///
    /// ```
    /// use all_is_cubes::math::{Face6::*, GridMatrix, GridRotation, GridVector};
    ///
    /// assert_eq!(
    ///     GridMatrix::new(
    ///         0, -1,  0,
    ///         1,  0,  0,
    ///         0,  0,  1,
    ///         7,  3, -8,
    ///     ).decompose(),
    ///     Some((
    ///         GridRotation::from_basis([NY, PX, PZ]),
    ///         GridVector::new(7, 3, -8),
    ///     )),
    /// );
    /// ```
    pub(crate) fn decompose(self) -> Option<(GridRotation, GridVector)> {
        loop {}
    }
}
impl Mul<Self> for GridMatrix {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        loop {}
    }
}
impl One for GridMatrix {
    #[inline]
    #[rustfmt::skip]
    fn one() -> Self {
        loop {}
    }
}
impl Transform<GridPoint> for GridMatrix {
    fn look_at(eye: GridPoint, center: GridPoint, up: GridVector) -> Self {
        loop {}
    }
    fn look_at_rh(_eye: GridPoint, _center: GridPoint, _up: GridVector) -> Self {
        loop {}
    }
    fn look_at_lh(_eye: GridPoint, _center: GridPoint, _up: GridVector) -> Self {
        loop {}
    }
    #[inline]
    fn transform_vector(&self, vec: GridVector) -> GridVector {
        loop {}
    }
    #[inline]
    fn transform_point(&self, point: GridPoint) -> GridPoint {
        loop {}
    }
    /// ```
    /// use all_is_cubes::math::{GridMatrix, GridPoint};
    /// use cgmath::Transform as _;
    ///
    /// let transform_1 = GridMatrix::new(
    ///     0, -1, 0,
    ///     1, 0, 0,
    ///     0, 0, 1,
    ///     0, 0, 0,
    /// );
    /// let transform_2 = GridMatrix::from_translation([10, 20, 30]);
    ///
    /// // Demonstrate the directionality of concatenation.
    /// assert_eq!(
    ///     transform_1.concat(&transform_2).transform_point(GridPoint::new(0, 3, 0)),
    ///     transform_1.transform_point(transform_2.transform_point(GridPoint::new(0, 3, 0))),
    /// );
    /// ```
    fn concat(&self, other: &Self) -> Self {
        loop {}
    }
    fn inverse_transform(&self) -> Option<Self> {
        loop {}
    }
}
