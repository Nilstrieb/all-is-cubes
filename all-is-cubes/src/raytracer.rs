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
pub(crate) use pixel_buf::*;
use surface::{Span, Surface};
/// Get cube data out of [`Space`].
#[inline]
fn prepare_cubes(space: &Space) -> GridArray<TracingCubeData> {
    loop {}
}
pub(crate) use updating::*;
