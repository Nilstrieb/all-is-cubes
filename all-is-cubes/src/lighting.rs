// Copyright 2020 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <http://opensource.org/licenses/MIT>.

//! Lighting algorithms for `Space`. This module is closely tied to `Space`
//! and separated out for readability, not modularity.

use cgmath::{EuclideanSpace as _, Point3, Transform as _, Vector3, Zero as _};
use once_cell::sync::Lazy;
use ordered_float::NotNan;
use std::convert::TryInto as _;

use crate::math::*;
use crate::raycast::Ray;
use crate::space::*;

pub(crate) type PackedLightScalar = u8;

/// Lighting within a `Space`; an `all_is_cubes::math::RGB` value stored with reduced
/// precision and range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackedLight(Vector3<PackedLightScalar>);
// TODO: Once we've built out the rest of the game, do some performance testing and
// decide whether having colored lighting is worth the compute and storage cost.
// If memory vs. bit depth is an issue, consider switching to something like YCbCr
// representation, or possibly something that GPUs specifically do well with.
//
// Also consider whether we should have gamma -- or even a logarithmic representation.

impl PackedLight {
    /// Unit value of these fixed-point color components, as a f32 for conversion calculations.
    const UNIT_F32: f32 = 64.0;

    pub(crate) const ZERO: PackedLight = PackedLight(Vector3 { x: 0, y: 0, z: 0 });

    #[inline]
    fn difference_magnitude(self, other: PackedLight) -> PackedLightScalar {
        fn dm(a: PackedLightScalar, b: PackedLightScalar) -> PackedLightScalar {
            a.max(b) - a.min(b)
        }
        dm(self.0[0], other.0[0])
            .max(dm(self.0[1], other.0[1]))
            .max(dm(self.0[2], other.0[2]))
    }
}

impl From<RGB> for PackedLight {
    #[inline]
    fn from(value: RGB) -> Self {
        PackedLight(Vector3::new(
            (value.red().into_inner() * PackedLight::UNIT_F32) as PackedLightScalar,
            (value.green().into_inner() * PackedLight::UNIT_F32) as PackedLightScalar,
            (value.blue().into_inner() * PackedLight::UNIT_F32) as PackedLightScalar,
        ))
    }
}
impl From<PackedLight> for [f32; 3] {
    #[inline]
    fn from(value: PackedLight) -> Self {
        [
            f32::from(value.0[0]) / PackedLight::UNIT_F32,
            f32::from(value.0[1]) / PackedLight::UNIT_F32,
            f32::from(value.0[2]) / PackedLight::UNIT_F32,
        ]
    }
}
impl From<PackedLight> for RGB {
    #[inline]
    fn from(value: PackedLight) -> Self {
        RGB::new(
            f32::from(value.0[0]) / PackedLight::UNIT_F32,
            f32::from(value.0[1]) / PackedLight::UNIT_F32,
            f32::from(value.0[2]) / PackedLight::UNIT_F32,
        )
    }
}

const RAY_DIRECTION_STEP: isize = 2;
const RAYS_PER_FACE: usize = ((1 + RAY_DIRECTION_STEP * 2) * (1 + RAY_DIRECTION_STEP * 2)) as usize;
const ALL_RAYS_COUNT: usize = RAYS_PER_FACE * 6;

/// Fixed configuration of light rays to use for light tracing.
#[derive(Clone, Copy)]
struct FaceRayData {
    // TODO: reflect_face was used in the original lighting algorithm but we haven't implemented that part yet (and one try produced bad results).
    #[allow(dead_code)]
    face: Face,
    rays: [Ray; RAYS_PER_FACE],
}

static LIGHT_RAYS: Lazy<[FaceRayData; 6]> = Lazy::new(|| {
    let mut ray_data = Vec::new();
    for &face in Face::ALL_SIX {
        let origin = Point3::new(0.5, 0.5, 0.5) + face.normal_vector() * 0.25;
        let mut face_ray_data = FaceRayData {
            face,
            rays: [Ray {
                origin: Point3::origin(),
                direction: Vector3::zero(),
            }; RAYS_PER_FACE],
        };
        // RAYS_PER_FACE is too big to use convenience traits, so we have to
        // explicitly index it to write into it. TODO: no longer true in rust 1.47
        let mut i = 0;
        for rayx in -RAY_DIRECTION_STEP..=RAY_DIRECTION_STEP {
            for rayy in -RAY_DIRECTION_STEP..=RAY_DIRECTION_STEP {
                // TODO: we can simplify this if we add "transform ray by matrix".
                face_ray_data.rays[i] = Ray {
                    origin,
                    direction: face.matrix().transform_vector(Vector3::new(
                        rayx as FreeCoordinate,
                        rayy as FreeCoordinate,
                        -1.0,
                    )),
                };
                i += 1;
            }
        }
        ray_data.push(face_ray_data);
    }
    (*ray_data).try_into().unwrap()
});

pub(crate) fn initialize_lighting(grid: Grid, color: PackedLight) -> Box<[PackedLight]> {
    vec![color; grid.volume()].into_boxed_slice()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LightUpdateRequest {
    priority: PackedLightScalar,
    cube: GridPoint,
}
impl Ord for LightUpdateRequest {
    fn cmp(&self, other: &LightUpdateRequest) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.cube[0].cmp(&other.cube[0]))
            .then_with(|| self.cube[1].cmp(&other.cube[1]))
            .then_with(|| self.cube[2].cmp(&other.cube[2]))
    }
}
impl PartialOrd for LightUpdateRequest {
    fn partial_cmp(&self, other: &LightUpdateRequest) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Space {
    pub(crate) fn light_needs_update(&mut self, cube: GridPoint, priority: PackedLightScalar) {
        if self.grid().contains_cube(cube) && !self.lighting_update_set.contains(&cube) {
            self.lighting_update_queue
                .push(LightUpdateRequest { priority, cube });
            self.lighting_update_set.insert(cube);
        }
    }

    /// Do some lighting updates.
    pub(crate) fn update_lighting_from_queue(&mut self) -> SpaceStepInfo {
        // Do a finite number of updates.
        let mut light_update_count: usize = 0;
        let mut max_difference: PackedLightScalar = 0;
        while let Some(LightUpdateRequest { cube, .. }) = self.lighting_update_queue.pop() {
            self.lighting_update_set.remove(&cube);
            light_update_count += 1;
            // Note: For performance, it is key that this call site ignores the info value
            // and the functions are inlined. Thus, the info calculation can be
            // optimized away.
            let (difference, _) = self.update_lighting_now_on(cube);
            max_difference = max_difference.max(difference);
            if light_update_count >= 120 {
                break;
            }
        }
        SpaceStepInfo {
            light_update_count,
            light_queue_count: self.lighting_update_queue.len(),
            max_light_update_difference: max_difference,
        }
    }

    #[inline]
    fn update_lighting_now_on(
        &mut self,
        cube: GridPoint,
    ) -> (PackedLightScalar, LightingUpdateInfo) {
        let (new_light_value, dependencies, info) = self.compute_lighting(cube);
        let old_light_value: PackedLight = self.get_lighting(cube);
        let difference_magnitude = new_light_value.difference_magnitude(old_light_value);
        if difference_magnitude > 0 {
            // TODO: compute index only once
            self.lighting[self.grid().index(cube).unwrap()] = new_light_value;
            self.notifier.notify(SpaceChange::Lighting(cube));
            for cube in dependencies {
                self.light_needs_update(cube, difference_magnitude);
            }
        }
        (difference_magnitude, info)
    }

    /// Compute the new lighting value for a cube.
    ///
    /// The returned vector of points lists those cubes which the computed value depends on
    /// (imprecisely; empty cubes passed through are not listed).
    #[inline]
    pub(crate) fn compute_lighting(
        &self,
        cube: GridPoint,
    ) -> (PackedLight, Vec<GridPoint>, LightingUpdateInfo) {
        // Accumulator of incoming light encountered.
        let mut incoming_light: RGB = RGB::ZERO;
        // Number of rays contributing to incoming_light.
        let mut total_rays = 0;
        // Cubes whose lighting value contributed to the incoming_light value.
        let mut dependencies: Vec<GridPoint> = Vec::new();
        // Diagnostics. TODO not written yet
        let mut info_rays: [Option<LightingUpdateRayInfo>; ALL_RAYS_COUNT] = [None; ALL_RAYS_COUNT];

        let ev_origin = self.get_evaluated(cube);
        if ev_origin.opaque {
            // Opaque blocks are always dark inside.
        } else {
            for face_ray_data in &*LIGHT_RAYS {
                let face = face_ray_data.face;
                // We can and should skip these rays if:
                // * There is no surface facing into this cube that would reflect arriving light into it.
                // * There is no surface *within* this cube, ditto.
                // * The ray isn't going to directly hit a light source.
                // (Without this last special case, opaque light sources won't look bright.)
                if !self
                    .get_evaluated(cube + face.opposite().normal_vector())
                    .visible
                    && !ev_origin.visible
                    && self
                        .get_evaluated(cube + face.normal_vector())
                        .attributes
                        .light_emission
                        == RGB::ZERO
                {
                    continue;
                }

                'each_ray: for ray in &face_ray_data.rays[..] {
                    let translated_ray =
                        ray.translate(cube.cast::<FreeCoordinate>().unwrap().to_vec());
                    let raycaster = translated_ray.cast().within_grid(*self.grid());

                    // Fraction of the light value that is to be determined by future, rather than past,
                    // tracing; starts at 1.0 and decreases as opaque surfaces are encountered.
                    let mut ray_alpha = 1.0_f32;

                    let info = &mut info_rays[total_rays];

                    'raycast: for hit in raycaster {
                        let ev_hit = self.get_evaluated(hit.cube);
                        if !ev_hit.visible {
                            // Completely transparent block is passed through.
                            continue 'raycast;
                        }

                        // TODO: Implement blocks with some faces opaque.
                        if ev_hit.opaque {
                            // On striking a fully opaque block, we use the light value from its
                            // adjacent cube as the light falling on that face.
                            let light_cube = hit.previous_cube();
                            if light_cube == cube {
                                // Don't read the value we're trying to recalculate.
                                // We hit an opaque block, so this ray is stopping.
                                continue 'each_ray;
                            }
                            let stored_light = self.get_lighting(light_cube);
                            let light_from_struck_face =
                                ev_hit.attributes.light_emission + stored_light.into();
                            incoming_light += light_from_struck_face * ray_alpha;
                            dependencies.push(light_cube);
                            // This terminates the raycast; we don't bounce rays
                            // (diffuse reflections, not specular/mirror).
                            ray_alpha = 0.0;

                            // Diagnostics. TODO: Track transparency to some extent.
                            *info = Some(LightingUpdateRayInfo {
                                ray: Ray {
                                    origin: translated_ray.origin,
                                    direction: translated_ray.direction * 10.0, // TODO: translate hit position into ray
                                },
                                trigger_cube: hit.cube,
                                value_cube: light_cube,
                                value: stored_light,
                            });

                            break;
                        } else {
                            // Block is partly transparent and light should pass through.
                            let light_cube = hit.cube;
                            if light_cube == cube {
                                // Don't read the value we're trying to recalculate.
                                continue 'raycast;
                            }
                            // 'coverage' is what fraction of the light ray we assume to hit this block,
                            // as opposed to passing through it.
                            // TODO: Compute coverage in EvaluatedBlock.
                            let coverage = 0.25;
                            incoming_light += (ev_hit.attributes.light_emission * ray_alpha
                                + self.get_lighting(light_cube).into())
                                * coverage;
                            ray_alpha *= 1.0 - coverage;
                            dependencies.push(hit.cube);
                        }
                    }
                    // TODO: set *info even if we hit the sky

                    // Note that if ray_alpha has reached zero, this has no effect.
                    incoming_light += self.sky_color() * ray_alpha;
                    total_rays += 1;
                }
            }
        }

        // Compare and set new value. Note that we MUST compare the packed value so that
        // changes are detected in terms of the low-resolution values.

        // if total_rays is zero then incoming_light is zero so the result will be zero.
        // We just need to avoid dividing by zero.
        let scale = NotNan::new(1.0 / total_rays.max(1) as f32).unwrap();
        let new_light_value: PackedLight = (incoming_light * scale).into();
        (
            new_light_value,
            dependencies,
            LightingUpdateInfo {
                cube,
                result: new_light_value,
                rays: info_rays,
            },
        )
    }
}

/// Diagnostic data returned by lighting updates.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LightingUpdateInfo {
    cube: GridPoint,
    result: PackedLight,
    rays: [Option<LightingUpdateRayInfo>; ALL_RAYS_COUNT],
}

impl Geometry for LightingUpdateInfo {
    type Coord = FreeCoordinate;

    fn translate(self, _offset: impl Into<Vector3<FreeCoordinate>>) -> Self {
        unimplemented!();
    }

    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<Point3<FreeCoordinate>>,
    {
        // Draw output cube
        AAB::from_cube(self.cube)
            .enlarge(0.1)
            .wireframe_points(output);
        // Draw rays
        for ray_info in &self.rays {
            if let Some(ray_info) = ray_info {
                ray_info.wireframe_points(output);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LightingUpdateRayInfo {
    ray: Ray,
    trigger_cube: GridPoint,
    value_cube: GridPoint,
    value: PackedLight,
}

impl Geometry for LightingUpdateRayInfo {
    type Coord = FreeCoordinate;

    fn translate(self, _offset: impl Into<Vector3<FreeCoordinate>>) -> Self {
        unimplemented!();
    }

    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<Point3<FreeCoordinate>>,
    {
        // TODO: represent self.value somehoe
        AAB::from_cube(self.value_cube)
            .enlarge(0.01)
            .wireframe_points(output);
        self.ray.wireframe_points(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Space;
    use std::time::Duration;

    #[test]
    fn initial_lighting_value() {
        let space = Space::empty_positive(1, 1, 1);
        assert_eq!(
            PackedLight::from(space.sky_color()),
            space.get_lighting((0, 0, 0))
        );
    }

    #[test]
    fn out_of_bounds_lighting_value() {
        let space = Space::empty_positive(1, 1, 1);
        assert_eq!(
            PackedLight::from(space.sky_color()),
            space.get_lighting((-1, 0, 0))
        );
    }

    #[test]
    fn step() {
        let mut space = Space::empty_positive(3, 1, 1);
        let former_sky_light = PackedLight::from(space.sky_color());
        space.set_sky_color(RGB::new(1.0, 0.0, 0.0));
        let new_sky_light = PackedLight::from(space.sky_color());

        space.set((0, 0, 0), &RGB::ONE.into()).unwrap();
        // Not changed yet... except for the now-opaque block
        assert_eq!(space.get_lighting((0, 0, 0)), PackedLight::ZERO);
        assert_eq!(space.get_lighting((1, 0, 0)), former_sky_light);
        assert_eq!(space.get_lighting((2, 0, 0)), former_sky_light);

        // Duration doesn't currently matter
        let info = space.step(Duration::from_millis(10));
        assert_eq!(
            info,
            SpaceStepInfo {
                light_update_count: 1,
                light_queue_count: 0,
                max_light_update_difference: new_sky_light.difference_magnitude(former_sky_light),
                ..SpaceStepInfo::default()
            }
        );

        assert_eq!(space.get_lighting((0, 0, 0)), PackedLight::ZERO); // opaque
        assert_eq!(space.get_lighting((1, 0, 0)), new_sky_light); // updated
        assert_eq!(space.get_lighting((2, 0, 0)), former_sky_light); // not updated
    }

    // TODO: test sky lighting propagation onto blocks after quiescing

    // TODO: test a single semi-transparent block will receive and diffuse light
}
