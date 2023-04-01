//! That which contains many blocks.

#[cfg(doc)]
use crate::character::Character;

use crate::math::{GridAab};

use std::fmt;
mod light;

mod space_txn;
/// Container for [`Block`]s arranged in three-dimensional space. The main “game world”
/// data structure.
pub struct Space {}
impl fmt::Debug for Space {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Number used to identify distinct blocks within a [`Space`].
pub(crate) type BlockIndex = u16;
impl crate::behavior::BehaviorHost for Space {
    type Attachment = SpaceBehaviorAttachment;
}
/// Description of where in a [`Space`] a [`Behavior<Space>`](crate::behavior::Behavior)
/// exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpaceBehaviorAttachment {}
/// Description of a change to a [`Space`] for use in listeners.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum SpaceChange {
    /// The block at the given location was replaced.
    Block(),
    /// The light level value at the given location changed.
    Lighting(),
    /// The given block index number was reassigned and now refers to a different
    /// [`Block`] value.
    Number(),
    /// The definition of the block referred to by the given block index number was
    /// changed; the result of [`Space::get_evaluated`] may differ.
    BlockValue(),
    /// Equivalent to [`SpaceChange::Block`] for every cube and [`SpaceChange::Number`]
    /// for every index.
    EveryBlock,
}
