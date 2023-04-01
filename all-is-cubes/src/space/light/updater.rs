//! Lighting algorithms for `Space`. This module is closely tied to `Space`
//! and separated out for readability, not modularity.
use std::fmt;
use once_cell::sync::Lazy;
use super::debug::LightComputeOutput;
use crate::block::EvaluatedBlock;
use crate::math::{FaceMap, GridPoint, Rgb};
use crate::raycast::{Ray, RaycastStep};
use crate::space::{GridAab, LightPhysics, PackedLight, PackedLightScalar, Space};
use crate::util::{CustomFormat, StatusText};
/// Given a block and its neighbors, which directions should we cast rays to find light
/// falling on it?
fn directions_to_seek_light(
    origin: &EvaluatedBlock,
    neighborhood: FaceMap<&EvaluatedBlock>,
) -> FaceMap<f32> {
    loop {}
}
/// Performance data for bulk light updates.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct LightUpdatesInfo {
    /// Number of blocks whose light data updates are aggregated in this data.
    pub(crate) update_count: usize,
    /// The largest change in light value that occurred.
    pub(crate) max_update_difference: u8,
    /// Number of entries in the light update queue.
    pub(crate) queue_count: usize,
    /// The largest update priority in the queue (corresponds to the size of
    /// difference that caused the cube to be added).
    pub(crate) max_queue_priority: u8,
}
impl std::ops::AddAssign<LightUpdatesInfo> for LightUpdatesInfo {
    fn add_assign(&mut self, other: Self) {
        loop {}
    }
}
impl CustomFormat<StatusText> for LightUpdatesInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: StatusText) -> fmt::Result {
        loop {}
    }
}
/// A special definition of opacity for the lighting algorithm:
/// we want to treat opaque light-emitting blocks similarly to transparent blocks
/// *when deciding to compute light for them*, because this produces better results
/// for smooth (interpolated) lighting.
///
/// This function is fairly straightforward; it exists for purposes of *documenting
/// the places that care about this* rather than for code reduction.
pub(crate) fn opaque_for_light_computation(block: &EvaluatedBlock) -> bool {
    loop {}
}
