//! [`Inventory`] for storing items.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::num::NonZeroU16;
use std::sync::Arc;
use crate::block::Block;
use crate::character::{Character, Cursor};
use crate::inv::{Icons, Tool, ToolError};
use crate::linking::BlockProvider;
use crate::transaction::{
    CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict,
};
use crate::universe::{RefVisitor, URef, UniverseTransaction, VisitRefs};
/// A collection of [`Tool`]s (items).
///
/// Note that unlike many other game objects in `all_is_cubes`, an `Inventory` does not
/// deliver change notifications. Instead, this is the responsibility of the `Inventory`'s
/// owner; its operations produce [`InventoryChange`]s (sometimes indirectly via
/// [`InventoryTransaction`]'s output) which the owner is responsible for forwarding
/// appropriately. This design choice allows an [`Inventory`] to be placed inside
/// other objects directly rather than via [`URef`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Inventory {
    /// TODO: This probably shouldn't be public forever.
    pub slots: Vec<Slot>,
}
impl Inventory {
    /// Construct an [`Inventory`] with the specified number of slots.
    ///
    /// Ordinary user actions cannot change the number of slots.
    pub fn new(size: usize) -> Self {
        loop {}
    }
    /// TODO: temporary interface, reevaluate design
    pub(crate) fn from_slots(mut items: Vec<Slot>) -> Self {
        loop {}
    }
    /// Use a tool stored in this inventory.
    ///
    /// `character` must be the character containing the inventory. TODO: Bad API
    pub fn use_tool(
        &self,
        cursor: Option<&Cursor>,
        character: URef<Character>,
        slot_index: usize,
    ) -> Result<UniverseTransaction, ToolError> {
        loop {}
    }
    /// Returns the total count of the given item in this inventory.
    ///
    /// Note on numeric range: this can overflow if the inventory has over 65537 slots.
    /// Let's not do that.
    ///
    /// TODO: Added for tests; is this generally useful?
    #[cfg(test)]
    pub(crate) fn count_of(&self, item: &Tool) -> u32 {
        loop {}
    }
}
impl VisitRefs for Inventory {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
/// The direct child of [`Inventory`]; a container for any number of identical [`Tool`]s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Slot {
    /// Slot contains nothing.
    Empty,
    /// Slot contains one or more of the given [`Tool`].
    Stack(NonZeroU16, Tool),
}
impl Slot {
    const COUNT_ONE: NonZeroU16 = { loop {} };
    /// Construct a [`Slot`] containing `count` copies of `tool`.
    ///
    /// If `count` is zero, the `tool` will be ignored.
    pub fn stack(count: u16, tool: Tool) -> Self {
        loop {}
    }
    /// Temporary const version of [`<Slot as From<Tool>>::from`].
    #[doc(hidden)]
    pub const fn one(tool: Tool) -> Self {
        loop {}
    }
    /// Returns the icon to use for this tool in the user interface.
    ///
    /// Note that this is _not_ the same as the block that a [`Tool::Block`] places.
    pub fn icon<'a>(&'a self, predefined: &'a BlockProvider<Icons>) -> Cow<'a, Block> {
        loop {}
    }
    /// Returns the count of items in this slot.
    pub fn count(&self) -> u16 {
        loop {}
    }
    /// If the given tool is in this slot, return the count thereof.
    ///
    /// TODO: Added for tests; is this generally useful?
    #[cfg(test)]
    pub(crate) fn count_of(&self, item: &Tool) -> u16 {
        loop {}
    }
    /// Moves as many items as possible from `self` to `destination` while obeying item
    /// stacking rules.
    ///
    /// Does nothing if `self` and `destination` contain different items.
    ///
    /// Returns whether anything was moved.
    fn unload_to(&mut self, destination: &mut Self) -> bool {
        loop {}
    }
}
impl From<Tool> for Slot {
    fn from(tool: Tool) -> Self {
        loop {}
    }
}
impl From<Option<Tool>> for Slot {
    fn from(tool: Option<Tool>) -> Self {
        loop {}
    }
}
impl VisitRefs for Slot {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
/// Specifies a limit on the number of a particular item that should be combined in a
/// single [`Slot`].
///
/// Each value of this enum is currently equivalent to a particular number, but (TODO:)
/// in the future, it may be possible for inventories or universes to specify a normal
/// stack size and specific deviations from it.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum StackLimit {
    One,
    Standard,
}
impl StackLimit {
    /// TODO: This is not public because we don't know what environment parameters it
    /// should need yet.
    pub(crate) fn get(self) -> u16 {
        loop {}
    }
}
/// Transaction type for [`Inventory`].
///
/// The output type is the change notification which should be passed on after commit,
/// if any change is made.
#[derive(Clone, Debug, Default, PartialEq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[must_use]
pub struct InventoryTransaction {
    replace: BTreeMap<usize, (Slot, Slot)>,
    insert: Vec<Slot>,
}
impl InventoryTransaction {
    /// Transaction to insert items/stacks into an inventory, which will fail if there is
    /// not sufficient space.
    pub fn insert<S: Into<Slot>, I: IntoIterator<Item = S>>(stacks: I) -> Self {
        loop {}
    }
    /// Transaction to replace the contents of an existing slot in an inventory, which
    /// will fail if the existing slot is not as expected.
    ///
    /// TODO: Right now, this requires an exact match. In the future, we should be able
    /// to compose multiple modifications like "add 1 item to stack" ×2 into "add 2 items".
    pub fn replace(slot: usize, old: Slot, new: Slot) -> Self {
        loop {}
    }
}
impl Transaction<Inventory> for InventoryTransaction {
    type CommitCheck = Option<InventoryCheck>;
    type Output = InventoryChange;
    fn check(
        &self,
        inventory: &Inventory,
    ) -> Result<Self::CommitCheck, PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        inventory: &mut Inventory,
        check: Self::CommitCheck,
        outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), CommitError> {
        loop {}
    }
}
impl Merge for InventoryTransaction {
    type MergeCheck = ();
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, other: Self, (): Self::MergeCheck) -> Self {
        loop {}
    }
}
/// Implementation type for [`InventoryTransaction::CommitCheck`].
#[derive(Debug)]
pub struct InventoryCheck {
    new: Vec<Slot>,
    change: InventoryChange,
}
/// Description of a change to an [`Inventory`] for use in listeners.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct InventoryChange {
    /// Which slots of the inventory have been changed.
    pub slots: Arc<[usize]>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::make_some_blocks;
    use crate::math::Rgba;
    use crate::transaction::TransactionTester;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    #[test]
    fn txn_identity_no_notification() {
        loop {}
    }
    #[test]
    fn txn_insert_empty_list() {
        loop {}
    }
    #[test]
    fn txn_insert_filtered_empty() {
        loop {}
    }
    #[test]
    fn txn_insert_success() {
        loop {}
    }
    #[test]
    fn txn_insert_no_space() {
        loop {}
    }
    #[test]
    fn txn_insert_into_existing_stack() {
        loop {}
    }
    #[test]
    fn txn_systematic() {
        loop {}
    }
    #[test]
    fn slot_unload_systematic() {
        loop {}
    }
}
