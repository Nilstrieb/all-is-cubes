//! Continuously moving objects and collision.
use crate::math::FreeCoordinate;
mod body;
pub use body::*;
mod collision;
pub use collision::*;
/// Close-but-not-intersecting objects are set to this separation.
pub(crate) const POSITION_EPSILON: FreeCoordinate = 1e-6 * 1e-6;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Resolution, AIR};
    use crate::content::{make_slab, make_some_blocks};
    use crate::math::{Aab, CubeFace, Face7, Geometry, GridAab, GridPoint};
    use crate::space::{Space, SpacePhysics};
    use crate::time::Tick;
    use crate::universe::Universe;
    use cgmath::{EuclideanSpace, InnerSpace as _, Point3, Vector3, Zero as _};
    use ordered_float::NotNan;
    use rand::prelude::SliceRandom as _;
    use rand::{Rng as _, SeedableRng as _};
    use std::collections::VecDeque;
    fn collision_noop(_: Contact) {}
    fn test_body() -> Body {
        loop {}
    }
    #[test]
    fn freefall_no_gravity() {
        loop {}
    }
    #[test]
    fn freefall_with_gravity() {
        loop {}
    }
    #[test]
    fn paused_does_not_move() {
        loop {}
    }
    #[test]
    fn falling_collision() {
        loop {}
    }
    #[test]
    fn falling_collision_partial_block() {
        loop {}
    }
    #[test]
    fn push_out_simple() {
        loop {}
    }
    #[test]
    fn no_passing_through_blocks() {
        loop {}
    }
    #[test]
    fn position_nan() {
        loop {}
    }
    #[test]
    fn velocity_nan() {
        loop {}
    }
    #[test]
    fn velocity_limit() {
        loop {}
    }
    /// Takes the maximum length on all coordinate axes; all points forming a cube
    /// centered on the origin will have the same value for this norm.
    ///
    /// See also <https://en.wikipedia.org/wiki/Uniform_norm>
    fn max_norm<S: num_traits::real::Real>(v: Vector3<S>) -> S {
        loop {}
    }
}
