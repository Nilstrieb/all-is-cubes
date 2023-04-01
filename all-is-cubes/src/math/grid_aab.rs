//! Axis-aligned integer-coordinate box volumes ([`GridAab`]), arrays bounded by them
//! ([`GridArray`]), and related.
use std::fmt;
use std::iter::FusedIterator;
use std::ops::Range;
use cgmath::{Point3, Vector3};
use crate::block::Resolution;
use crate::math::{
    Face6, FaceMap, FreeCoordinate, GridCoordinate, GridMatrix, GridPoint, GridVector,
};
/// An axis-aligned box with integer coordinates, whose volume is no larger than [`usize::MAX`].
/// [`GridAab`]s are used to specify the coordinate extent of [`Space`](crate::space::Space)s, and
/// regions within them.
///
/// When we refer to “a cube” in a [`GridAab`], that is a unit cube which is identified by the
/// integer coordinates of its most negative corner. Hence, coordinate bounds are always
/// half-open intervals: lower inclusive and upper exclusive. There are functions to help with
/// this such as [`cube_to_midpoint`](super::cube_to_midpoint).
///
/// A [`GridAab`] may have a zero-size range in any direction, thus making its total volume zero.
/// The different possibilities are not considered equal; thus, points, lines, and planes may be
/// represented, which may be useful for procedural-generation purposes.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct GridAab {}
impl GridAab {
    /// Box containing the unit cube from `[0, 0, 0]` to `[1, 1, 1]`.
    ///
    /// This constant is for convenience; there are several other ways that this box could
    /// be constructed, but they're all kind of verbose:
    ///
    /// ```
    /// use all_is_cubes::block::Resolution;
    /// use all_is_cubes::math::{GridAab, GridPoint};
    ///
    /// assert_eq!(GridAab::ORIGIN_CUBE, GridAab::from_lower_upper([0, 0, 0], [1, 1, 1]));
    ///
    /// // Note that GridAab::for_block() is const too.
    /// assert_eq!(GridAab::ORIGIN_CUBE, GridAab::for_block(Resolution::R1));
    ///
    /// assert_eq!(GridAab::ORIGIN_CUBE, GridAab::single_cube(GridPoint::new(0, 0, 0)));
    /// ```
    ///
    pub const ORIGIN_CUBE: GridAab = GridAab::for_block(Resolution::R1);
    /// Constructs a [`GridAab`] from coordinate lower bounds and sizes.
    ///
    /// For example, if on one axis the lower bound is 5 and the size is 10,
    /// then the positions where blocks can exist are numbered 5 through 14
    /// (inclusive) and the occupied volume (from a perspective of continuous
    /// rather than discrete coordinates) spans 5 to 15.
    ///
    /// Panics if the sizes are negative or the resulting range would cause
    /// numeric overflow. Use [`GridAab::checked_from_lower_size`] to avoid panics.
    #[track_caller]
    pub(crate) fn from_lower_size(
        lower_bounds: impl Into<GridPoint>,
        sizes: impl Into<GridVector>,
    ) -> Self {
        loop {}
    }
    /// Constructs a [`GridAab`] from coordinate lower bounds and sizes.
    ///
    /// For example, if on one axis the lower bound is 5 and the size is 10,
    /// then the positions where blocks can exist are numbered 5 through 14
    /// (inclusive) and the occupied volume (from a perspective of continuous
    /// rather than discrete coordinates) spans 5 to 15.
    ///
    /// Returns [`Err`] if the sizes are non-negative or the resulting range would cause
    /// numeric overflow.
    pub(crate) fn checked_from_lower_size(
        lower_bounds: impl Into<GridPoint>,
        sizes: impl Into<GridVector>,
    ) -> Result<Self, GridOverflowError> {
        loop {}
    }
    /// Constructs a [`GridAab`] from inclusive lower bounds and exclusive upper bounds.
    ///
    /// For example, if on one axis the lower bound is 5 and the upper bound is 10,
    /// then the positions where blocks can exist are numbered 5 through 9
    /// (inclusive) and the occupied volume (from a perspective of continuous
    /// rather than discrete coordinates) spans 5 to 10.
    #[track_caller]
    pub(crate) fn from_lower_upper(
        lower_bounds: impl Into<GridPoint>,
        upper_bounds: impl Into<GridPoint>,
    ) -> GridAab {
        loop {}
    }
    /// Constructs a [`GridAab`] from inclusive lower bounds and exclusive upper bounds.
    ///
    /// Returns [`Err`] if the bounds are reversed or the resulting range would cause
    /// numeric overflow.
    #[track_caller]
    pub(crate) fn checked_from_lower_upper(
        lower_bounds: impl Into<GridPoint>,
        upper_bounds: impl Into<GridPoint>,
    ) -> Result<Self, GridOverflowError> {
        loop {}
    }
    /// Constructs a [`GridAab`] with a volume of 1, containing the specified cube.
    ///
    /// Panics if `cube` has any coordinates equal to [`GridCoordinate::MAX`](i32::MAX)
    /// since that is not valid, as per [`GridAab::from_lower_size`].
    #[inline]
    pub(crate) fn single_cube(cube: GridPoint) -> GridAab {
        loop {}
    }
    /// Constructs a [`GridAab`] with a cubical volume in the positive octant, as is used
    /// for recursive blocks.
    ///
    /// If you need such a box at a position other than the origin, use
    /// [`GridAab::translate()`].
    #[inline]
    pub(crate) const fn for_block(resolution: Resolution) -> GridAab {
        loop {}
    }
    /// Generate a [`GridAab`] whose volume is as specified or smaller.
    #[cfg(feature = "arbitrary")]
    pub(crate) fn arbitrary_with_max_volume(
        u: &mut arbitrary::Unstructured<'_>,
        volume: usize,
    ) -> arbitrary::Result<Self> {
        loop {}
    }
    #[cfg(feature = "arbitrary")]
    const ARBITRARY_SIZE_HINT: (usize, Option<usize>) = { loop {} };
    /// Compute volume with checked arithmetic. In a function solely for the convenience
    /// of the `?` operator without which this is even worse.
    fn checked_volume_helper(sizes: GridVector) -> Result<usize, ()> {
        loop {}
    }
    /// Computes the volume of this box in cubes, i.e. the product of all sizes.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    ///
    /// let a = GridAab::from_lower_size([-10, 3, 7], [100, 200, 300]);
    /// assert_eq!(a.volume(), 6_000_000);
    ///
    /// let b = GridAab::from_lower_size([0, 0, 0], [100, 200, 0]);
    /// assert_eq!(b.volume(), 0);
    /// ```
    pub(crate) fn volume(&self) -> usize {
        loop {}
    }
    /// Returns whether the box contains no cubes (its volume is zero).
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        loop {}
    }
    /// Determines whether a unit cube lies within this box and, if it does, returns the
    /// flattened array index for it.
    ///
    /// The flattening is currently X major, Z minor, but this is not guaranteed to be
    /// the same in future versions; profiling may lead us to choose to place the Y axis
    /// first or last.
    ///
    /// ```
    /// let bounds = all_is_cubes::math::GridAab::from_lower_size([0, 0, 0], [10, 10, 10]);
    /// assert_eq!(bounds.index((0, 0, 0)), Some(0));
    /// assert_eq!(bounds.index((1, 2, 3)), Some(123));
    /// assert_eq!(bounds.index((9, 9, 9)), Some(999));
    /// assert_eq!(bounds.index((0, 0, -1)), None);
    /// assert_eq!(bounds.index((0, 0, 10)), None);
    /// ```
    #[inline(always)]
    pub(crate) fn index(&self, point: impl Into<GridPoint>) -> Option<usize> {
        loop {}
    }
    /// Inclusive upper bounds on cube coordinates, or the most negative corner of the
    /// box.
    #[inline]
    pub(crate) fn lower_bounds(&self) -> GridPoint {
        loop {}
    }
    /// Exclusive upper bounds on cube coordinates, or the most positive corner of the
    /// box.
    #[inline]
    pub(crate) fn upper_bounds(&self) -> GridPoint {
        loop {}
    }
    /// Size of the box in each axis; equivalent to
    /// `self.upper_bounds() - self.lower_bounds()`.
    #[inline]
    pub(crate) fn size(&self) -> GridVector {
        loop {}
    }
    /// Size of the box in each axis; equivalent to
    /// `self.upper_bounds() - self.lower_bounds()`, except that the result is an
    /// unsigned integer.
    ///
    /// Compared to [`GridAab::size()`], this is a convenience so that callers needing
    /// unsigned integers do not need to write a fallible-looking conversion.
    #[inline]
    pub(crate) fn unsigned_size(&self) -> Vector3<u32> {
        loop {}
    }
    /// The range of X coordinates for unit cubes within the box.
    #[inline]
    pub(crate) fn x_range(&self) -> Range<GridCoordinate> {
        loop {}
    }
    /// The range of Y coordinates for unit cubes within the box.
    #[inline]
    pub(crate) fn y_range(&self) -> Range<GridCoordinate> {
        loop {}
    }
    /// The range of Z coordinates for unit cubes within the box.
    #[inline]
    pub(crate) fn z_range(&self) -> Range<GridCoordinate> {
        loop {}
    }
    /// The range of coordinates for cubes within the box along the given axis.
    ///
    /// Panics if `axis >= 3`.
    #[inline]
    pub(crate) fn axis_range(&self, axis: usize) -> Range<GridCoordinate> {
        loop {}
    }
    /// The center of the enclosed volume. Returns [`FreeCoordinate`] since the center
    /// may be at a half-block position.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    /// use all_is_cubes::cgmath::Point3;
    ///
    /// let b = GridAab::from_lower_size([0, 0, -2], [10, 3, 4]);
    /// assert_eq!(b.center(), Point3::new(5.0, 1.5, 0.0));
    /// ```
    #[inline]
    pub(crate) fn center(&self) -> Point3<FreeCoordinate> {
        loop {}
    }
    /// Iterate over all cubes.
    ///
    /// ```
    /// use all_is_cubes::math::{GridAab, GridPoint};
    ///
    /// let b = GridAab::from_lower_size([10, 20, 30], [1, 2, 3]);
    /// assert_eq!(
    ///     b.interior_iter().collect::<Vec<GridPoint>>(),
    ///     &[
    ///         GridPoint::new(10, 20, 30),
    ///         GridPoint::new(10, 20, 31),
    ///         GridPoint::new(10, 20, 32),
    ///         GridPoint::new(10, 21, 30),
    ///         GridPoint::new(10, 21, 31),
    ///         GridPoint::new(10, 21, 32),
    ///     ])
    /// ```
    pub(crate) fn interior_iter(self) -> GridIter {
        loop {}
    }
    /// Returns whether the box includes the cube with the given coordinates in its
    /// volume.
    ///
    /// ```
    /// let b = all_is_cubes::math::GridAab::from_lower_size([4, 4, 4], [6, 6, 6]);
    /// assert!(!b.contains_cube([3, 5, 5]));
    /// assert!(b.contains_cube([4, 5, 5]));
    /// assert!(b.contains_cube([9, 5, 5]));
    /// assert!(!b.contains_cube([10, 5, 5]));
    /// ```
    #[inline]
    pub(crate) fn contains_cube(&self, point: impl Into<GridPoint>) -> bool {
        loop {}
    }
    /// Returns whether this box includes every cube in the other box.
    ///
    /// TODO: Precisely define the behavior on zero volume boxes.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    /// let b46 = GridAab::from_lower_size([4, 4, 4], [6, 6, 6]);
    /// assert!(b46.contains_box(b46));
    /// assert!(!b46.contains_box(GridAab::from_lower_size([4, 4, 4], [7, 6, 6])));
    /// assert!(!GridAab::from_lower_size((0, 0, 0), (6, 6, 6)).contains_box(b46));
    /// ```
    pub(crate) fn contains_box(&self, other: GridAab) -> bool {
        loop {}
    }
    /// Returns the intersection of two grids, or None if they have no cubes in common.
    ///
    /// TODO: Precisely define the behavior on zero volume grids.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    ///
    /// let g1 = GridAab::from_lower_size([1, 2, 3], [4, 5, 6]);
    /// assert_eq!(g1.intersection(g1), Some(g1));
    /// assert_eq!(
    ///     GridAab::from_lower_size([0, 0, 0], [2, 2, 2]).intersection(
    ///        GridAab::from_lower_size([2, 0, 0], [2, 1, 2])),
    ///     None);
    /// assert_eq!(
    ///     GridAab::from_lower_size([0, 0, 0], [2, 2, 2]).intersection(
    ///         GridAab::from_lower_size([1, 0, 0], [2, 1, 2])),
    ///     Some(GridAab::from_lower_size([1, 0, 0], [1, 1, 2])));
    /// ```
    pub(crate) fn intersection(self, other: GridAab) -> Option<GridAab> {
        loop {}
    }
    /// Returns the smallest [`GridAab`] which fully encloses the two inputs,
    /// or [`GridOverflowError`] if the volume of the result exceeds [`usize::MAX`].
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    ///
    /// let g1 = GridAab::from_lower_size([1, 2, 3], [1, 1, 1]);
    /// assert_eq!(g1.union(g1), Ok(g1));
    ///
    /// let g2 = GridAab::from_lower_size([4, 7, 11], [1, 1, 1]);
    /// assert_eq!(g1.union(g2), Ok(GridAab::from_lower_upper([1, 2, 3], [5, 8, 12])));
    ///
    /// let u = i32::MAX - 1;
    /// g1.union(GridAab::from_lower_size([u, u, u], [1, 1, 1]))
    ///     .unwrap_err();
    /// ```
    #[inline]
    pub(crate) fn union(self, other: GridAab) -> Result<GridAab, GridOverflowError> {
        loop {}
    }
    pub(crate) fn minkowski_sum(
        self,
        other: GridAab,
    ) -> Result<GridAab, GridOverflowError> {
        loop {}
    }
    /// Returns a random cube contained by the box, if there are any.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    /// use rand::SeedableRng;
    /// let mut rng = &mut rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    ///
    /// let b = GridAab::from_lower_size([4, 4, 4], [6, 6, 6]);
    /// for _ in 0..50 {
    ///     assert!(b.contains_cube(b.random_cube(rng).unwrap()));
    /// }
    ///
    /// let empty = GridAab::from_lower_size([1, 2, 3], [0, 9, 9]);
    /// assert_eq!(empty.random_cube(rng), None);
    /// ```
    pub(crate) fn random_cube(&self, rng: &mut impl rand::Rng) -> Option<GridPoint> {
        loop {}
    }
    /// Displaces the box by the given `offset`, leaving its size unchanged
    /// (unless that is impossible due to numeric overflow).
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    ///
    /// assert_eq!(
    ///     GridAab::from_lower_size([0, 0, 0], [10, 20, 30]).translate([-10, 0, 0]),
    ///     GridAab::from_lower_size([-10, 0, 0], [10, 20, 30]),
    /// );
    /// ```
    #[must_use]
    pub(crate) fn translate(&self, offset: impl Into<GridVector>) -> Self {
        loop {}
    }
    /// Transforms the box.
    ///
    /// Caution: The results are undefined if the matrix mixes axes
    /// rather than only swapping and scaling them.
    /// TODO: Find the proper mathematical concept to explain that.
    /// TODO: Check and error in that case.
    ///
    /// TODO: Fail nicely on numeric overflow.
    /// The `Option` return is not currently used.
    #[must_use]
    pub(crate) fn transform(self, transform: GridMatrix) -> Option<Self> {
        loop {}
    }
    /// Scales the box down by the given factor, rounding outward.
    ///
    /// For example, this may be used to convert from voxels (subcubes) to blocks or
    /// blocks to chunks.
    ///
    /// Panics if the divisor is not positive.
    ///
    /// ```
    /// # use all_is_cubes::math::GridAab;
    /// assert_eq!(
    ///     GridAab::from_lower_size([-10, -10, -10], [20, 20, 20]).divide(10),
    ///     GridAab::from_lower_size([-1, -1, -1], [2, 2, 2]),
    /// );
    /// assert_eq!(
    ///     GridAab::from_lower_size([-10, -10, -10], [21, 21, 21]).divide(10),
    ///     GridAab::from_lower_size([-1, -1, -1], [3, 3, 3]),
    /// );
    /// assert_eq!(
    ///     GridAab::from_lower_size([-11, -11, -11], [20, 20, 20]).divide(10),
    ///     GridAab::from_lower_size([-2, -2, -2], [3, 3, 3]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub(crate) fn divide(self, divisor: GridCoordinate) -> Self {
        loop {}
    }
    /// Scales the box up by the given factor.
    ///
    /// Panics on numeric overflow.
    ///
    /// ```
    /// # use all_is_cubes::math::GridAab;
    /// assert_eq!(
    ///     GridAab::from_lower_size([-1, 2, 3], [4, 5, 6]).multiply(10),
    ///     GridAab::from_lower_size([-10, 20, 30], [40, 50, 60]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub(crate) fn multiply(self, scale: GridCoordinate) -> Self {
        loop {}
    }
    /// Moves all bounds outward or inward by the specified distances.
    ///
    /// TODO: Currently this will panic if the result is empty. Make it return Option
    /// instead.
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    /// use all_is_cubes::math::FaceMap;
    ///
    /// assert_eq!(
    ///     GridAab::from_lower_upper([10, 10, 10], [20, 20, 20])
    ///         .expand(FaceMap {
    ///             nx: 1, ny: 2, nz: 3,
    ///             px: 4, py: 5, pz: 6,
    ///         }),
    ///     GridAab::from_lower_upper([9, 8, 7], [24, 25, 26]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub(crate) fn expand(self, deltas: FaceMap<GridCoordinate>) -> Self {
        loop {}
    }
    /// Returns a [`GridAab`] which includes the volume between the given `face` rectangle
    /// of `self` and the same rectangle translated `thickness` cubes outward from it
    /// (inward if negative).
    ///
    /// Edge cases:
    /// * If `thickness` is negative and greater than the size of the input, it is clamped
    ///   (so that the returned [`GridAab`] never extends beyond the opposite face of
    ///   `self`).
    ///
    /// For example, it may be used to construct the walls of a room:
    ///
    /// ```
    /// use all_is_cubes::math::GridAab;
    /// use all_is_cubes::math::Face6;
    ///
    /// let interior = GridAab::from_lower_upper([10, 10, 10], [20, 20, 20]);
    /// let left_wall = interior.abut(Face6::NX, 2)?;
    /// let right_wall = interior.abut(Face6::PX, 2)?;
    ///
    /// assert_eq!(left_wall, GridAab::from_lower_upper([8, 10, 10], [10, 20, 20]));
    /// assert_eq!(right_wall, GridAab::from_lower_upper([20, 10, 10], [22, 20, 20]));
    /// # Ok::<(), all_is_cubes::math::GridOverflowError>(())
    /// ```
    ///
    /// Example of negative thickness:
    ///
    /// ```
    /// # use all_is_cubes::math::GridAab;
    /// # use all_is_cubes::math::Face6;
    ///
    /// let b = GridAab::from_lower_upper([10, 10, 10], [20, 20, 20]);
    /// assert_eq!(
    ///     b.abut(Face6::PX, -3)?,
    ///     GridAab::from_lower_upper([17, 10, 10], [20, 20, 20]),
    /// );
    /// assert_eq!(
    ///     // Thicker than the input, therefore clamped.
    ///     b.abut(Face6::PX, -30)?,
    ///     GridAab::from_lower_upper([10, 10, 10], [20, 20, 20]),
    /// );
    /// # Ok::<(), all_is_cubes::math::GridOverflowError>(())
    /// ```
    #[inline]
    pub(crate) fn abut(
        self,
        face: Face6,
        thickness: GridCoordinate,
    ) -> Result<Self, GridOverflowError> {
        loop {}
    }
}
impl fmt::Debug for GridAab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for GridAab {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// Iterator produced by [`GridAab::interior_iter()`].
#[derive(Clone, Debug)]
pub(crate) struct GridIter {}
impl GridIter {
    #[inline]
    fn new(bounds: GridAab) -> Self {
        loop {}
    }
}
impl Iterator for GridIter {
    type Item = GridPoint;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        loop {}
    }
}
impl ExactSizeIterator for GridIter {}
impl FusedIterator for GridIter {}
/// Error when a [`GridAab`] cannot be constructed from the given input.
#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct GridOverflowError(String);
/// A 3-dimensional array with arbitrary element type instead of [`Space`](crate::space::Space)'s
/// fixed types.
///
/// TODO: Should we rebuild Space on top of this?
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GridArray<V> {
    contents: Box<[V]>,
}
