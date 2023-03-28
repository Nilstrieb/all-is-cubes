use cgmath::{EuclideanSpace, InnerSpace, Point3};
use crate::block::{Block};
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
impl<B> SpaceBuilder<B> {
    /// Sets the [`Block`] that the space's volume will be filled with.
    ///
    /// Caution: If [evaluating](Block::evaluate) the block fails, constructing the space
    /// will panic. Future versions may improve on this.
    pub fn filled_with(mut self, block: Block) -> Self {
        loop {}
    }
    /// Sets the value for [`Space::physics`], which determines global characteristics
    /// of gravity and light in the space.
    pub fn physics(mut self, physics: SpacePhysics) -> Self {
        loop {}
    }
    /// Sets the value of [`SpacePhysics::sky_color`] for the space.
    pub fn sky_color(mut self, color: Rgb) -> Self {
        loop {}
    }
    /// Sets the value of [`SpacePhysics::light`] for the space, which determines the
    /// behavior of light within the space.
    pub fn light_physics(mut self, light_physics: LightPhysics) -> Self {
        loop {}
    }
    /// Sets the value for [`Space::spawn`], which determines the default circumstances of
    /// new characters.
    ///
    /// If not set, the default spawn position will be [0, 0, 0].
    /// (TODO: Improve this and document it centrally.)
    pub fn spawn(mut self, spawn: Spawn) -> Self {
        loop {}
    }
}
impl<B: SpaceBuilderBounds> SpaceBuilder<B> {
    /// Set the bounds unless they have already been set.
    pub fn bounds_if_not_set(
        self,
        bounds_fn: impl FnOnce() -> GridAab,
    ) -> SpaceBuilder<GridAab> {
        loop {}
    }
}
impl SpaceBuilder<()> {
    /// Use [`SpaceBuilder::default()`] as the public way to call this.
    pub(super) const fn new() -> Self {
        loop {}
    }
    /// Set the bounds of the space, outside which no blocks may be placed.
    pub fn bounds(self, bounds: GridAab) -> SpaceBuilder<GridAab> {
        loop {}
    }
}
impl SpaceBuilder<GridAab> {
    /// Sets the default spawn location of new characters.
    ///
    /// Panics if any of the given coordinates is infinite or NaN.
    #[track_caller]
    pub fn spawn_position(mut self, position: Point3<FreeCoordinate>) -> Self {
        loop {}
    }
    /// Construct a [`Space`] with the contents and settings from this builder.
    ///
    /// The builder must have had bounds specified.
    pub fn build(self) -> Space {
        loop {}
    }
}
impl Default for SpaceBuilder<()> {
    fn default() -> Self {
        loop {}
    }
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
impl SpaceBuilderBounds for () {
    fn bounds_if_not_set(
        builder: SpaceBuilder<Self>,
        bounds_fn: impl FnOnce() -> GridAab,
    ) -> SpaceBuilder<GridAab> {
        loop {}
    }
}
impl SpaceBuilderBounds for GridAab {
    fn bounds_if_not_set(
        builder: SpaceBuilder<Self>,
        _bounds_fn: impl FnOnce() -> GridAab,
    ) -> SpaceBuilder<GridAab> {
        loop {}
    }
}
/// Module for sealed trait
mod sbb {
    use super::*;
    #[doc(hidden)]
    pub trait SbbSealed {}
    impl SbbSealed for () {}
    impl SbbSealed for GridAab {}
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for Space {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use crate::math::Rgba;
    use super::*;
    #[test]
    fn defaults() {
        loop {}
    }
    #[test]
    fn filled_with() {
        loop {}
    }
    #[test]
    fn bounds_if_not_set_when_not_set() {
        loop {}
    }
    #[test]
    fn bounds_if_not_set_when_already_set() {
        loop {}
    }
}
