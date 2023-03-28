use std::cmp::Ordering;
use std::fmt;
use std::iter::FusedIterator;
use cgmath::{EuclideanSpace as _, Point3, Vector3, Zero as _};
use crate::math::{Face6, FreeCoordinate, Geometry, GridAab, GridCoordinate, GridPoint};
use crate::mesh::LineVertex;
/// Axis-Aligned Box data type.
///
/// Note that this has continuous coordinates, and a discrete analogue exists as
/// [`GridAab`].
#[derive(Copy, Clone, PartialEq)]
pub struct Aab {
    lower_bounds: Point3<FreeCoordinate>,
    upper_bounds: Point3<FreeCoordinate>,
    sizes: Vector3<FreeCoordinate>,
}
impl Aab {
    /// The [`Aab`] of zero size at the origin.
    pub const ZERO: Aab = Aab {
        lower_bounds: Point3 { x: 0.0, y: 0.0, z: 0.0 },
        upper_bounds: Point3 { x: 0.0, y: 0.0, z: 0.0 },
        sizes: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    };
    /// Constructs an [`Aab`] from individual coordinates.
    #[track_caller]
    pub fn new(
        lx: FreeCoordinate,
        hx: FreeCoordinate,
        ly: FreeCoordinate,
        hy: FreeCoordinate,
        lz: FreeCoordinate,
        hz: FreeCoordinate,
    ) -> Self {
        loop {}
    }
    /// Constructs an [`Aab`] from most-negative and most-positive corner points.
    ///
    /// Panics if the points are not in the proper order or if they are NaN.
    #[track_caller]
    pub fn from_lower_upper(
        lower_bounds: impl Into<Point3<FreeCoordinate>>,
        upper_bounds: impl Into<Point3<FreeCoordinate>>,
    ) -> Self {
        loop {}
    }
    /// Constructs an [`Aab`] from most-negative and most-positive corner points.
    ///
    /// Returns [`None`] if the points are not in the proper order or if they are NaN.
    fn checked_from_lower_upper(
        lower_bounds: Point3<FreeCoordinate>,
        upper_bounds: Point3<FreeCoordinate>,
    ) -> Option<Self> {
        loop {}
    }
    /// Returns the AAB of a given cube in the interpretation used by [`GridAab`] and
    /// [`Space`](crate::space::Space); that is, a unit cube extending in the positive
    /// directions from the given point.
    ///
    /// ```
    /// use all_is_cubes::math::{Aab, GridPoint};
    ///
    /// assert_eq!(
    ///     Aab::from_cube(GridPoint::new(10, 20, -30)),
    ///     Aab::new(10.0, 11.0, 20.0, 21.0, -30.0, -29.0)
    /// );
    /// ```
    pub fn from_cube(cube: GridPoint) -> Self {
        loop {}
    }
    /// The most negative corner of the box, as a [`Point3`].
    pub const fn lower_bounds_p(&self) -> Point3<FreeCoordinate> {
        loop {}
    }
    /// The most positive corner of the box, as a [`Point3`].
    pub const fn upper_bounds_p(&self) -> Point3<FreeCoordinate> {
        loop {}
    }
    /// The most negative corner of the box, as a [`Vector3`].
    pub fn lower_bounds_v(&self) -> Vector3<FreeCoordinate> {
        loop {}
    }
    /// The most positive corner of the box, as a [`Vector3`].
    pub fn upper_bounds_v(&self) -> Vector3<FreeCoordinate> {
        loop {}
    }
    /// Returns the position of the identified face of the box on the axis it is
    /// perpendicular to.
    ///
    /// Note that negative faces' coordinates _are_ inverted; that is, all results
    /// will be positive if the box contains its origin.
    pub fn face_coordinate(&self, face: Face6) -> FreeCoordinate {
        loop {}
    }
    /// Size of the box in each axis; equivalent to
    /// `self.upper_bounds() - self.lower_bounds()`.
    pub fn size(&self) -> Vector3<FreeCoordinate> {
        loop {}
    }
    /// The center of the enclosed volume.
    ///
    /// ```
    /// use all_is_cubes::math::Aab;
    /// use cgmath::Point3;
    ///
    /// let aab = Aab::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    /// assert_eq!(aab.center(), Point3::new(1.5, 3.5, 5.5));
    /// ```
    pub fn center(&self) -> Point3<FreeCoordinate> {
        loop {}
    }
    /// Iterates over the eight corner points of the box.
    /// The ordering is deterministic but not currently declared stable.
    pub(crate) fn corner_points(
        self,
    ) -> impl Iterator<
        Item = Point3<FreeCoordinate>,
    > + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
        let l = self.lower_bounds;
        let u = self.upper_bounds;
        (0..8)
            .map(move |i| {
                Point3::new(
                    if i & 1 == 0 { l.x } else { u.x },
                    if i & 2 == 0 { l.y } else { u.y },
                    if i & 4 == 0 { l.z } else { u.z },
                )
            })
    }
    /// Returns whether this AAB, including the boundary, contains the point.
    ///
    /// TODO: example + tests
    pub fn contains(&self, point: Point3<FreeCoordinate>) -> bool {
        loop {}
    }
    /// Returns whether this AAB, including the boundary, intersects the other AAB.
    ///
    /// TODO: example + tests
    pub fn intersects(&self, other: Aab) -> bool {
        loop {}
    }
    /// Returns a random point within this box, using inclusive ranges
    /// (`lower_bounds[axis] ≤ random_point()[axis] ≤ upper_bounds[axis]`).
    pub fn random_point(self, rng: &mut impl rand::Rng) -> Point3<FreeCoordinate> {
        loop {}
    }
    /// Scale this AAB by the given amount (about the zero point, not its center).
    #[must_use]
    pub fn scale(self, scalar: FreeCoordinate) -> Self {
        loop {}
    }
    /// Enlarges the AAB by moving each face outward by the specified distance (or inward
    /// if negative).
    ///
    /// If this would result in a negative or NaN size, produces a zero size AAB located
    /// at the center point of `self`.
    ///
    /// ```
    /// use all_is_cubes::math::Aab;
    ///
    /// assert_eq!(
    ///     Aab::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).expand(0.25),
    ///     Aab::new(0.75, 2.25, 2.75, 4.25, 4.75, 6.25)
    /// );
    /// ````
    #[must_use]
    pub fn expand(self, distance: FreeCoordinate) -> Self {
        loop {}
    }
    #[inline]
    pub(crate) fn leading_corner(
        &self,
        direction: Vector3<FreeCoordinate>,
    ) -> Vector3<FreeCoordinate> {
        loop {}
    }
    /// Construct the [`GridAab`] containing all cubes this [`Aab`] intersects.
    ///
    /// Grid cubes are considered to be half-open ranges, so, for example, an [`Aab`] with
    /// exact integer bounds on some axis will convert exactly as one might intuitively
    /// expect, while non-integer bounds will be rounded outward:
    ///
    /// ```
    /// use all_is_cubes::math::{Aab, GridAab};
    ///
    /// let grid_aab = Aab::from_lower_upper([3.0, 0.5, 0.0], [5.0, 1.5, 1.0])
    ///     .round_up_to_grid();
    /// assert_eq!(grid_aab, GridAab::from_lower_upper([3, 0, 0], [5, 2, 1]));
    ///
    /// assert!(grid_aab.contains_cube([4, 1, 0]));
    /// assert!(!grid_aab.contains_cube([5, 1, 0]));
    /// ```
    ///
    /// If the floating-point coordinates are out of [`GridCoordinate`]'s numeric range,
    /// then they will be clamped.
    ///
    /// ```
    /// # use all_is_cubes::math::{Aab, GridAab};
    /// use all_is_cubes::math::{FreeCoordinate, GridCoordinate};
    ///
    /// assert_eq!(
    ///     Aab::from_lower_upper(
    ///         [3.0, 0.0, 0.0],
    ///         [(GridCoordinate::MAX as FreeCoordinate) * 10.0, 1.0, 1.0],
    ///     ).round_up_to_grid(),
    ///     GridAab::from_lower_upper([3, 0, 0], [GridCoordinate::MAX, 1, 1]),
    /// );
    /// assert_eq!(
    ///     Aab::from_lower_upper(
    ///         [3.0, 0.0, 0.0],
    ///         [FreeCoordinate::INFINITY, 1.0, 1.0],
    ///     ).round_up_to_grid(),
    ///     GridAab::from_lower_upper([3, 0, 0], [GridCoordinate::MAX, 1, 1]),
    /// );
    /// ```
    ///
    /// (There is no handling of NaN, because [`Aab`] does not allow NaN values.)
    pub fn round_up_to_grid(self) -> GridAab {
        loop {}
    }
}
impl fmt::Debug for Aab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Geometry for Aab {
    type Coord = FreeCoordinate;
    fn translate(self, offset: Vector3<FreeCoordinate>) -> Self {
        loop {}
    }
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<LineVertex>,
    {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_wrong_order() {
        loop {}
    }
    #[test]
    fn new_nan() {
        loop {}
    }
    #[test]
    #[should_panic = "invalid AAB points that are misordered or NaN: lower Point3 [0.0, 0.0, 0.0] upper Point3 [1.0, 1.0, NaN]"]
    fn new_panic_message() {
        loop {}
    }
    #[test]
    /// Test `Debug` formatting. Note this should be similar to the [`GridAab`]
    /// formatting.
    fn debug() {
        loop {}
    }
    #[test]
    fn expand_nan() {
        loop {}
    }
    #[test]
    fn expand_negative_failure() {
        loop {}
    }
    #[test]
    #[should_panic]
    fn expand_negative_success() {
        loop {}
    }
    #[test]
    fn expand_inf() {
        loop {}
    }
    #[test]
    fn wireframe_smoke_test() {
        loop {}
    }
    #[test]
    fn leading_corner_consistency() {
        loop {}
    }
    /// This would be a doc test except `corner_points` is not public for now
    /// (since it's oddball and not fully nailed down).
    #[test]
    fn corner_points() {
        loop {}
    }
}
