//! [`Cursor`] type and related items.
//!
//! TODO: It's unclear what the scope of this module should be.
use std::fmt;
use cgmath::Point3;
use crate::block::{Block, EvaluatedBlock};
use crate::math::{
    Face7, FreeCoordinate, Geometry, GridCoordinate, GridPoint, GridVector,
};
use crate::mesh::LineVertex;
use crate::raycast::Ray;
use crate::space::{PackedLight, Space};
use crate::universe::URef;
/// Find the first selectable block the ray strikes and express the result in a [`Cursor`]
/// value, or [`None`] if nothing was struck within the distance limit.
pub fn cursor_raycast(
    mut ray: Ray,
    space_ref: &URef<Space>,
    maximum_distance: FreeCoordinate,
) -> Option<Cursor> {
    loop {}
}
/// Data collected by [`cursor_raycast`] about the blocks struck by the ray; intended to be
/// sufficient for various player interactions with blocks.
///
/// TODO: Should carry information about both the struck and preceding cubes.
#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    /// The space the selected cube is in.
    space: URef<Space>,
    /// The face that the cursor ray entered the cube via.
    ///
    /// Note that this is not necessarily the same as “the face of the block” in the case
    /// where the block occupies less than the full volume.
    face_entered: Face7,
    /// Intersection point where the ray entered the cube.
    point_entered: Point3<FreeCoordinate>,
    /// Distance from ray origin (viewpoint) to `point_entered`.
    distance_to_point: FreeCoordinate,
    /// Data about the cube the cursor selected/hit.
    hit: CubeSnapshot,
    /// Data about the cube the cursor ray was in before it hit [`Self::hit`],
    /// if there was one, or `None` if the cursor ray started in the cube it hit.
    preceding: Option<CubeSnapshot>,
}
/// Snapshot of the contents of one cube of a [`Space`], independent of the [`Space`].
///
/// TODO: Can we find a cleaner name for this class?
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub struct CubeSnapshot {
    pub position: GridPoint,
    pub block: Block,
    pub evaluated: EvaluatedBlock,
    pub light: PackedLight,
}
impl Cursor {
    /// The space the selected cube is in.
    #[inline]
    pub fn space(&self) -> &URef<Space> {
        loop {}
    }
    /// Which cube of the space that the cursor ray selected/hit.
    pub fn cube(&self) -> GridPoint {
        loop {}
    }
    /// The cube the ray passed through immediately before the selected cube.
    ///
    /// This may be the same cube if the ray started there.
    pub fn preceding_cube(&self) -> GridPoint {
        loop {}
    }
    /// Which face of the block the cursor ray selected/hit.
    ///
    /// This is currently defined as the face of the *cube* that the ray entered, but
    /// that is planned to be revised to a more block-shape-sensitive definition.
    ///
    /// Will be [`Face7::Within`] if the ray started in the same cube.
    pub fn face_selected(&self) -> Face7 {
        loop {}
    }
    /// Returns data about the cube the cursor selected/hit.
    #[inline]
    pub fn hit(&self) -> &CubeSnapshot {
        loop {}
    }
}
impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// TODO: This implementation exists because it was convenient to support drawing;
/// eventually we will probably want cursor rendering to be its own more elaborate
/// thing.
impl Geometry for Cursor {
    type Coord = GridCoordinate;
    /// Not implemented for [`Cursor`].
    fn translate(self, _offset: GridVector) -> Self {
        loop {}
    }
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<LineVertex>,
    {
        loop {}
    }
}
