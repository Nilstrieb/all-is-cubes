use super::collision::Contact;
use crate::math::FreeCoordinate;

use crate::transaction::{self, Transaction};
use crate::util::{ConciseDebug, CustomFormat, StatusText};
use cgmath::Vector3;
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
pub(crate) struct BodyStepInfo {
    /// Whether movement computation was skipped due to approximately zero velocity.
    pub(crate) quiescent: bool,
    #[allow(missing_docs)]
    pub(crate) push_out: Option<Vector3<FreeCoordinate>>,
    #[allow(missing_docs)]
    pub(crate) already_colliding: Option<Contact>,
    /// Details on movement and collision. A single frame's movement may have up to three
    /// segments as differently oriented faces are collided with.
    pub(crate) move_segments: [MoveSegment; 3],
}
impl CustomFormat<ConciseDebug> for BodyStepInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, format_type: ConciseDebug) -> fmt::Result {
        loop {}
    }
}
/// One of the individual straight-line movement segments of a [`BodyStepInfo`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub(crate) struct MoveSegment {
    /// The change in position.
    pub(crate) delta_position: Vector3<FreeCoordinate>,
    /// What solid object stopped this segment from continuing further
    /// (there may be others, but this is one of them), or None if there
    /// was no obstacle.
    pub(crate) stopped_by: Option<Contact>,
}
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
pub struct BodyTransaction {
    #[allow(missing_docs)]
    pub(crate) delta_yaw: FreeCoordinate,
}
impl transaction::Transactional for Body {
    type Transaction = BodyTransaction;
}
impl Transaction<Body> for BodyTransaction {
    type CommitCheck = ();
    type Output = transaction::NoOutput;
    fn check(&self, _body: &Body) -> Result<Self::CommitCheck, transaction::PreconditionFailed> {
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
