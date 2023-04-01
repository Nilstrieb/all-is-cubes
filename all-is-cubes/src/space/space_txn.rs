//! TODO: Maybe this file is too small

use crate::behavior::{BehaviorSet, BehaviorSetTransaction};

use crate::space::Space;
use crate::transaction::{
    CommitError, Merge, NoOutput, PreconditionFailed, Transaction, TransactionConflict,
    Transactional,
};
impl Transactional for Space {
    type Transaction = SpaceTransaction;
}
/// A [`Transaction`] that modifies a [`Space`].
#[derive(Clone, Default, Eq, PartialEq)]
#[must_use]
pub struct SpaceTransaction {}
impl Transaction<Space> for SpaceTransaction {
    type CommitCheck =
        <BehaviorSetTransaction<Space> as Transaction<BehaviorSet<Space>>>::CommitCheck;
    type Output = NoOutput;
    fn check(&self, space: &Space) -> Result<Self::CommitCheck, PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        space: &mut Space,
        check: Self::CommitCheck,
        _outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), CommitError> {
        loop {}
    }
}
impl Merge for SpaceTransaction {
    type MergeCheck = <BehaviorSetTransaction<Space> as Merge>::MergeCheck;
    fn check_merge(&self, other: &Self) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, mut other: Self, check: Self::MergeCheck) -> Self {
        loop {}
    }
}
