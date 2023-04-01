use crate::block::{self, Block, Evoxel, MinEval};
use crate::math::Rgba;
use crate::universe;
/// Data for [`Modifier::Composite`], describing how to combine the voxels of another
/// block with the original one.
///
/// TODO: This modifier is not complete. It needs additional rules, particularly about combining
/// the blocks' attributes (right now it always chooses the destination), and the ability to
/// systematically combine or break apart the composite when applicable.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) struct Composite {
    /// The “source” input to the compositing operator.
    /// (The “destination” input is the block this modifier is attached to.)
    pub(crate) source: Block,
    /// The compositing operator used to combine the source and destination blocks.
    pub(crate) operator: CompositeOperator,
    /// Swap the roles of “source” and “destination” for the [`operator`](Self::operator).
    pub(crate) reverse: bool,
    /// Whether the block should come apart into its components when removed from its place.
    pub(crate) disassemblable: bool,
}
impl universe::VisitRefs for Composite {
    fn visit_refs(&self, visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
/// Compositing operators, mostly as per Porter-Duff.
///
/// The “source” block is the [`Composite`]'s stored block, and the “destination” block
/// is the block the modifier is attached to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) enum CompositeOperator {
    /// Porter-Duff “over”. If both source and destination are opaque, the source is taken;
    /// otherwise the destination is taken.
    Over,
}
