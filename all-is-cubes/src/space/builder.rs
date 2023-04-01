

use crate::space::{GridAab};
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
pub(crate) struct SpaceBuilder<B> {
    pub(super) bounds: B,
}
/// Helper for [`SpaceBuilder::bounds_if_not_set()`]. Do not call or implement this trait.
pub(crate) trait SpaceBuilderBounds: sbb::SbbSealed + Sized {
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
    pub(crate) trait SbbSealed {}
    impl SbbSealed for () {}
    impl SbbSealed for GridAab {}
}
