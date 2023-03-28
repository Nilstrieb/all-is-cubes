use cgmath::Point3;
use crate::block::Evoxel;
use crate::math::{Face7, FreeCoordinate, GridArray, GridPoint, Rgb, Rgba};
use crate::raycast::{Ray, Raycaster};
use crate::raytracer::{RtBlockData, SpaceRaytracer, TracingBlock, TracingCubeData};
/// Simple directional lighting used to give corners extra definition.
/// Note that this algorithm is also implemented in the fragment shader for GPU rendering.
fn fixed_directional_lighting(face: Face7) -> f32 {
    loop {}
}
