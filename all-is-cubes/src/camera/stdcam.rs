use cgmath::{One, Point2};
use crate::camera::{Camera, GraphicsOptions, ViewTransform, Viewport};
use crate::character::{Character, Cursor};
use crate::listen::{DirtyFlag, ListenableCell, ListenableSource};
use crate::math::FreeCoordinate;
use crate::space::Space;
use crate::universe::{URef, Universe};
/// A collection of values associated with each of the layers of graphics that
/// is normally drawn (HUD on top of world, currently) by [`HeadlessRenderer`] or
/// other renderers.
///
/// [`HeadlessRenderer`]: crate::camera::HeadlessRenderer
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Layers<T> {
    /// The game world.
    pub world: T,
    /// The user interface, or HUD, drawn in front of the world.
    pub ui: T,
}
impl<T> Layers<T> {
    pub(crate) fn as_refs(&self) -> Layers<&T> {
        loop {}
    }
    pub(crate) fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Layers<U> {
        loop {}
    }
    #[doc(hidden)]
    pub fn try_map_ref<U, E>(
        &self,
        mut f: impl FnMut(&T) -> Result<U, E>,
    ) -> Result<Layers<U>, E> {
        loop {}
    }
}
/// Bundle of inputs specifying the “standard” configuration of [`Camera`]s and other
/// things to render an All is Cubes scene and user interface.
///
/// All of its data is provided through [`ListenableSource`]s, and consists of:
///
/// * [`GraphicsOptions`].
/// * A [`Viewport`] specifying the dimensions of image to render.
/// * A [`URef`] to the [`Character`] whose eyes we look through to render the “world”
///   [`Space`].
/// * A [`URef`] to the UI/HUD [`Space`] overlaid on the world, if any.
///
/// When [`StandardCameras::update()`] is called, all of these data sources are read
/// and used to update the [`Camera`] data. Those cameras, and copies of the input
/// data, are then available for use while rendering.
///
/// Because every input is a [`ListenableSource`], it is never necessary to call a setter.
/// Every [`StandardCameras`] which was created with the same sources will have the same
/// results (after `update()`).
///
/// Design note: The sense in which this is “standard” is that if an application wished
/// to, for example, have multiple views into the same [`Space`], it would need to create
/// additional [`Camera`]s (or multiple [`StandardCameras`]) and update them itself.
#[derive(Debug)]
pub struct StandardCameras {
    /// Cameras are synced with this
    graphics_options: ListenableSource<GraphicsOptions>,
    graphics_options_dirty: DirtyFlag,
    character_source: ListenableSource<Option<URef<Character>>>,
    /// Tracks whether the character was replaced (not whether its view changed).
    character_dirty: DirtyFlag,
    character: Option<URef<Character>>,
    /// Cached and listenable version of character's space.
    /// TODO: This should be in a Layers along with ui_state...?
    world_space: ListenableCell<Option<URef<Space>>>,
    ui_source: ListenableSource<UiViewState>,
    ui_dirty: DirtyFlag,
    ui_space: Option<URef<Space>>,
    viewport_source: ListenableSource<Viewport>,
    viewport_dirty: DirtyFlag,
    cameras: Layers<Camera>,
}
impl StandardCameras {
    /// Most general constructor; hidden because the details needed might vary and so we
    /// want to discourage use of this directly.
    #[doc(hidden)]
    pub fn new(
        graphics_options: ListenableSource<GraphicsOptions>,
        viewport_source: ListenableSource<Viewport>,
        character_source: ListenableSource<Option<URef<Character>>>,
        ui_source: ListenableSource<UiViewState>,
    ) -> Self {
        loop {}
    }
    #[doc(hidden)]
    pub fn from_constant_for_test(
        graphics_options: GraphicsOptions,
        viewport: Viewport,
        universe: &Universe,
    ) -> Self {
        loop {}
    }
    /// Updates camera state from data sources.
    ///
    /// This should be called at the beginning of each frame or as needed when the
    /// cameras are to be used.
    pub fn update(&mut self) {
        loop {}
    }
    /// Returns current graphics options as of the last [`update()`](Self::update).
    pub fn graphics_options(&self) -> &GraphicsOptions {
        loop {}
    }
    /// Returns a clone of the source of graphics options that this [`StandardCameras`]
    /// was created with.
    pub fn graphics_options_source(&self) -> ListenableSource<GraphicsOptions> {
        loop {}
    }
    /// Returns [`Camera`]s appropriate for drawing each graphical layer.
    pub fn cameras(&self) -> &Layers<Camera> {
        loop {}
    }
    /// Returns the character's viewpoint to draw in the world layer.
    /// May be [`None`] if there is no current character.
    pub fn character(&self) -> Option<&URef<Character>> {
        loop {}
    }
    /// Returns the space that should be drawn as the game world, using `self.cameras().world`.
    ///
    /// This is a [`ListenableSource`] to make it simple to cache the Space rendering data and
    /// follow space transitions.
    /// It updates when [`Self::update()`] is called.
    pub fn world_space(&self) -> ListenableSource<Option<URef<Space>>> {
        loop {}
    }
    /// Returns the UI space, that should be drawn on top of the world using `self.cameras().ui`.
    ///
    /// This implements [`GraphicsOptions::show_ui`] by returning [`None`] when the option is
    /// false.
    ///
    /// TODO: Make this also a [`ListenableSource`]
    pub fn ui_space(&self) -> Option<&URef<Space>> {
        loop {}
    }
    /// Returns the current viewport.
    ///
    /// This is always equal to the viewports of all managed [`Camera`]s,
    /// and only updates when [`StandardCameras::update()`] is called.
    pub fn viewport(&self) -> Viewport {
        loop {}
    }
    /// Returns a clone of the viewport source this is following.
    pub fn viewport_source(&self) -> ListenableSource<Viewport> {
        loop {}
    }
    /// Perform a raycast through these cameras to find what the cursor hits.
    ///
    /// Make sure to call [`StandardCameras::update`] first so that the cameras are
    /// up to date with game state.
    pub fn project_cursor(&self, ndc_pos: Point2<FreeCoordinate>) -> Option<Cursor> {
        loop {}
    }
}
impl Clone for StandardCameras {
    /// Returns a [`StandardCameras`] which tracks the same data sources (graphics
    /// options, scene sources, viewport) as `self`, but whose local state (such as
    /// the last updated camera state) is independent.
    fn clone(&self) -> Self {
        loop {}
    }
}
/// Specifies what to render for the UI layer in front of the world.
///
/// This struct contains all the information needed to know how to render the UI
/// *specifically* (distinct from the world). It differs from [`Camera`] in that it
/// includes the [`Space`] and excludes the viewport.
///
/// TODO: This struct needs a better name. And is it good for non-UI, too?
/// Note that we may wish to revise this bundle if we start having continuously changing
/// `view_transform`.
#[derive(Clone, Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct UiViewState {
    /// The [`Space`] to render as the UI.
    pub space: Option<URef<Space>>,
    /// The viewpoint to render the `space` from.
    pub view_transform: ViewTransform,
    /// The graphics options to render the `space` with.
    pub graphics_options: GraphicsOptions,
}
impl Default for UiViewState {
    /// Draws no space, with default graphics options.
    fn default() -> Self {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Space;
    use crate::universe::{Universe, UniverseIndex};
    #[test]
    fn cameras_follow_character_and_world() {
        loop {}
    }
    #[test]
    fn cameras_clone() {
        loop {}
    }
}
