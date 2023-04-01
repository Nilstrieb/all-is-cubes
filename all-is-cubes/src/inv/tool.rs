//! [`Tool`] and related.
use std::borrow::Cow;
use std::sync::Arc;
use std::{fmt, hash};
use crate::block::Block;
use crate::character::Cursor;
use crate::inv::{self, Icons, StackLimit};
use crate::linking::BlockProvider;
use crate::math::GridPoint;
use crate::universe::{RefError, RefVisitor, UniverseTransaction, VisitRefs};
/// A `Tool` is an object which a character can use to have some effect in the game,
/// such as placing or removing a block. In particular, a tool use usually corresponds
/// to a click.
///
/// Currently, `Tool`s also play the role of “inventory items”. This may change in the
/// future.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum Tool {
    /// “Click”, or “push button”, or generally “activate the function of this”
    /// as opposed to editing it.
    ///
    /// This can activate an [`ActivatableRegion`](crate::space::ActivatableRegion).
    /// It may have more functions in the future.
    Activate,
    /// Delete any targeted block from the space.
    RemoveBlock {},
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
    Jetpack {},
    /// A tool which calls an arbitrary function.
    ExternalAction {},
}
/// Ways that a tool can fail.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ToolError {
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
