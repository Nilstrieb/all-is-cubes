use crate::transaction::{self, Transaction};


/// An object with a position, velocity, and collision volume.
/// What it collides with is determined externally.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub struct Body {}
/// The [`Transaction`] type for [`Body`].
///
/// TODO: Very incomplete; just a sketch of what eventually needs to exist.
#[derive(Clone, Debug, Default, PartialEq)]
#[must_use]
#[non_exhaustive]
pub struct BodyTransaction {}
impl Transaction<Body> for BodyTransaction {
    type CommitCheck = ();
    type Output = transaction::NoOutput;
    fn check(
        &self,
        _body: &Body,
    ) -> Result<Self::CommitCheck, transaction::PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        body: &mut Body,
        _: Self::CommitCheck,
        _outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), transaction::CommitError> {
        loop {}
    }
}
impl transaction::Merge for BodyTransaction {
    type MergeCheck = ();
    fn check_merge(
        &self,
        _other: &Self,
    ) -> Result<Self::MergeCheck, transaction::TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, other: Self, (): Self::MergeCheck) -> Self {
        loop {}
    }
}
