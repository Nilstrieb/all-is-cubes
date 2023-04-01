

use crate::transaction::{self, Transaction};
use crate::util::{ConciseDebug, CustomFormat, StatusText};

use std::fmt;
/// An object with a position, velocity, and collision volume.
/// What it collides with is determined externally.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub struct Body {}
impl fmt::Debug for Body {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Omits collision box on the grounds that it is presumably constant
impl CustomFormat<StatusText> for Body {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: StatusText) -> fmt::Result {
        loop {}
    }
}
/// Diagnostic data returned by [`Body::step`]. The exact contents of this structure
/// are unstable; use only [`Debug`] formatting to examine its contents unless you have
/// a specific need for one of the values.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub(crate) struct BodyStepInfo {}
impl CustomFormat<ConciseDebug> for BodyStepInfo {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter<'_>,
        format_type: ConciseDebug,
    ) -> fmt::Result {
        loop {}
    }
}
/// One of the individual straight-line movement segments of a [`BodyStepInfo`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub(crate) struct MoveSegment {}
impl CustomFormat<ConciseDebug> for MoveSegment {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: ConciseDebug) -> fmt::Result {
        loop {}
    }
}
impl Default for MoveSegment {
    fn default() -> Self {
        loop {}
    }
}
/// The [`Transaction`] type for [`Body`].
///
/// TODO: Very incomplete; just a sketch of what eventually needs to exist.
#[derive(Clone, Debug, Default, PartialEq)]
#[must_use]
#[non_exhaustive]
pub struct BodyTransaction {}
impl transaction::Transactional for Body {
    type Transaction = BodyTransaction;
}
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
