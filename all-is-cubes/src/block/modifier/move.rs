use cgmath::Zero;
use crate::block::{
    self, Block, BlockAttributes, BlockCollision, Evoxel, Evoxels, MinEval, Modifier,
    Resolution::R16, AIR,
};
use crate::drawing::VoxelBrush;
use crate::math::{Face6, GridAab, GridArray, GridCoordinate};
use crate::universe;
/// Data for [`Modifier::Move`]; displaces the block out of the grid, cropping it.
/// A pair of `Move`s can depict a block moving between two cubes.
///
/// # Animation
///
/// * If the `distance` is zero then the modifier will remove itself, if possible,
///   on the next tick.
/// * If the `distance` and `velocity` are such that the block is out of view and will
///   never strt being in view, the block will be replaced with [`AIR`].
///
/// (TODO: Define the conditions for “if possible”.)
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Move {
    /// The direction in which the block is displaced.
    pub direction: Face6,
    /// The distance, in 1/256ths, by which it is displaced.
    pub distance: u16,
    /// The velocity **per tick** with which the displacement is changing.
    ///
    /// TODO: "Per tick" is a bad unit.
    pub velocity: i16,
}
impl Move {
    /// TODO: make a cleaner, less internals-ish constructor
    pub fn new(direction: Face6, distance: u16, velocity: i16) -> Self {
        loop {}
    }
    /// Create a pair of [`Modifier::Move`]s to displace a block.
    /// The first goes on the block being moved and the second on the air
    /// it's moving into.
    ///
    /// TODO: This is going to need to change again in order to support
    /// moving one block in and another out at the same time.
    pub fn paired_move(direction: Face6, distance: u16, velocity: i16) -> [Modifier; 2] {
        loop {}
    }
    /// Note that `Modifier::Move` does some preprocessing to keep this simpler.
    pub(super) fn evaluate(
        &self,
        block: &Block,
        this_modifier_index: usize,
        mut input: MinEval,
        depth: u8,
    ) -> Result<MinEval, block::EvalBlockError> {
        loop {}
    }
}
impl From<Move> for block::Modifier {
    fn from(value: Move) -> Self {
        loop {}
    }
}
impl universe::VisitRefs for Move {
    fn visit_refs(&self, _visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use cgmath::EuclideanSpace;
    use crate::block::{Block, Composite, EvaluatedBlock, Evoxel, Resolution::*};
    use crate::content::make_some_blocks;
    use crate::math::{FaceMap, GridPoint, OpacityCategory, Rgba};
    use crate::space::Space;
    use crate::time::Tick;
    use crate::universe::Universe;
    use super::*;
    #[test]
    fn move_atom_block_evaluation() {
        loop {}
    }
    #[test]
    fn move_voxel_block_evaluation() {
        loop {}
    }
    /// [`Modifier::Move`] incorporates [`Modifier::Quote`] to ensure that no conflicting
    /// effects happen.
    #[test]
    fn move_also_quotes() {
        loop {}
    }
    /// Set up a `Modifier::Move`, let it run, and then allow assertions to be made about the result.
    fn move_block_test(
        direction: Face6,
        velocity: i16,
        checker: impl FnOnce(&Space, &Block),
    ) {
        loop {}
    }
    #[test]
    fn velocity_zero() {
        loop {}
    }
    #[test]
    fn velocity_slow() {
        loop {}
    }
    #[test]
    fn velocity_whole_cube_in_one_tick() {
        loop {}
    }
    /// Test [`Move`] acting within another modifier ([`Composite`]).
    #[test]
    fn move_inside_composite_destination() {
        loop {}
    }
    /// Test [`Move`] acting within the `source` position of a [`Modifier::Composite`].
    ///
    /// TODO: This is not yet implemented, but should be.
    #[test]
    fn move_inside_composite_source() {
        loop {}
    }
}
