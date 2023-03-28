use cgmath::{Point3};
use crate::block::{Evoxel};

use crate::math::{Face7, FreeCoordinate, GridArray, GridPoint, Rgb, Rgba};
use crate::raycast::{Ray, Raycaster};
use crate::raytracer::{RtBlockData, SpaceRaytracer, TracingBlock, TracingCubeData};
/// Description of a surface the ray passes through (or from the volumetric perspective,
/// a transition from one material to another).
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Surface<'a, D> {
    pub block_data: &'a D,
    pub diffuse_color: Rgba,
    /// The cube of the [`Space`] which contains the block this surface belongs to.
    cube: GridPoint,
    /// The distance along the ray, in units of the ray's direction vector,
    /// where it intersected the surface.
    pub t_distance: FreeCoordinate,
    /// The point in the [`Space`]'s coordinate system where the ray intersected the surface.
    intersection_point: Point3<FreeCoordinate>,
    pub normal: Face7,
}
impl<D: RtBlockData> Surface<'_, D> {
    /// Convert the surface and its lighting to a single RGBA value as determined by
    /// the given graphics options, or [`None`] if it is invisible.
    ///
    /// Note this is not true volumetric ray tracing: we're considering each
    /// voxel surface to be discrete.
    #[inline]
    pub(crate) fn to_lit_color(&self, rt: &SpaceRaytracer<D>) -> Option<Rgba> {
        loop {}
    }
    fn compute_illumination(&self, rt: &SpaceRaytracer<D>) -> Rgb {
        loop {}
    }
}
/// Simple directional lighting used to give corners extra definition.
/// Note that this algorithm is also implemented in the fragment shader for GPU rendering.
fn fixed_directional_lighting(face: Face7) -> f32 {
    loop {}
}
/// Builds on [`Surface`] to report the depth (length of ray through volume)
/// of a transparent surface.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Span<'a, D> {
    pub surface: Surface<'a, D>,
    /// Distance along the ray at which the ray exits this volume.
    ///
    /// The `surface.t_distance` is the corresponding entry point.
    pub exit_t_distance: FreeCoordinate,
}
/// Output of [`SurfaceIter`], describing a single step of the raytracing process.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum TraceStep<'a, D> {
    /// A block or voxel surface as described was entered.
    ///
    /// Note that when the surface is invisible (alpha is zero or treated as zero),
    /// [`TraceStep::Invisible`] is returned.
    EnterSurface(Surface<'a, D>),
    /// A completely invisible surface was found.
    /// This is reported so that it may be counted in a decision to stop tracing.
    /// It is separate from [`TraceStep::EnterSurface`] to avoid computing lighting.
    Invisible { t_distance: FreeCoordinate },
    /// A recursive block was entered.
    ///
    /// This is reported so as to keep the iteration process O(1).
    /// In particular, otherwise an unbounded number of steps could be taken if the ray
    /// passes through a large number of recursive blocks with small bounds (such that no
    /// actual voxels exist to be visible or invisible).
    EnterBlock { t_distance: FreeCoordinate },
}
/// An [`Iterator`] which reports each visible surface a [`Raycaster`] ray passes through.
#[derive(Clone, Debug)]
pub(crate) struct SurfaceIter<'a, D> {
    ray: Ray,
    block_raycaster: Raycaster,
    state: SurfaceIterState,
    current_block: Option<VoxelSurfaceIter<'a, D>>,
    blocks: &'a [TracingBlock<D>],
    array: &'a GridArray<TracingCubeData>,
}
#[derive(Clone, Copy, Debug)]
enum SurfaceIterState {
    Initial,
    /// At least one raycast step within the space has been seen.
    EnteredSpace,
}
impl<'a, D: RtBlockData> SurfaceIter<'a, D> {
    #[inline]
    pub(crate) fn new(rt: &'a SpaceRaytracer<D>, ray: Ray) -> Self {
        loop {}
    }
}
impl<'a, D> Iterator for SurfaceIter<'a, D> {
    type Item = TraceStep<'a, D>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
/// Iterates over a [`Block`]'s voxels. Internal helper for [`SurfaceIter`].
#[derive(Clone, Debug)]
struct VoxelSurfaceIter<'a, D> {
    voxel_ray: Ray,
    voxel_raycaster: Raycaster,
    block_data: &'a D,
    /// Reciprocal of resolution, for scaling back to outer world
    antiscale: FreeCoordinate,
    array: &'a GridArray<Evoxel>,
    /// Cube these voxels are located in, for lighting lookups.
    block_cube: GridPoint,
}
impl<'a, D> VoxelSurfaceIter<'a, D> {
    /// This is not an  implementation of `Iterator` because it doesn't need to be — it's
    /// purely internal to [`SurfaceIter`].
    fn next(&mut self) -> Option<TraceStep<'a, D>> {
        loop {}
    }
}
/// Builds on [`SurfaceIter`] to report spans of transparency along the ray.
pub(crate) struct DepthIter<'a, D> {
    surface_iter: SurfaceIter<'a, D>,
    /// Present if the last `EnterSurface` we discovered was transparent, or if
    /// we have another surface to report.
    last_surface: Option<Surface<'a, D>>,
}
impl<'a, D> DepthIter<'a, D> {
    #[inline]
    pub(crate) fn new(surface_iter: SurfaceIter<'a, D>) -> Self {
        loop {}
    }
}
impl<'a, D> Iterator for DepthIter<'a, D> {
    type Item = DepthStep<'a, D>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum DepthStep<'a, D> {
    Invisible,
    Span(Span<'a, D>),
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Block, Resolution::*, AIR};
    use crate::camera::GraphicsOptions;
    use crate::math::GridAab;
    use crate::space::Space;
    use crate::universe::Universe;
    use pretty_assertions::assert_eq;
    use TraceStep::{EnterBlock, EnterSurface, Invisible};
    #[test]
    fn surface_iter_smoke_test() {
        loop {}
    }
    /// Test that exiting a block at the edge of the space still reports the exit t-distance.
    #[test]
    fn surface_iter_exit_block_at_end_of_space() {
        loop {}
    }
}
