//! Data structures and helper traits for getting detailed debug info
//! out of the lighting algorithm.
//!
//! Note that this entire module is `doc(hidden)`; pub items inside it
//! are for intra-project use only.
use cgmath::Vector3;
use crate::math::{FreeCoordinate, Geometry, GridPoint};
use crate::mesh::LineVertex;
use crate::raycast::Ray;
use crate::space::PackedLight;
/// Trait used to encourage the generation of with-debug-info and without-info versions
/// of the lighting algorithm.
pub trait LightComputeOutput {
    type RayInfoBuffer: Default;
    fn new(cube: GridPoint, result: PackedLight, rays: Self::RayInfoBuffer) -> Self;
    fn push_ray(buffer: &mut Self::RayInfoBuffer, ray_info: LightUpdateRayInfo);
}
impl LightComputeOutput for LightUpdateCubeInfo {
    type RayInfoBuffer = Vec<LightUpdateRayInfo>;
    fn new(cube: GridPoint, result: PackedLight, rays: Self::RayInfoBuffer) -> Self {
        loop {}
    }
    fn push_ray(buffer: &mut Self::RayInfoBuffer, ray_info: LightUpdateRayInfo) {
        loop {}
    }
}
/// Diagnostic data describing the details of the light calculation for one cube.
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(dead_code)]
pub struct LightUpdateCubeInfo {
    pub(crate) cube: GridPoint,
    pub(crate) result: PackedLight,
    pub(crate) rays: Vec<LightUpdateRayInfo>,
}
impl Geometry for LightUpdateCubeInfo {
    type Coord = FreeCoordinate;
    fn translate(self, _offset: Vector3<FreeCoordinate>) -> Self {
        loop {}
    }
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<LineVertex>,
    {
        loop {}
    }
}
#[derive(Clone, Copy, Debug)]
pub struct LightUpdateRayInfo {
    pub(crate) ray: Ray,
    #[allow(dead_code)]
    pub(crate) trigger_cube: GridPoint,
    pub(crate) value_cube: GridPoint,
    pub(crate) value: PackedLight,
}
impl Geometry for LightUpdateRayInfo {
    type Coord = FreeCoordinate;
    fn translate(self, _offset: Vector3<FreeCoordinate>) -> Self {
        loop {}
    }
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<LineVertex>,
    {
        loop {}
    }
}
