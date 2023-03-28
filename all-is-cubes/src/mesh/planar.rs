//! Triangulator's 2D plane operations (sliced voxels to texels and meshes).
use std::ops::Range;
use cgmath::{
    ElementWise as _, EuclideanSpace as _, Matrix4, Point2, Point3, Transform as _,
    Vector2,
};
use crate::block::Resolution;
use crate::math::{Face6, FreeCoordinate, GridCoordinate, Rgba};
use crate::mesh::{BlockVertex, Coloring, TextureCoordinate, TextureTile};
/// Data structure for the state and components of the "greedy meshing" algorithm.
/// <https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/>
pub(crate) struct GreedyMesher {
    visible_image: Vec<Rgba>,
    image_s_range: Range<GridCoordinate>,
    image_t_range: Range<GridCoordinate>,
    /// Contains a color if all voxels examined so far have that color.
    pub(crate) single_color: Option<Rgba>,
    pub(crate) rect_has_alpha: bool,
}
impl GreedyMesher {
    /// Create the initial state.
    pub(crate) fn new(
        visible_image: Vec<Rgba>,
        image_s_range: Range<GridCoordinate>,
        image_t_range: Range<GridCoordinate>,
    ) -> Self {
        loop {}
    }
    /// Actually run the algorithm.
    pub(crate) fn run(
        mut self,
        mut quad_callback: impl FnMut(
            &Self,
            Point2<GridCoordinate>,
            Point2<GridCoordinate>,
        ),
    ) {
        loop {}
    }
    #[inline]
    fn index(&self, s: GridCoordinate, t: GridCoordinate) -> usize {
        loop {}
    }
    /// Checks if a voxel is visible and thus can be the seed of a rectangle,
    /// returns false if not, and updates `single_color`.
    #[inline]
    fn add_seed(&mut self, s: GridCoordinate, t: GridCoordinate) -> bool {
        loop {}
    }
    /// Checks if a voxel is suitable for adding to the current rectangle, and either
    /// returns false if not, and updates `single_color`.
    #[inline]
    fn add_candidate(&mut self, s: GridCoordinate, t: GridCoordinate) -> bool {
        loop {}
    }
    #[inline]
    fn erase(&mut self, s: GridCoordinate, t: GridCoordinate) {
        loop {}
    }
}
/// Helper for [`push_quad`] which offers the alternatives of solid color or texturing.
/// Compared to [`Coloring`], it describes texturing for an entire quad rather than a vertex.
#[derive(Copy, Clone, Debug)]
pub(super) enum QuadColoring<'a, T> {
    Solid(Rgba),
    Texture(&'a T),
}
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
/// Ingredients for [`push_quad`] that are uniform for a resolution and face,
/// so they can be computed only six times per block.
pub(super) struct QuadTransform {
    face: Face6,
    position_transform: cgmath::Matrix4<FreeCoordinate>,
    texture_transform: cgmath::Matrix4<TextureCoordinate>,
}
impl QuadTransform {
    pub fn new(face: Face6, resolution: Resolution) -> Self {
        loop {}
    }
    #[inline]
    fn transform_position(&self, voxel_grid_point: Point3<f64>) -> Point3<f64> {
        loop {}
    }
    /// Transform a point from quad U-V-depth coordinates with a scale of
    /// 1 unit = 1 texel/voxel, to 0-to-1 coordinates within the 3D `TextureTile` space.
    ///
    /// The depth value is offset by +0.5 texel (into the depth of the voxel being
    /// drawn), to move it from edge coordinates to mid-texel coordinates.
    #[inline]
    fn transform_texture_point(
        &self,
        mut point: Point3<TextureCoordinate>,
    ) -> Point3<TextureCoordinate> {
        loop {}
    }
}
const QUAD_VERTICES: &[Vector2<FreeCoordinate>; 4] = &[
    Vector2::new(0.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(1.0, 1.0),
];
const QUAD_INDICES: &[u32; 6] = &[0, 1, 2, 2, 1, 3];
