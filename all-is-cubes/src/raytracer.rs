//! Raytracer for [`Space`]s.
//!
//! ## Why?
//!
//! The original reason this exists is that I thought “we have [`raycast`](crate::raycast),
//! and that's nearly all the work, so why not?” Secondarily, it was written before
//! [the mesh-based renderer](crate::mesh), and was useful as a cross-check since
//! it is much simpler. It continues to serve as a “reference implementation” and is used
//! by the terminal UI and in unit tests via [`print_space`].
use std::fmt;
use cgmath::{
    EuclideanSpace as _, InnerSpace as _, Point2, Vector2, Vector3, VectorSpace as _,
};
use cgmath::{Point3, Vector4};
use ordered_float::NotNan;
#[cfg(feature = "threads")]
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use crate::block::{Evoxels, AIR};
use crate::camera::{Camera, GraphicsOptions, TransparencyOption};
use crate::math::{
    point_to_enclosing_cube, smoothstep, Face7, FreeCoordinate, GridAab, GridArray,
    GridPoint, Rgb, Rgba,
};
use crate::raycast::Ray;
use crate::space::{BlockIndex, PackedLight, Space, SpaceBlockData};
use crate::util::{CustomFormat, StatusText};
mod pixel_buf;
pub use pixel_buf::*;
mod renderer;
pub use renderer::*;
mod surface;
use surface::{DepthIter, DepthStep, Span, Surface, SurfaceIter, TraceStep};
mod text;
pub use text::*;
/// Precomputed data for raytracing a single frame of a single [`Space`], and bearer of
/// the methods for actually performing raytracing.
pub struct SpaceRaytracer<D: RtBlockData> {
    blocks: Vec<TracingBlock<D>>,
    cubes: GridArray<TracingCubeData>,
    graphics_options: GraphicsOptions,
    custom_options: D::Options,
    sky_color: Rgb,
    sky_data: D,
    packed_sky_color: PackedLight,
}
impl<D: RtBlockData> SpaceRaytracer<D> {
    /// Snapshots the given [`Space`] to prepare for raytracing it.
    pub fn new(
        space: &Space,
        graphics_options: GraphicsOptions,
        custom_options: D::Options,
    ) -> Self {
        loop {}
    }
    /// Construct a [`SpaceRaytracer`] with nothing to render.
    pub(crate) fn new_empty(
        sky_color: Rgb,
        graphics_options: GraphicsOptions,
        custom_options: D::Options,
    ) -> Self {
        loop {}
    }
    /// Computes a single image pixel from the given ray.
    pub fn trace_ray<P: PixelBuf<BlockData = D>>(
        &self,
        ray: Ray,
        include_sky: bool,
    ) -> (P, RaytraceInfo) {
        loop {}
    }
    #[inline]
    fn get_packed_light(&self, cube: GridPoint) -> PackedLight {
        loop {}
    }
    #[inline]
    fn get_lighting(&self, cube: GridPoint) -> Rgb {
        loop {}
    }
    fn get_interpolated_light(&self, point: Point3<FreeCoordinate>, face: Face7) -> Rgb {
        loop {}
    }
}
/// Text-specific methods.
impl<D: RtBlockData> SpaceRaytracer<D> {
    /// Raytrace to text, using any [`PixelBuf`] whose output can be [`String`].
    ///
    /// `F` is the function accepting the output, and `E` is the type of error it may
    /// produce. This function-based interface is intended to abstract over the
    /// inconvenient difference between [`std::io::Write`] and [`std::fmt::Write`].
    ///
    /// After each line (row) of the image, `write(line_ending)` will be called.
    pub fn trace_scene_to_text<P, F, E>(
        &self,
        camera: &Camera,
        line_ending: &str,
        write: F,
    ) -> Result<RaytraceInfo, E>
    where
        P: PixelBuf<BlockData = D> + Into<String>,
        F: FnMut(&str) -> Result<(), E>,
    {
        loop {}
    }
    #[cfg(feature = "threads")]
    fn trace_scene_to_text_impl<P, F, E>(
        &self,
        camera: &Camera,
        line_ending: &str,
        mut write: F,
    ) -> Result<RaytraceInfo, E>
    where
        P: PixelBuf<BlockData = D> + Into<String>,
        F: FnMut(&str) -> Result<(), E>,
    {
        loop {}
    }
    #[cfg(not(feature = "threads"))]
    fn trace_scene_to_text_impl<P, F, E>(
        &self,
        camera: &Camera,
        line_ending: &str,
        mut write: F,
    ) -> Result<RaytraceInfo, E>
    where
        P: PixelBuf<BlockData = D> + Into<String>,
        F: FnMut(&str) -> Result<(), E>,
    {
        loop {}
    }
    /// As [`Self::trace_scene_to_text()`], but returning a string.
    pub fn trace_scene_to_string<P>(&self, camera: &Camera, line_ending: &str) -> String
    where
        P: PixelBuf<BlockData = D> + Into<String>,
    {
        loop {}
    }
}
impl<D: RtBlockData> fmt::Debug for SpaceRaytracer<D>
where
    D::Options: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Performance info from a [`SpaceRaytracer`] operation.
///
/// The contents of this structure are subject to change; use [`Debug`] to view it.
/// The [`Default`] value is the zero value.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct RaytraceInfo {
    cubes_traced: usize,
}
impl std::ops::AddAssign<RaytraceInfo> for RaytraceInfo {
    fn add_assign(&mut self, other: Self) {
        loop {}
    }
}
impl std::iter::Sum for RaytraceInfo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        loop {}
    }
}
impl CustomFormat<StatusText> for RaytraceInfo {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter<'_>,
        _format_type: StatusText,
    ) -> fmt::Result {
        loop {}
    }
}
/// Get cube data out of [`Space`].
#[inline]
fn prepare_cubes(space: &Space) -> GridArray<TracingCubeData> {
    loop {}
}
#[derive(Clone, Debug)]
struct TracingCubeData {
    block_index: BlockIndex,
    lighting: PackedLight,
    /// True if the block is [`AIR`].
    ///
    /// This special information allows us to skip an indirect memory access in this
    /// extremely common case. We could generalize it to any block which is fully
    /// invisible, but only if *the block is not an indirection* since if it is, the
    /// block data could change without signaling a cube change, and currently we don't
    /// have a mechanism to obtain that information from the Space.
    always_invisible: bool,
}
#[derive(Clone, Debug)]
struct TracingBlock<D> {
    block_data: D,
    voxels: Evoxels,
}
impl<D: RtBlockData> TracingBlock<D> {
    fn from_block(
        options: RtOptionsRef<'_, D::Options>,
        space_block_data: &SpaceBlockData,
    ) -> Self {
        loop {}
    }
}
/// Holds a [`PixelBuf`] and other per-ray state, and updates it
/// according to the things it encounters.
#[derive(Clone, Debug, Default)]
struct TracingState<P: PixelBuf> {
    /// Conversion factor from raycaster `t` values to “true” [`Space`] distance values
    /// where 1 unit = 1 block thickness.
    t_to_absolute_distance: f64,
    /// Number of cubes traced through -- controlled by the caller, so not necessarily
    /// equal to the number of calls to [`Self::trace_through_surface()`].
    cubes_traced: usize,
    pixel_buf: P,
}
impl<P: PixelBuf> TracingState<P> {
    #[inline]
    fn count_step_should_stop(
        &mut self,
        options: RtOptionsRef<'_, <P::BlockData as RtBlockData>::Options>,
    ) -> bool {
        loop {}
    }
    fn finish(mut self, sky_color: Rgba, sky_data: &P::BlockData) -> (P, RaytraceInfo) {
        loop {}
    }
    /// Apply the effect of a given surface color.
    #[inline]
    fn trace_through_surface(
        &mut self,
        surface: Surface<'_, P::BlockData>,
        rt: &SpaceRaytracer<P::BlockData>,
    ) {
        loop {}
    }
    #[inline]
    fn trace_through_span(
        &mut self,
        span: Span<'_, P::BlockData>,
        rt: &SpaceRaytracer<P::BlockData>,
    ) {
        loop {}
    }
}
pub use updating::*;
mod updating;
#[cfg(feature = "threads")]
mod rayon_helper {
    use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator as _};
    use std::iter::{empty, once, Sum};
    /// Implements [`ParallelExtend`] to just sum things, so that
    /// [`ParallelIterator::unzip`] can produce a sum.
    #[derive(Clone, Copy, Debug, Default)]
    pub(crate) struct ParExtSum<T>(Option<T>);
    impl<T: Sum> ParExtSum<T> {
        pub fn result(self) -> T {
            loop {}
        }
    }
    impl<T: Sum + Send> ParallelExtend<T> for ParExtSum<T> {
        fn par_extend<I>(&mut self, par_iter: I)
        where
            I: IntoParallelIterator<Item = T>,
        {
            loop {}
        }
    }
}
