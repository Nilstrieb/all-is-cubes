use std::mem;
use ordered_float::NotNan;
use crate::block::{
    self, Block, BlockCollision, Evoxel, Evoxels, MinEval, Modifier, Resolution::R1, AIR,
};
use crate::math::{GridAab, GridArray, GridCoordinate, GridRotation, Rgba};
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
pub struct Composite {
    /// The “source” input to the compositing operator.
    /// (The “destination” input is the block this modifier is attached to.)
    pub source: Block,
    /// The compositing operator used to combine the source and destination blocks.
    pub operator: CompositeOperator,
    /// Swap the roles of “source” and “destination” for the [`operator`](Self::operator).
    pub reverse: bool,
    /// Whether the block should come apart into its components when removed from its place.
    pub disassemblable: bool,
}
impl Composite {
    /// Construct a new [`Composite`] modifier with the given source and operator, and
    /// `reverse: false`.
    pub fn new(source: Block, operator: CompositeOperator) -> Self {
        loop {}
    }
    /// Set the disassemblable flag to true.
    ///
    /// This will allow the composite to be taken apart by player action.
    /// TODO: explain further
    #[must_use]
    pub fn with_disassemblable(mut self) -> Self {
        loop {}
    }
    /// Compose `self` and `destination`, except that:
    ///
    /// * If `destination` is [`AIR`], then the `self.source` block will be returned.
    /// * If `self.source` is [`AIR`], then `destination` will be returned.
    /// * If `destination` has a rotation modifier, it will be rearranged to be last.
    ///   (In this way, there won't be any unequal-but-equivalent blocks generated due
    ///   to rotation.)
    ///
    /// This operation is of limited use and is designed for world-generation purposes, not
    /// player action (since it has no restrictions on what it can compose). Its particular
    /// use is to build corner joint blocks.
    ///
    /// TODO: Generalize this so it has a filter on which things should be composed,
    /// replaced, or left unchanged (failure).
    ///
    /// TODO: Figure out a way to express "sorting order" rules for swapping self and
    /// destination, because for corner joints we don't care which is on top but we want
    /// there to be only one kind of corner block, not two depending on operation order.
    pub fn compose_or_replace(mut self, mut destination: Block) -> Block {
        loop {}
    }
    /// Use [`Composite::compose_or_replace()`] repeatedly to assemble a block from parts.
    pub fn stack(
        destination: Block,
        parts: impl IntoIterator<Item = Composite>,
    ) -> Block {
        loop {}
    }
    /// Called by [`Modifier::evaluate`].
    pub(super) fn evaluate(
        &self,
        mut dst_evaluated: MinEval,
        depth: u8,
    ) -> Result<MinEval, block::EvalBlockError> {
        loop {}
    }
    /// Called by [`Modifier::unspecialize()`].
    pub(super) fn unspecialize(
        &self,
        entire_block: &Block,
    ) -> block::ModifierUnspecialize {
        loop {}
    }
}
impl From<Composite> for Modifier {
    fn from(value: Composite) -> Self {
        loop {}
    }
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
pub enum CompositeOperator {
    /// Porter-Duff “over”. If both source and destination are opaque, the source is taken;
    /// otherwise the destination is taken.
    Over,
}
impl CompositeOperator {
    fn blend_color(&self, source: Rgba, destination: Rgba) -> Rgba {
        loop {}
    }
    fn blend_evoxel(&self, src_ev: Evoxel, dst_ev: Evoxel) -> Evoxel {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::make_some_blocks;
    use pretty_assertions::assert_eq;
    #[test]
    fn composite_silly_floats() {
        loop {}
    }
    #[test]
    fn compose_or_replace_source_is_air() {
        loop {}
    }
    #[test]
    fn compose_or_replace_destination_is_air() {
        loop {}
    }
    #[test]
    fn unspecialize_no() {
        loop {}
    }
    #[test]
    fn unspecialize_yes() {
        loop {}
    }
}
