//! Player-character stuff.
use crate::behavior::{BehaviorSet, BehaviorSetTransaction};
use crate::inv::{Inventory, InventoryChange, InventoryTransaction, TOOL_SELECTIONS};
use crate::physics::{Body, BodyStepInfo, BodyTransaction};
use crate::transaction::{
    self, CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict,
    Transactional,
};
use crate::universe::{RefVisitor, URef, UniverseTransaction, VisitRefs};
use crate::util::{CustomFormat, StatusText};
use std::fmt;
mod cursor;
pub(crate) use cursor::*;
mod spawn;
pub(crate) use spawn::*;
/// A `Character`:
///
/// * knows what [`Space`] it is looking at, by reference,
/// * knows where it is located and how it collides via a `Body` which it owns and
///   steps, and
/// * handles the parts of input management that are associated with universe state
///   (controlling velocity, holding tools).
pub struct Character {}
impl fmt::Debug for Character {
    #[mutants::skip]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Transactional for Character {
    type Transaction = CharacterTransaction;
}
impl crate::behavior::BehaviorHost for Character {
    type Attachment = ();
}
/// A [`Transaction`] that modifies a [`Character`].
#[derive(Clone, Debug, Default, PartialEq)]
#[must_use]
pub struct CharacterTransaction {}
#[allow(clippy::type_complexity)]
impl Transaction<Character> for CharacterTransaction {
    type CommitCheck = (
        <BodyTransaction as Transaction<Body>>::CommitCheck,
        <InventoryTransaction as Transaction<Inventory>>::CommitCheck,
        <BehaviorSetTransaction<
            Character,
        > as Transaction<BehaviorSet<Character>>>::CommitCheck,
    );
    type Output = transaction::NoOutput;
    fn check(
        &self,
        target: &Character,
    ) -> Result<Self::CommitCheck, PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        target: &mut Character,
        (body_check, inventory_check, behaviors_check): Self::CommitCheck,
        outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), CommitError> {
        loop {}
    }
}
impl Merge for CharacterTransaction {
    type MergeCheck = (
        <BodyTransaction as Merge>::MergeCheck,
        <InventoryTransaction as Merge>::MergeCheck,
        <BehaviorSetTransaction<Character> as Merge>::MergeCheck,
    );
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(
        self,
        other: Self,
        (body_check, inventory_check, behaviors_check): Self::MergeCheck,
    ) -> Self {
        loop {}
    }
}
