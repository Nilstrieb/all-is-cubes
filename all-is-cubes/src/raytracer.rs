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
use cgmath::{EuclideanSpace as _, InnerSpace as _, VectorSpace as _};
use cgmath::Point3;
#[cfg(feature = "threads")]
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use crate::block::Evoxels;
use crate::camera::{Camera, GraphicsOptions};
use crate::math::{Face7, FreeCoordinate, GridArray, GridPoint, Rgb, Rgba};
use crate::raycast::Ray;
use crate::space::{BlockIndex, PackedLight, Space, SpaceBlockData};
use crate::util::{CustomFormat, StatusText};
mod pixel_buf;
pub(crate) use pixel_buf::*;
mod surface;
use surface::{Span, Surface};
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
pub(crate) use updating::*;
mod updating;
