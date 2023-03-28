use cgmath::{EuclideanSpace as _, InnerSpace as _, Point3, Vector3, Zero};
use ordered_float::NotNan;
use std::fmt;
use super::collision::Contact;
use crate::math::{Aab, FreeCoordinate, Geometry as _};
use crate::space::Space;
use crate::time::Tick;
use crate::transaction::{self, Transaction};
use crate::util::{ConciseDebug, CustomFormat, StatusText};
/// An object with a position, velocity, and collision volume.
/// What it collides with is determined externally.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub struct Body {
    /// Position.
    pub position: Point3<FreeCoordinate>,
    /// Velocity, in position units per second.
    pub velocity: Vector3<FreeCoordinate>,
    /// Collision volume, defined with `position` as the origin.
    pub collision_box: Aab,
    /// Is this body not subject to gravity?
    pub flying: bool,
    /// Is this body not subject to collision?
    pub noclip: bool,
    /// Yaw of the camera look direction, in degrees clockwise from looking towards -Z.
    ///
    /// The preferred range is 0 inclusive to 360 exclusive.
    ///
    /// This does not affect the behavior of the [`Body`] itself; it has nothing to do with
    /// the direction of the velocity.
    pub yaw: FreeCoordinate,
    /// Pitch of the camera look direction, in degrees downward from looking horixontally.
    ///
    /// The preferred range is -90 to 90, inclusive.
    ///
    /// This does not affect the behavior of the [`Body`] itself; it has nothing to do with
    /// the direction of the velocity.
    pub pitch: FreeCoordinate,
}
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
impl Body {
    /// Constructs a [`Body`] requiring only information that can't be reasonably defaulted.
    pub fn new_minimal(
        position: impl Into<Point3<FreeCoordinate>>,
        collision_box: impl Into<Aab>,
    ) -> Self {
        loop {}
    }
    /// Advances time for the body.
    ///
    /// If `colliding_space` is present then the body may collide with blocks in that space
    /// (constraining possible movement) and `collision_callback` will be called with all
    /// such blocks. It is not guaranteed that `collision_callback` will be called only once
    /// per block.
    pub fn step<CC>(
        &mut self,
        tick: Tick,
        mut colliding_space: Option<&Space>,
        mut collision_callback: CC,
    ) -> BodyStepInfo
    where
        CC: FnMut(Contact),
    {
        loop {}
    }
    /// Perform a single straight-line position change, stopping at the first obstacle.
    /// Returns the remainder of `delta_position` that should be retried for sliding movement.
    fn collide_and_advance<CC>(
        &mut self,
        space: &Space,
        collision_callback: &mut CC,
        mut delta_position: Vector3<FreeCoordinate>,
    ) -> (Vector3<FreeCoordinate>, MoveSegment)
    where
        CC: FnMut(Contact),
    {
        loop {}
    }
    /// Check if we're intersecting any blocks and fix that if so.
    fn push_out(&mut self, space: &Space) -> Option<Vector3<FreeCoordinate>> {
        loop {}
    }
    /// Try moving in the given direction, find an empty space, and
    /// return the position and distance to it.
    fn attempt_push_out(
        &self,
        space: &Space,
        direction: Vector3<FreeCoordinate>,
    ) -> Option<(Point3<FreeCoordinate>, NotNan<FreeCoordinate>)> {
        loop {}
    }
    /// Returns the body's collision box in world coordinates
    /// (`collision_box` translated by `position`).
    ///
    /// ```
    /// use all_is_cubes::math::Aab;
    /// use all_is_cubes::physics::Body;
    ///
    /// let body = Body::new_minimal(
    ///     (0.0, 20.0, 0.0),
    ///     Aab::new(-1.0, 1.0, -2.0, 2.0, -3.0, 3.0)
    /// );
    /// assert_eq!(body.collision_box_abs(), Aab::new(-1.0, 1.0, 18.0, 22.0, -3.0, 3.0));
    /// ```
    pub fn collision_box_abs(&self) -> Aab {
        loop {}
    }
    /// Changes [`self.yaw`](Self::yaw) and [`self.pitch`](Self::pitch) to look directly
    /// towards the given point within the same coordinate system as
    /// [`self.position`](Self::position).
    pub fn look_at(&mut self, point: impl Into<Point3<FreeCoordinate>>) {
        loop {}
    }
}
/// Diagnostic data returned by [`Body::step`]. The exact contents of this structure
/// are unstable; use only [`Debug`] formatting to examine its contents unless you have
/// a specific need for one of the values.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct BodyStepInfo {
    /// Whether movement computation was skipped due to approximately zero velocity.
    pub quiescent: bool,
    #[allow(missing_docs)]
    pub push_out: Option<Vector3<FreeCoordinate>>,
    #[allow(missing_docs)]
    pub already_colliding: Option<Contact>,
    /// Details on movement and collision. A single frame's movement may have up to three
    /// segments as differently oriented faces are collided with.
    pub move_segments: [MoveSegment; 3],
}
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
pub struct MoveSegment {
    /// The change in position.
    pub delta_position: Vector3<FreeCoordinate>,
    /// What solid object stopped this segment from continuing further
    /// (there may be others, but this is one of them), or None if there
    /// was no obstacle.
    pub stopped_by: Option<Contact>,
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
    pub delta_yaw: FreeCoordinate,
}
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
