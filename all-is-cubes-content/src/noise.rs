// Copyright 2020-2022 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

use all_is_cubes::cgmath::Vector3;
use noise::NoiseFn;

use all_is_cubes::block::Resolution;
use all_is_cubes::math::{cube_to_midpoint, GridPoint};
use all_is_cubes::space::{Grid, GridArray};

/// Generates a [`Block`]-shape of noise values from a [`NoiseFn`].
///
/// As a convenience, it also accepts a postprocessing function that is allowed to return
/// any type.
///
/// The [`noise`] library currently uses `&dyn NoiseFn` ubiquitously in all combiners and
/// modifiers, which does not implement [`Send`] and so cannot be used inside futures that
/// need to be [`Send`]. As a stopgap before a release that fixes this (or switching to a
/// different noise library), we use this utility function to pre-compute all the noise we
/// want for patterns in a [`Block`].
pub(crate) fn array_of_noise<O>(
    resolution: Resolution,
    noise_fn: &impl NoiseFn<[f64; 3]>,
    mut postprocess: impl FnMut(f64) -> O,
) -> GridArray<O> {
    GridArray::from_fn(Grid::for_block(resolution), |cube| {
        postprocess(noise_fn.get(cube_to_midpoint(cube).into()))
    })
}

/// Extension trait for [`noise::NoiseFn`] which makes it usable with our [`GridPoint`]s.
pub trait NoiseFnExt: NoiseFn<[f64; 3]> {
    /// Sample the noise at the center of the given cube. That is, convert the integer
    /// vector to `f64`, add 0.5 to all coordinates, and call [`NoiseFn::get`].
    ///
    /// This offset is appropriate for the most resolution-independent sampling, or
    /// symmetric shapes with even-numbered widths.
    fn at_cube(&self, cube: GridPoint) -> f64;

    /// As [`NoiseFn::get`], but converting from integer. Unlike [`NoiseFnExt::at_cube`],
    /// does not apply any offset.
    fn at_grid(&self, point: GridPoint) -> f64;
}
impl<T> NoiseFnExt for T
where
    T: NoiseFn<[f64; 3]> + Sized,
{
    fn at_cube(&self, cube: GridPoint) -> f64
    where
        Self: Sized,
    {
        let point = cube.map(f64::from) + Vector3::new(0.5, 0.5, 0.5);
        NoiseFn::get(&self, point.into())
    }

    fn at_grid(&self, point: GridPoint) -> f64
    where
        Self: Sized,
    {
        let point = point.map(f64::from);
        NoiseFn::get(&self, point.into())
    }
}