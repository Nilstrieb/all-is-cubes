//! Text-based raytracing output.
use std::borrow::Cow;
use cgmath::{Vector3};

use crate::math::{FreeCoordinate, Rgba};
use crate::raytracer::{
    PixelBuf, RaytraceInfo, RtBlockData, RtOptionsRef,
};
use crate::space::{Space, SpaceBlockData};
/// TODO: better name, docs
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct CharacterRtData(pub Cow<'static, str>);
impl RtBlockData for CharacterRtData {
    type Options = ();
    fn from_block(_: RtOptionsRef<'_, Self::Options>, s: &SpaceBlockData) -> Self {
        loop {}
    }
    fn error(_: RtOptionsRef<'_, Self::Options>) -> Self {
        loop {}
    }
    fn sky(_: RtOptionsRef<'_, Self::Options>) -> Self {
        loop {}
    }
}
/// Implements [`PixelBuf`] for text output: captures the first characters of block names
/// rather than colors.
#[derive(Clone, Debug, Default, PartialEq)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct CharacterBuf {
    /// Text to draw, if determined yet.
    hit_text: Option<String>,
}
impl PixelBuf for CharacterBuf {
    type BlockData = CharacterRtData;
    #[inline]
    fn opaque(&self) -> bool {
        loop {}
    }
    #[inline]
    fn add(&mut self, _surface_color: Rgba, d: &Self::BlockData) {
        loop {}
    }
    fn hit_nothing(&mut self) {
        loop {}
    }
    fn mean<const N: usize>(items: [Self; N]) -> Self {
        loop {}
    }
}
impl From<CharacterBuf> for String {
    #[inline]
    fn from(buf: CharacterBuf) -> String {
        loop {}
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Block, Resolution::R4};
    use crate::content::make_some_blocks;
    use crate::universe::Universe;
    #[test]
    fn print_space_test() {
        loop {}
    }
    /// Check that blocks with small spaces are handled without out-of-bounds errors
    #[test]
    fn partial_voxels() {
        loop {}
    }
}
