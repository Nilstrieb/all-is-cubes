//! Triangulator's 2D plane operations (sliced voxels to texels and meshes).
use std::ops::Range;
use cgmath::{Point2, Point3, Vector2};
use crate::block::Resolution;
use crate::math::{Face6, FreeCoordinate, GridCoordinate, Rgba};
use crate::mesh::{BlockVertex, TextureCoordinate, TextureTile};
/// Compute vertices for a quad and push them into the supplied vectors.
///
/// `depth`, `low_corner`, and `high_corner` are in units of 1 texel.
#[inline]
#[allow(clippy::too_many_arguments)]
pub(super) fn push_quad<V: From<BlockVertex<Tex::Point>>, Tex: TextureTile>(
    vertices: &mut Vec<V>,
    indices: &mut Vec<u32>,
    transform: &QuadTransform,
    depth: FreeCoordinate,
    low_corner: Point2<FreeCoordinate>,
    high_corner: Point2<FreeCoordinate>,
    coloring: QuadColoring<'_, Tex>,
) {
    loop {}
}
