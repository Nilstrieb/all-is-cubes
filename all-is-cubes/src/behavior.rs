//! Dynamic add-ons to game objects; we might also have called them “components”.

use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::sync::Arc;
use downcast_rs::{impl_downcast, Downcast};
use crate::time::Tick;
use crate::transaction::{self, Transaction};
use crate::universe::{UniverseTransaction, VisitRefs};
/// Dynamic add-ons to game objects; we might also have called them “components”.
/// Each behavior is owned by a “host” of type `H` which determines when the behavior
/// is invoked.
pub(crate) trait Behavior<
    H: BehaviorHost,
>: Debug + Send + Sync + Downcast + VisitRefs + 'static {
    /// Computes a transaction to apply the effects of this behavior for one timestep.
    ///
    /// TODO: Define what happens if the transaction fails.
    fn step(
        &self,
        _context: &BehaviorContext<'_, H>,
        _tick: Tick,
    ) -> UniverseTransaction {
        loop {}
    }
    /// Returns [`false`] if the [`Behavior`] should be dropped because conditions under
    /// which it is useful no longer apply.
    fn alive(&self, context: &BehaviorContext<'_, H>) -> bool;
    /// Whether the behavior should never be persisted/saved to disk, because it will be
    /// reconstructed as needed (e.g. collision, occupancy, user interaction, particles).
    ///
    /// If a behavior changes its answer over its lifetime, which outcome will occur is
    /// unspecified.
    fn ephemeral(&self) -> bool;
}
impl_downcast!(Behavior < H > where H : BehaviorHost);
/// A type that can have attached behaviors.
pub trait BehaviorHost: transaction::Transactional + 'static {
    /// Additional data about “where” the behavior is attached to the host; what part of
    /// the host should be affected by the behavior.
    type Attachment: Debug + Clone + Eq + 'static;
}
/// Items available to a [`Behavior`] during [`Behavior::step()`].
#[non_exhaustive]
pub(crate) struct BehaviorContext<'a, H: BehaviorHost> {
    /// The current state of the behavior's host object.
    pub(crate) host: &'a H,
    /// Additional data about “where” the behavior is attached to the host; what part of
    /// the host should be affected by the behavior.
    pub(crate) attachment: &'a H::Attachment,
    host_transaction_binder: &'a dyn Fn(H::Transaction) -> UniverseTransaction,
    self_transaction_binder: &'a dyn Fn(Arc<dyn Behavior<H>>) -> UniverseTransaction,
}
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
    behavior: Arc<dyn Behavior<H>>,
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
    /// Replacement of existing behaviors or their attachments.
    replace: BTreeMap<usize, Replace<H>>,
    /// Newly inserted behaviors.
    insert: Vec<BehaviorSetEntry<H>>,
}
#[derive(Debug)]
struct Replace<H: BehaviorHost> {
    old: BehaviorSetEntry<H>,
    new: BehaviorSetEntry<H>,
}
impl<H: BehaviorHost> BehaviorSetTransaction<H> {
    pub(crate) fn is_empty(&self) -> bool {
        loop {}
    }
    /// This function is private because the normal way it is used is via
    /// [`BehaviorContext::replace_self()`]
    fn replace(index: usize, replacement: Replace<H>) -> Self {
        loop {}
    }
    /// Constructs a transaction that adds a behavior to the behavior set.
    pub(crate) fn insert(
        attachment: H::Attachment,
        behavior: Arc<dyn Behavior<H>>,
    ) -> Self {
        loop {}
    }
    /// Returns an iterator over every behavior attachment added, removed, or modified by
    /// this transaction (not necessary free of duplicates).
    pub(crate) fn attachments_affected(&self) -> impl Iterator<Item = &H::Attachment> {
        let replace = self
            .replace
            .values()
            .flat_map(|Replace { old, new }| [&old.attachment, &new.attachment]);
        let insert = self.insert.iter().map(|entry| &entry.attachment);
        replace.chain(insert)
    }
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
impl<H: BehaviorHost> PartialEq for Replace<H> {
    #[allow(clippy::vtable_address_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
impl<H: BehaviorHost> Eq for BehaviorSetTransaction<H> {}
impl<H: BehaviorHost> Eq for Replace<H> {}
impl<H: BehaviorHost> Clone for Replace<H> {
    fn clone(&self) -> Self {
        loop {}
    }
}
