use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use crate::block::BlockDef;
use crate::character::Character;
use crate::space::Space;
use crate::transaction::{
    self, CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict,
    Transactional,
};
use crate::universe::{
    AnyURef, Name, UBorrowMutImpl, URef, URefErased as _, Universe, UniverseId,
};
/// Conversion from concrete transaction types to [`UniverseTransaction`].
///
/// Most code should be able to call [`Transaction::bind`] rather than mentioning this
/// trait at all; it is an implementation detail of the conversion that unfortunately
/// cannot be hidden.
pub trait UTransactional: Transactional + 'static
where
    Self: Sized,
{
    /// Specify the target of the transaction as a [`URef`], and erase its type,
    /// so that it can be combined with other transactions in the same universe.
    ///
    /// This is also available as [`Transaction::bind`].
    fn bind(target: URef<Self>, transaction: Self::Transaction) -> UniverseTransaction;
}
/// Pair of a transaction and a [`URef`] to its target.
///
/// [`AnyTransaction`] is a singly-typed wrapper around this.
///
/// This type is public out of necessity due to appearing in trait bounds; you should not
/// need to use it.
///
/// TODO: Better name.
#[derive(Debug, Eq)]
struct TransactionInUniverse<O: Transactional + 'static> {
    transaction: O::Transaction,
}
impl<O> Transaction<()> for TransactionInUniverse<O>
where
    O: Transactional + 'static,
{
    type CommitCheck = (
        UBorrowMutImpl<O>,
        <O::Transaction as Transaction<O>>::CommitCheck,
    );
    type Output = <O::Transaction as Transaction<O>>::Output;
    fn check(
        &self,
        _dummy_target: &(),
    ) -> Result<Self::CommitCheck, PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        _dummy_target: &mut (),
        (mut borrow, check): Self::CommitCheck,
        outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), CommitError> {
        loop {}
    }
}
impl<O> Merge for TransactionInUniverse<O>
where
    O: Transactional + 'static,
{
    type MergeCheck = <O::Transaction as Merge>::MergeCheck;
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, other: Self, check: Self::MergeCheck) -> Self {
        loop {}
    }
}
/// Manual implementation to avoid `O: Clone` bound.
impl<O> Clone for TransactionInUniverse<O>
where
    O: Transactional,
    O::Transaction: Clone,
{
    fn clone(&self) -> Self {
        loop {}
    }
}
/// Manual implementation to avoid `O: PartialEq` bound.
impl<O: Transactional> PartialEq for TransactionInUniverse<O>
where
    O::Transaction: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
/// Polymorphic container for transactions in a [`UniverseTransaction`].
#[derive(Clone, Default, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
enum AnyTransaction {
    #[default]
    Noop,
    BlockDef(),
    Character(),
    Space(),
}
/// Not-an-associated-type alias for check values produced by [`AnyTransaction`].
/// TODO: Make this a newtype struct since we're bothering to name it.
type AnyTransactionCheck = Box<dyn Any>;
impl AnyTransaction {
    fn target_name(&self) -> Option<Name> {
        loop {}
    }
    /// Returns the transaction out of the [`TransactionInUniverse`] wrapper.
    fn transaction_as_debug(&self) -> &dyn Debug {
        loop {}
    }
    fn universe_id(&self) -> Option<UniverseId> {
        loop {}
    }
}
/// Hide the wrapper type entirely since its type is determined entirely by its contents.
impl Debug for AnyTransaction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// A [`Transaction`] which operates on one or more objects in a [`Universe`]
/// simultaneously.
///
/// Construct this by calling [`Transaction::bind`] on other transaction types
/// and combine them into larger transactions with [`Merge::merge`].
#[derive(Clone, Default, PartialEq)]
#[must_use]
pub struct UniverseTransaction {}
#[doc(hidden)]
#[derive(Debug)]
pub struct UniverseMergeCheck(HashMap<Name, MemberMergeCheck>);
#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct UniverseCommitCheck {}
impl UniverseTransaction {
    /// If this transaction contains any operations that are on a specific member of a
    /// universe, then returns the ID of that universe.
    pub(crate) fn universe_id(&self) -> Option<UniverseId> {
        loop {}
    }
}
impl From<AnyTransaction> for UniverseTransaction {
    fn from(transaction: AnyTransaction) -> Self {
        loop {}
    }
}
impl Merge for UniverseTransaction {
    type MergeCheck = UniverseMergeCheck;
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(
        self,
        other: Self,
        UniverseMergeCheck(check): Self::MergeCheck,
    ) -> Self
    where
        Self: Sized,
    {
        loop {}
    }
}
/// This formatting is chosen to be similar to [`Universe`]'s.
impl Debug for UniverseTransaction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Transaction for anything that can be done to a single member of a [`Universe`].
///
/// Note: This does not implement [`Transaction`] because it needs to refer to an
/// _entry_ in a Universe. We could kludge around that by having it take the Universe
/// and embed the Name, but that's unnecessary.
#[derive(Clone, Debug, Default, PartialEq)]
enum MemberTxn {
    /// Mergeable types are required to have a no-operation [`Default`] value,
    /// though this shouldn't come up much.
    #[default]
    Noop,
    /// Apply given transaction to the existing value.
    Modify(),
    /// Insert the provided [pending](URef::new_pending) [`URef`] in the universe.
    ///
    /// Note: This transaction can only succeed once, since after the first time it will
    /// no longer be pending.
    Insert(),
    /// Delete this member from the universe.
    ///
    /// See [`UniverseTransaction::delete()`] for full documentation.
    Delete,
}
#[derive(Debug)]
struct MemberMergeCheck();
#[derive(Debug)]
struct MemberCommitCheck();
