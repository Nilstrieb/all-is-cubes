use crate::block::{self, Block, MinEval, Modifier};
use crate::math::Face6;
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
impl universe::VisitRefs for Move {
    fn visit_refs(&self, _visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
