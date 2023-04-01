//! Lighting algorithms for `Space`. This module is closely tied to `Space`
//! and separated out for readability, not modularity.
use std::fmt;
use crate::block::EvaluatedBlock;
use crate::math::FaceMap;
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
pub(crate) struct LightUpdatesInfo {}
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
