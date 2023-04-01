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
pub(crate) struct Move {}
impl universe::VisitRefs for Move {
    fn visit_refs(&self, _visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
