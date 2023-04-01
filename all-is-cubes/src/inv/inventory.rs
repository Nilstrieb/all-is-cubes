//! [`Inventory`] for storing items.






use crate::transaction::{
    CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict,
};

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
pub struct Inventory {}
/// The direct child of [`Inventory`]; a container for any number of identical [`Tool`]s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum Slot {
    /// Slot contains nothing.
    Empty,
    /// Slot contains one or more of the given [`Tool`].
    Stack(),
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
/// Transaction type for [`Inventory`].
///
/// The output type is the change notification which should be passed on after commit,
/// if any change is made.
#[derive(Clone, Debug, Default, PartialEq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[must_use]
pub struct InventoryTransaction {}
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
pub struct InventoryCheck {}
/// Description of a change to an [`Inventory`] for use in listeners.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct InventoryChange {}
