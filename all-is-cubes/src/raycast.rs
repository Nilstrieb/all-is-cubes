//! Algorithm for raycasting through voxel grids.
//!
//! This deals purely with the question "which cubes does this ray intersect",
//! and does not concern itself with what might occupy those cubes. If you're
//! looking for *raytracing*, forming an image from many rays, that's
//! `all_is_cubes::raytracer`.


/// A ray; a half-infinite line segment (sometimes used as finite by the length of the
/// direction vector).
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Ray {}
