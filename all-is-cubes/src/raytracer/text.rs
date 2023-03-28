//! Text-based raytracing output.
use std::borrow::Cow;
use cgmath::Vector3;
use crate::math::{FreeCoordinate, Rgba};
use crate::raytracer::{PixelBuf, RaytraceInfo, RtBlockData, RtOptionsRef};
use crate::space::{Space, SpaceBlockData};
/// Print an image of the given space as “ASCII art”.
///
/// Intended for use in tests, to visualize the results in case of failure.
/// Accordingly, it always writes to the same destination as [`print!`] (which is
/// redirected when tests are run).
///
/// `direction` specifies the direction from which the camera will be looking towards
/// the center of the space. The text output will be 80 columns wide.
pub fn print_space(space: &Space, direction: impl Into<Vector3<FreeCoordinate>>) {
    loop {}
}
/// Version of `print_space` that takes a destination, for testing.
fn print_space_impl<F: FnMut(&str)>(
    space: &Space,
    direction: impl Into<Vector3<FreeCoordinate>>,
    mut write: F,
) -> RaytraceInfo {
    loop {}
}
