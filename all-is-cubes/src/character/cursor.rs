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
pub(crate) fn cursor_raycast(
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
pub(crate) struct Cursor {}
/// Snapshot of the contents of one cube of a [`Space`], independent of the [`Space`].
///
/// TODO: Can we find a cleaner name for this class?
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub(crate) struct CubeSnapshot {}
impl Cursor {
    /// The space the selected cube is in.
    #[inline]
    pub(crate) fn space(&self) -> &URef<Space> {
        loop {}
    }
    /// Which cube of the space that the cursor ray selected/hit.
    pub(crate) fn cube(&self) -> GridPoint {
        loop {}
    }
    /// The cube the ray passed through immediately before the selected cube.
    ///
    /// This may be the same cube if the ray started there.
    pub(crate) fn preceding_cube(&self) -> GridPoint {
        loop {}
    }
    /// Which face of the block the cursor ray selected/hit.
    ///
    /// This is currently defined as the face of the *cube* that the ray entered, but
    /// that is planned to be revised to a more block-shape-sensitive definition.
    ///
    /// Will be [`Face7::Within`] if the ray started in the same cube.
    pub(crate) fn face_selected(&self) -> Face7 {
        loop {}
    }
    /// Returns data about the cube the cursor selected/hit.
    #[inline]
    pub(crate) fn hit(&self) -> &CubeSnapshot {
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
