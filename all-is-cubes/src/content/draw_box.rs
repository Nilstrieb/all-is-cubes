use crate::block::{Block};
use crate::math::{GridAab};
use crate::space;
/// Set of blocks used to draw 3D boxes of any size, allowing corners and edges to have
/// different blocks than faces and the interior.
///
/// This can be considered a 3-dimensional analogue of the “9-patch image” concept,
/// or as a voxel “tile set” (but one with no “inside corner” pieces).
#[derive(Clone, Debug)]
pub struct BoxStyle {
    /// Contains every type of block that can appear in a box.
    /// 3D array `.parts[x][y][z]` indexed by two bitflags:
    /// * Bit 0 (1) = the block is on an lower edge.
    /// * Bit 1 (2) = the block is on an upper edge.
    /// So the sequence on each axis is [interior, lower, upper, lower & upper].
    parts: [[[Option<Block>; 4]; 4]; 4],
}
impl BoxStyle {
    /// Construct a `BoxStyle` that uses the given blocks for
    /// interior, faces, edges, and corners, never rotated.
    pub fn from_geometric_categories(
        interior: Option<Block>,
        face: Option<Block>,
        edge: Option<Block>,
        corner: Option<Block>,
    ) -> Self {
        loop {}
    }
    /// Construct a `BoxStyle` from block types for walls, floor, ceiling, and corners.
    /// The one-block corners will be left blank.
    /// without anything to handle corners of one-block walls.
    ///
    /// `corner` should be oriented so as to join up with +X and +Z walls, and will be
    /// rotated about the Y axis to suit other corners.
    ///
    /// TODO: this is a quick kludge to migrate some other worldgen code, and doesn't
    /// have a necessarily logical combination of parameters or the best rotation choices.
    ///
    /// TODO: wall blocks should be rotated
    #[doc(hidden)]
    pub fn from_whole_blocks_for_walls(
        wall: Option<Block>,
        floor: Option<Block>,
        ceiling: Option<Block>,
        corner: Option<Block>,
    ) -> Self {
        loop {}
    }
    /// Construct a `BoxStyle` made up of one type of corner block and one type of edge
    /// block.
    ///
    /// * `corner_block` will be composited with the line sections at all eight
    ///   corners of the box. It should be oriented as the `lower_bounds` corner of the box;
    ///   the other seven corners will be mirrored across the relevant axis.
    /// * `line_section_block` should be a block which is a line segment at the origin and
    ///   extending in the [`+Z`](crate::math::Face6::PZ) direction.
    ///   It should be symmetric about the X-Y=0 plane, and will be rotated and mirrored
    ///   to make the other forms.
    pub fn from_composited_corner_and_edge(
        corner_block: Block,
        line_section_block: Block,
    ) -> Self {
        #![allow(non_snake_case)]
        loop {}
    }
    #[must_use]
    pub fn with_interior(mut self, interior: Option<Block>) -> Self {
        loop {}
    }
    /// Returns a transaction that places an axis-aligned box of blocks from this [`BoxStyle`],
    /// within and up to the bounds of the given [`GridAab`].
    ///
    /// * The lines will lie just inside of `bounds`.
    pub fn create_box(&self, bounds: GridAab) -> space::SpaceTransaction {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use crate::math::Rgba;
    use super::*;
    /// Test that funny box sizes (e.g. zero in some axis) don't cause panics.
    #[test]
    fn box_smoke_test() {
        loop {}
    }
}
