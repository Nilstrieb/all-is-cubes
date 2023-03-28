//! [`Tool`] and related.
use std::borrow::Cow;
use std::sync::Arc;
use std::{fmt, hash};
use crate::block::{self, Block, Primitive, RotationPlacementRule, AIR};
use crate::character::{Character, CharacterTransaction, Cursor};
use crate::fluff::Fluff;
use crate::inv::{self, Icons, InventoryTransaction, StackLimit};
use crate::linking::BlockProvider;
use crate::math::{Face6, GridPoint, GridRotation};
use crate::space::{Space, SpaceTransaction};
use crate::transaction::{Merge, Transaction};
use crate::universe::{RefError, RefVisitor, URef, UniverseTransaction, VisitRefs};
/// A `Tool` is an object which a character can use to have some effect in the game,
/// such as placing or removing a block. In particular, a tool use usually corresponds
/// to a click.
///
/// Currently, `Tool`s also play the role of “inventory items”. This may change in the
/// future.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Tool {
    /// “Click”, or “push button”, or generally “activate the function of this”
    /// as opposed to editing it.
    ///
    /// This can activate an [`ActivatableRegion`](crate::space::ActivatableRegion).
    /// It may have more functions in the future.
    Activate,
    /// Delete any targeted block from the space.
    RemoveBlock {
        /// If true, move it to inventory. If false, discard it entirely.
        keep: bool,
    },
    /// Move the given block out of inventory (consuming this tool) into the targeted
    /// empty space.
    Block(Block),
    /// Places copies of the given block in targeted empty space. Infinite uses.
    InfiniteBlocks(Block),
    /// Copy block from space to inventory.
    CopyFromSpace,
    /// Teleport into a block's space for editing.
    ///
    /// TODO: This is not yet actually implemented.
    EditBlock,
    /// Push targeted block into adjacent cube.
    PushPull,
    /// Allows flight.
    ///
    /// TODO: This should probably be a feature a tool can have rather than a
    /// single-purpose item, but we don't yet have a plan for programmable items.
    Jetpack {
        /// Actually currently flying?
        active: bool,
    },
    /// A tool which calls an arbitrary function.
    ExternalAction {
        /// Function that will be called when the tool is activated.
        function: EphemeralOpaque<dyn Fn(&ToolInput) + Send + Sync>,
        /// Icon for the tool.
        icon: Block,
    },
}
impl Tool {
    #[allow(dead_code)]
    pub(crate) fn external_action<F: Fn(&ToolInput) + Send + Sync + 'static>(
        icon: Block,
        function: F,
    ) -> Self {
        loop {}
    }
    /// Computes the effect of using the tool.
    ///
    /// The effect consists of both mutations to `self` and a [`UniverseTransaction`].
    /// If the result is `None` then the tool is deleted.
    /// If the transaction does not succeed, the original `Tool` value should be kept.
    ///
    /// TODO: Return type is inelegant
    pub fn use_tool(
        self,
        input: &ToolInput,
    ) -> Result<(Option<Self>, UniverseTransaction), ToolError> {
        loop {}
    }
    /// As [`Self::use_tool`], except that it does not allow the tool to modify itself.
    ///
    /// This operation is used for special cases where an action is expressed by a tool
    /// but the tool is not a “game item”.
    pub fn use_immutable_tool(
        &self,
        input: &ToolInput,
    ) -> Result<UniverseTransaction, ToolError> {
        loop {}
    }
    /// Return a block to use as an icon for this tool. For tools that place blocks, has the
    /// same appearance as the block to be placed. The display name of the block should be
    /// the display name of the tool.
    ///
    /// TODO (API instability): Eventually we will probably want additional decorations
    /// that probably should not need to be painted into the block itself.
    pub fn icon<'a>(&'a self, predefined: &'a BlockProvider<Icons>) -> Cow<'a, Block> {
        loop {}
    }
    /// Specifies a limit on the number of this item that should be combined in a single
    /// [`Slot`].
    pub(crate) fn stack_limit(&self) -> StackLimit {
        loop {}
    }
}
impl VisitRefs for Tool {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
/// Resources available to a `Tool` to perform its function.
///
/// This is intended to provide future extensibility compared to having a complex
/// parameter list for `Tool::use_tool`.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct ToolInput {
    /// Cursor identifying block(s) to act on. If [`None`] then the tool was used while
    /// pointing at nothing or by an agent without an ability to aim.
    pub cursor: Option<Cursor>,
    /// Character that is using the tool.
    ///
    /// TODO: We want to be able to express “inventory host”, not just specifically Character (but there aren't any other examples).
    pub character: Option<URef<Character>>,
}
impl ToolInput {
    /// Generic handler for a tool that replaces one cube.
    ///
    /// TODO: This should probably be replaced with a `Transaction` whose failure
    /// is translated into the `ToolError`, since this code is basically doing
    /// `SpaceTransaction::check` anyway.
    fn set_cube(
        &self,
        cube: GridPoint,
        old_block: Block,
        new_block: Block,
    ) -> Result<UniverseTransaction, ToolError> {
        loop {}
    }
    /// As [`Self::set_cube`] but also applying rotation (or other transformations
    /// in the future) specified by the block's attributes
    fn place_block(
        &self,
        cursor: &Cursor,
        old_block: Block,
        new_block: Block,
    ) -> Result<UniverseTransaction, ToolError> {
        loop {}
    }
    /// Returns a [`Cursor`] indicating what blocks the tool should act on, if it is
    /// a sort of tool that acts on blocks. If there is no [`Cursor`], because of aim
    /// or because of being used in a context where there cannot be any aiming, returns
    /// [`Err(ToolError::NothingSelected)`](ToolError::NothingSelected) for convenient
    /// propagation.
    pub fn cursor(&self) -> Result<&Cursor, ToolError> {
        loop {}
    }
    /// Add the provided items to the inventory from which the tool was used.
    pub fn produce_items<S: Into<inv::Slot>, I: IntoIterator<Item = S>>(
        &self,
        items: I,
    ) -> Result<UniverseTransaction, ToolError> {
        loop {}
    }
}
/// Ways that a tool can fail.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum ToolError {
    /// There was no tool to use (empty inventory slot, nonexistent slot, nonexistent inventory…).
    #[error("no tool")]
    NoTool,
    /// The tool cannot currently be used or does not apply to the target.
    #[error("does not apply")]
    NotUsable,
    /// Cannot place a block or similar because there's a block occupying the space.
    #[error("there's something in the way")]
    Obstacle,
    /// The tool requires a target cube and none was present.
    #[error("nothing is selected")]
    NothingSelected,
    /// The space to be operated on could not be accessed.
    #[error("error accessing space: {0}")]
    SpaceRef(#[from] RefError),
    /// An error occurred while executing the effects of the tool.
    /// TODO: Improve this along with [`Transaction`] error types.
    #[error("unexpected error: {0}")]
    Internal(String),
}
impl ToolError {
    /// Return [`Fluff`] to accompany this error.
    ///
    /// TODO: This should have spatial information (located at the cursor target or the
    /// character's "hand" or other).
    pub fn fluff(&self) -> impl Iterator<Item = Fluff> {
        std::iter::once(Fluff::Beep)
    }
}
/// A wrapper around a value which cannot be printed or serialized,
/// used primarily to allow external functions to be called from objects
/// within a [`Universe`](crate::universe::Universe).
///
/// TODO: relocate this type once we figure out where it belongs.
/// TODO: Probably they should be their own kind of `UniverseMember`, so that they can
/// be reattached in the future.
pub struct EphemeralOpaque<T: ?Sized>(pub(crate) Option<Arc<T>>);
impl<T: ?Sized> EphemeralOpaque<T> {
    /// Get a reference to the value if it still exists.
    pub fn try_ref(&self) -> Option<&T> {
        loop {}
    }
}
impl<T: ?Sized> From<Arc<T>> for EphemeralOpaque<T> {
    fn from(contents: Arc<T>) -> Self {
        loop {}
    }
}
impl<T: ?Sized> fmt::Debug for EphemeralOpaque<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<T: ?Sized> PartialEq for EphemeralOpaque<T> {
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
impl<T: ?Sized> Eq for EphemeralOpaque<T> {}
impl<T: ?Sized> Clone for EphemeralOpaque<T> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<T: ?Sized> hash::Hash for EphemeralOpaque<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a, T: arbitrary::Arbitrary<'a>> arbitrary::Arbitrary<'a> for EphemeralOpaque<T> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Primitive;
    use crate::character::cursor_raycast;
    use crate::content::{make_some_blocks, make_some_voxel_blocks};
    use crate::inv::Slot;
    use crate::math::{FreeCoordinate, GridRotation};
    use crate::raycast::Ray;
    use crate::raytracer::print_space;
    use crate::space::Space;
    use crate::transaction;
    use crate::universe::{UBorrow, URef, Universe};
    use crate::util::YieldProgress;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    #[derive(Debug)]
    struct ToolTester {
        universe: Universe,
        character_ref: URef<Character>,
        space_ref: URef<Space>,
    }
    impl ToolTester {
        /// The provided function should modify the space to contain the blocks to operate on,
        /// given a cursor ray along the line of cubes from the origin in the +X direction.
        fn new<F: FnOnce(&mut Space)>(f: F) -> Self {
            loop {}
        }
        fn input(&self) -> ToolInput {
            loop {}
        }
        fn equip_and_use_tool(
            &self,
            stack: impl Into<Slot>,
        ) -> Result<UniverseTransaction, ToolError> {
            loop {}
        }
        /// As `equip_and_use_tool`, but also commit the transaction.
        /// TODO: Needs a better error return type (requires Transaction to do so).
        fn equip_use_commit(
            &mut self,
            stack: impl Into<Slot>,
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            loop {}
        }
        fn space(&self) -> UBorrow<Space> {
            loop {}
        }
        fn space_ref(&self) -> &URef<Space> {
            loop {}
        }
        fn character(&self) -> UBorrow<Character> {
            loop {}
        }
    }
    async fn dummy_icons() -> BlockProvider<Icons> {
        loop {}
    }
    #[tokio::test]
    async fn icon_activate() {
        loop {}
    }
    #[test]
    fn use_activate() {
        loop {}
    }
    #[tokio::test]
    async fn icon_remove_block() {
        loop {}
    }
    #[test]
    fn use_remove_block() {
        loop {}
    }
    #[test]
    fn use_remove_block_without_target() {
        loop {}
    }
    #[tokio::test]
    async fn icon_place_block() {
        loop {}
    }
    #[test]
    fn use_block() {
        loop {}
    }
    /// TODO: Expand this test to exhaustively test all rotation placement rules?
    #[test]
    fn use_block_automatic_rotation() {
        loop {}
    }
    /// Note: This is more of a test of [`Inventory`] and [`Slot`] stack management
    /// than the tool.
    #[test]
    fn use_block_stack_decrements() {
        loop {}
    }
    #[test]
    fn use_block_with_obstacle() {
        loop {}
    }
    #[test]
    fn use_block_without_target() {
        loop {}
    }
    #[test]
    fn use_copy_from_space() {
        loop {}
    }
    #[test]
    fn use_external_action() {
        loop {}
    }
}
