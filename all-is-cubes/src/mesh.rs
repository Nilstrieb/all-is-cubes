//! Data structures for triangle meshes and algorithms for converting blocks/voxels to
//! meshes for rendering.
//!
//! All of the algorithms here are independent of graphics API, but they require providing
//! vertex and texture data types suitable for the API or data format you wish to use.
use crate::camera::{GraphicsOptions, TransparencyOption};
use crate::math::FreeCoordinate;
mod block_vertex;
pub(crate) use block_vertex::*;
mod block_mesh;
pub(crate) use block_mesh::*;
#[doc(hidden)]
pub(crate) mod chunked_mesh;
mod space_mesh;
use cgmath::Point3;
pub(crate) use space_mesh::*;
mod texalloc;
pub(crate) use texalloc::*;
/// Parameters for creating meshes that aren't the block/space data itself
/// (or the texture allocator, since that may need to be mutable).
///
/// Creating this and comparing it against a previous instance is appropriate for
/// determining when to invalidate previously computed meshes. This type is also intended
/// to make the API future-proof against additional configuration being needed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub(crate) struct MeshOptions {}
impl MeshOptions {
    /// Take the options relevant to mesh generation from the given [`GraphicsOptions`].
    pub(crate) fn new(graphics_options: &GraphicsOptions) -> Self {
        loop {}
    }
    /// Placeholder for use in tests which do not care about any of the
    /// characteristics that are affected by options (yet).
    #[doc(hidden)]
    pub(crate) fn dont_care_for_test() -> Self {
        loop {}
    }
}
/// One end of a line to be drawn.
///
/// Used for debugging visualizations and not for game content, with the current exception
/// of [`Cursor`](crate::character::Cursor).
///
/// The primary way in which these are used in this crate is
/// [`Geometry::wireframe_points()`](crate::math::Geometry::wireframe_points).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct LineVertex {}
impl From<Point3<FreeCoordinate>> for LineVertex {
    fn from(position: Point3<FreeCoordinate>) -> Self {
        loop {}
    }
}
