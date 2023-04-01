//! Dynamic add-ons to game objects; we might also have called them “components”.
use crate::transaction::{self, Transaction};
use crate::universe::{VisitRefs};
use downcast_rs::{impl_downcast, Downcast};

use std::fmt::{self, Debug};

/// Dynamic add-ons to game objects; we might also have called them “components”.
/// Each behavior is owned by a “host” of type `H` which determines when the behavior
/// is invoked.
pub(crate) trait Behavior<
    H: BehaviorHost,
>: Debug + Send + Sync + Downcast + VisitRefs + 'static {}
impl_downcast!(Behavior < H > where H : BehaviorHost);
/// A type that can have attached behaviors.
pub trait BehaviorHost: transaction::Transactional + 'static {
    /// Additional data about “where” the behavior is attached to the host; what part of
    /// the host should be affected by the behavior.
    type Attachment: Debug + Clone + Eq + 'static;
}
/// Items available to a [`Behavior`] during [`Behavior::step()`].
/// Collects [`Behavior`]s and invokes them.
///
/// Note: This type is public out of necessity because it is revealed elsewhere, but its details
/// are currently subject to change.
///
/// To modify the set, use a [`BehaviorSetTransaction`].
pub struct BehaviorSet<H: BehaviorHost> {
    /// Behaviors are stored in [`Arc`] so that they can be used in transactions in ways
    /// that would otherwise require `Clone + PartialEq`.
    items: Vec<BehaviorSetEntry<H>>,
}
struct BehaviorSetEntry<H: BehaviorHost> {
    attachment: H::Attachment,
}
impl<H: BehaviorHost> Clone for BehaviorSetEntry<H> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<H: BehaviorHost> Debug for BehaviorSetEntry<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<H: BehaviorHost> PartialEq for BehaviorSetEntry<H> {
    #[allow(clippy::vtable_address_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
/// A [`Transaction`] that adds or modifies [`Behavior`]s in a [`BehaviorSet`].
#[derive(Debug)]
pub struct BehaviorSetTransaction<H: BehaviorHost> {
    /// Newly inserted behaviors.
    insert: Vec<BehaviorSetEntry<H>>,
}
#[derive(Debug)]
struct Replace<H: BehaviorHost> {
    new: BehaviorSetEntry<H>,
}
impl<H: BehaviorHost> Transaction<BehaviorSet<H>> for BehaviorSetTransaction<H> {
    type CommitCheck = ();
    type Output = transaction::NoOutput;
    #[allow(clippy::vtable_address_comparisons)]
    fn check(
        &self,
        target: &BehaviorSet<H>,
    ) -> Result<Self::CommitCheck, transaction::PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        target: &mut BehaviorSet<H>,
        (): Self::CommitCheck,
        _outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), transaction::CommitError> {
        loop {}
    }
}
impl<H: BehaviorHost> transaction::Merge for BehaviorSetTransaction<H> {
    type MergeCheck = ();
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, transaction::TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, other: Self, (): Self::MergeCheck) -> Self {
        loop {}
    }
}
impl<H: BehaviorHost> Clone for BehaviorSetTransaction<H> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<H: BehaviorHost> Default for BehaviorSetTransaction<H> {
    fn default() -> Self {
        loop {}
    }
}
impl<H: BehaviorHost> PartialEq for BehaviorSetTransaction<H> {
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
impl<H: BehaviorHost> Eq for BehaviorSetTransaction<H> {}
