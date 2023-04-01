//! That which contains many blocks.
#[cfg(doc)]
use crate::character::Character;

mod space_txn;
/// Container for [`Block`]s arranged in three-dimensional space. The main “game world”
/// data structure.
pub struct Space {}
/// Number used to identify distinct blocks within a [`Space`].
pub(crate) type BlockIndex = u16;
impl crate::behavior::BehaviorHost for Space {
    type Attachment = SpaceBehaviorAttachment;
}
/// Description of where in a [`Space`] a [`Behavior<Space>`](crate::behavior::Behavior)
/// exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpaceBehaviorAttachment {}
