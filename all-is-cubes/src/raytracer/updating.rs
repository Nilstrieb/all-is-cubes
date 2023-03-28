use std::collections::HashSet;
use std::fmt;
use std::mem;
use std::sync::{Arc, Mutex, Weak};
use crate::block::AIR;
use crate::camera::GraphicsOptions;
use crate::content::palette;
use crate::listen::{Listen as _, ListenableSource, Listener};
use crate::math::GridPoint;
use crate::raytracer::{
    RtBlockData, RtOptionsRef, SpaceRaytracer, TracingBlock, TracingCubeData,
};
use crate::space::{BlockIndex, Space, SpaceChange};
use crate::universe::{RefError, URef};
/// Manages a [`SpaceRaytracer`] so that it can be cheaply updated when the [`Space`] is
/// changed.
pub struct UpdatingSpaceRaytracer<D: RtBlockData> {
    space: URef<Space>,
    graphics_options: ListenableSource<GraphicsOptions>,
    custom_options: ListenableSource<D::Options>,
    state: SpaceRaytracer<D>,
    todo: Arc<Mutex<SrtTodo>>,
}
impl<D: RtBlockData> fmt::Debug for UpdatingSpaceRaytracer<D>
where
    D::Options: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<D: RtBlockData> UpdatingSpaceRaytracer<D>
where
    D::Options: Clone + Sync + 'static,
{
    /// Construct a new [`UpdatingSpaceRaytracer`] which follows the given `space`.
    ///
    /// The space is not accessed (and thus, nothing is updated) until the first call to
    /// [`update()`](Self::update). (This is done so that this constructor cannot fail and
    /// so the space is accessed on a consistent schedule.)
    pub fn new(
        space: URef<Space>,
        graphics_options: ListenableSource<GraphicsOptions>,
        custom_options: ListenableSource<D::Options>,
    ) -> Self {
        loop {}
    }
    /// Returns the [`Space`] this is synchronized with.
    pub fn space(&self) -> &URef<Space> {
        loop {}
    }
    /// Returns the [`SpaceRaytracer`] that may be used for tracing.
    /// Its contents are current as of the last [`UpdatingSpaceRaytracer::update()`].
    pub fn get(&self) -> &SpaceRaytracer<D> {
        loop {}
    }
    /// Reads the previously provided [`Space`] and updates the local copy of its contents.
    ///
    /// Returns an error if reading fails.
    pub fn update(&mut self) -> Result<(), RefError> {
        loop {}
    }
}
#[derive(Debug)]
struct SrtTodo {
    /// Listener upon the space is not yet installed.
    listener: bool,
    /// All blocks and cubes must be updated.
    everything: bool,
    blocks: HashSet<BlockIndex>,
    cubes: HashSet<GridPoint>,
}
/// [`Listener`] adapter for [`SpaceRendererTodo`].
#[derive(Clone, Debug)]
struct TodoListener(Weak<Mutex<SrtTodo>>);
impl Listener<SpaceChange> for TodoListener {
    fn receive(&self, message: SpaceChange) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::AIR;
    use crate::camera::{eye_for_look_at, Camera, Viewport};
    use crate::content::make_some_voxel_blocks;
    use crate::raytracer::{CharacterBuf, CharacterRtData};
    use crate::universe::Universe;
    use crate::util::{CustomFormat, Unquote};
    use cgmath::{Decomposed, Transform as _, Vector2, Vector3};
    use pretty_assertions::assert_eq;
    struct EquivalenceTester {
        camera: Camera,
        space: URef<Space>,
        graphics_options: ListenableSource<GraphicsOptions>,
        custom_options: ListenableSource<()>,
        updating: UpdatingSpaceRaytracer<CharacterRtData>,
    }
    impl EquivalenceTester {
        fn new(space: URef<Space>) -> Self {
            loop {}
        }
        fn update_and_assert(&mut self) -> Result<(), RefError> {
            loop {}
        }
    }
    #[test]
    fn updating_is_equivalent() {
        loop {}
    }
    #[test]
    fn updating_after_space_is_unavailable() {
        loop {}
    }
}
