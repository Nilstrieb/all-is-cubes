use cgmath::{EuclideanSpace, InnerSpace, Point3};
use crate::block::Block;
use crate::character::Spawn;
use crate::math::{FreeCoordinate, Rgb};
use crate::space::{GridAab, LightPhysics, Space, SpacePhysics};
/// Tool for constructing new [`Space`]s.
///
/// To create one, call [`Space::builder()`](Space::builder).
///
/// TODO: Allow specifying behaviors.
///
/// # Type parameters
///
/// * `B` is either `()` or `GridAab` according to whether the bounds have been specified.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct SpaceBuilder<B> {
    pub(super) bounds: B,
    pub(super) spawn: Option<Spawn>,
    pub(super) physics: SpacePhysics,
    pub(super) initial_fill: Block,
}
/// Helper for [`SpaceBuilder::bounds_if_not_set()`]. Do not call or implement this trait.
pub trait SpaceBuilderBounds: sbb::SbbSealed + Sized {
    /// Set the bounds unless they have already been set.
    ///
    /// This function is an implementation detail; call
    /// [`SpaceBuilder::bounds_if_not_set()`] instead.
    #[doc(hidden)]
    fn bounds_if_not_set(
        builder: SpaceBuilder<Self>,
        bounds_fn: impl FnOnce() -> GridAab,
    ) -> SpaceBuilder<GridAab>;
}
/// Module for sealed trait
mod sbb {
    use super::*;
    #[doc(hidden)]
    pub trait SbbSealed {}
    impl SbbSealed for () {}
    impl SbbSealed for GridAab {}
}
