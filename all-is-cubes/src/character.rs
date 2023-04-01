//! Player-character stuff.
use crate::behavior::{Behavior, BehaviorSet, BehaviorSetTransaction};
use crate::camera::ViewTransform;
use crate::inv::{Inventory, InventoryChange, InventoryTransaction, TOOL_SELECTIONS};
use crate::listen::{Listen, Listener};
use crate::math::FreeCoordinate;
use crate::physics::{Body, BodyStepInfo, BodyTransaction};
use crate::space::Space;
use crate::time::Tick;
use crate::transaction::{
    self, CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict, Transactional,
};
use crate::universe::{RefVisitor, URef, UniverseTransaction, VisitRefs};
use crate::util::{CustomFormat, StatusText};
use cgmath::Vector3;
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
impl CustomFormat<StatusText> for Character {
    #[mutants::skip]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: StatusText) -> fmt::Result {
        loop {}
    }
}
impl VisitRefs for Character {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
impl Listen for Character {
    type Msg = CharacterChange;
    /// Registers a listener for mutations of this character.
    fn listen<L: Listener<CharacterChange> + Send + Sync + 'static>(&self, listener: L) {
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
pub struct CharacterTransaction {
    body: BodyTransaction,
    inventory: InventoryTransaction,
    behaviors: BehaviorSetTransaction<Character>,
}
impl CharacterTransaction {}
#[allow(clippy::type_complexity)]
impl Transaction<Character> for CharacterTransaction {
    type CommitCheck = (
        <BodyTransaction as Transaction<Body>>::CommitCheck,
        <InventoryTransaction as Transaction<Inventory>>::CommitCheck,
        <BehaviorSetTransaction<Character> as Transaction<BehaviorSet<Character>>>::CommitCheck,
    );
    type Output = transaction::NoOutput;
    fn check(&self, target: &Character) -> Result<Self::CommitCheck, PreconditionFailed> {
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
    fn check_merge(&self, other: &Self) -> Result<Self::MergeCheck, TransactionConflict> {
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
/// Description of a change to a [`Character`] for use in listeners.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum CharacterChange {
    /// Inventory contents.
    Inventory(InventoryChange),
    /// Which inventory slots are selected.
    Selections,
}
