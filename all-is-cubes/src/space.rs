//! That which contains many blocks.
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, Weak};
use cgmath::Vector3;
use instant::Duration;
use crate::behavior::{BehaviorSet};
use crate::block::{
    Block, BlockChange, EvalBlockError, EvaluatedBlock, Resolution, AIR, AIR_EVALUATED,
};
#[cfg(doc)]
use crate::character::Character;
use crate::character::Spawn;
use crate::content::palette;
use crate::drawing::DrawingPlane;

use crate::listen::{Gate, Listen, Listener, Notifier};
use crate::math::{
    FreeCoordinate, GridAab, GridArray, GridCoordinate, GridMatrix, GridPoint,
    GridRotation, NotNan, Rgb,
};
use crate::time::Tick;
use crate::transaction::{Merge, Transaction as _};
use crate::universe::{RefVisitor, URef, UniverseTransaction, VisitRefs};
use crate::util::TimeStats;
use crate::util::{CustomFormat, StatusText};
mod builder;
pub(crate) use builder::{SpaceBuilder, SpaceBuilderBounds};
mod light;

use light::{LightUpdateQueue};
pub(crate) use light::{LightUpdatesInfo, PackedLight};
mod space_txn;

/// Container for [`Block`]s arranged in three-dimensional space. The main “game world”
/// data structure.
pub struct Space {
    bounds: GridAab,
    /// Lookup from `Block` value to the index by which it is represented in
    /// the array.
    block_to_index: HashMap<Block, BlockIndex>,
    /// Lookup from arbitrarily assigned indices (used in `contents`) to data for them.
    block_data: Vec<SpaceBlockData>,
    /// The blocks in the space, stored compactly:
    ///
    /// * Coordinates are transformed to indices by [`GridAab::index`].
    /// * Each element is an index into [`Self::block_data`].
    contents: Box<[BlockIndex]>,
    /// Parallel array to `contents` for lighting data.
    pub(crate) lighting: Box<[PackedLight]>,
    /// Queue of cubes whose light values should be updated.
    light_update_queue: LightUpdateQueue,
    /// Debug log of the updated cubes from last frame.
    /// Empty unless this debug function is enabled.
    #[doc(hidden)]
    pub(crate) last_light_updates: Vec<GridPoint>,
    /// Global characteristics such as the behavior of light and gravity.
    physics: SpacePhysics,
    /// A converted copy of `physics.sky_color`.
    packed_sky_color: PackedLight,
    behaviors: BehaviorSet<Space>,
    spawn: Spawn,
    /// Cubes that should be checked on the next call to step()
    cubes_wanting_ticks: HashSet<GridPoint>,
    notifier: Notifier<SpaceChange>,
    /// Storage for incoming change notifications from blocks.
    todo: Arc<Mutex<SpaceTodo>>,
}
/// Information about the interpretation of a block index.
///
/// Design note: This doubles as an internal data structure for [`Space`]. While we'll
/// try to keep it available, this interface has a higher risk of needing to change
/// incompatibility.
pub(crate) struct SpaceBlockData {
    /// The block itself.
    block: Block,
    /// Number of uses of this block in the space.
    count: usize,
    evaluated: EvaluatedBlock,
    #[allow(dead_code)]
    block_listen_gate: Option<Gate>,
}
impl fmt::Debug for Space {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Debug for SpaceBlockData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Number used to identify distinct blocks within a [`Space`].
pub(crate) type BlockIndex = u16;
impl Space {
    /// Returns a [`SpaceBuilder`] configured for a block,
    /// which may be used to construct a new [`Space`].
    ///
    /// This means that its bounds are as per [`GridAab::for_block()`], and its
    /// [`physics`](Self::physics) is [`SpacePhysics::DEFAULT_FOR_BLOCK`].
    pub(crate) fn for_block(resolution: Resolution) -> SpaceBuilder<GridAab> {
        loop {}
    }
    /// Returns a [`SpaceBuilder`] with the given bounds and all default values,
    /// which may be used to construct a new [`Space`].
    pub(crate) fn builder(bounds: GridAab) -> SpaceBuilder<GridAab> {
        loop {}
    }
    /// Constructs a [`Space`] that is entirely filled with [`AIR`].
    ///
    /// Equivalent to `Space::builder(bounds).build()`
    pub(crate) fn empty(bounds: GridAab) -> Space {
        loop {}
    }
    /// Implementation of [`SpaceBuilder`]'s terminal methods.
    fn new_from_builder(builder: SpaceBuilder<GridAab>) -> Self {
        loop {}
    }
    /// Constructs a `Space` that is entirely empty and whose coordinate system
    /// is in the +X+Y+Z octant. This is a shorthand intended mainly for tests.
    pub(crate) fn empty_positive(
        wx: GridCoordinate,
        wy: GridCoordinate,
        wz: GridCoordinate,
    ) -> Space {
        loop {}
    }
    /// Returns the [`GridAab`] describing the bounds of this space; no blocks may exist
    /// outside it.
    pub(crate) fn bounds(&self) -> GridAab {
        loop {}
    }
    /// Returns the internal unstable numeric ID for the block at the given position,
    /// which may be mapped to a [`Block`] by [`Space::block_data`].
    /// If you are looking for *simple* access, use `space[position]` (the
    /// [`std::ops::Index`] trait) instead.
    ///
    /// These IDs may be used to perform efficient processing of many blocks, but they
    /// may be renumbered after any mutation.
    #[inline(always)]
    pub(crate) fn get_block_index(
        &self,
        position: impl Into<GridPoint>,
    ) -> Option<BlockIndex> {
        loop {}
    }
    /// Copy data out of a portion of the space in a caller-chosen format.
    ///
    /// If the provided [`GridAab`] contains portions outside of this space's bounds,
    /// those positions in the output will be treated as if they are filled with [`AIR`]
    /// and lit by [`SpacePhysics::sky_color`].
    pub(crate) fn extract<V>(
        &self,
        subgrid: GridAab,
        mut extractor: impl FnMut(Option<BlockIndex>, &SpaceBlockData, PackedLight) -> V,
    ) -> GridArray<V> {
        loop {}
    }
    /// Gets the [`EvaluatedBlock`] of the block in this space at the given position.
    #[inline(always)]
    pub(crate) fn get_evaluated(
        &self,
        position: impl Into<GridPoint>,
    ) -> &EvaluatedBlock {
        loop {}
    }
    /// Returns the light occupying the given cube.
    ///
    /// This value may be considered as representing the average of the light reflecting
    /// off of all surfaces within, or immediately adjacent to and facing toward, this cube.
    /// If there are no such surfaces, or if the given position is out of bounds, the result
    /// is arbitrary. If the position is within an opaque block, the result is black.
    ///
    /// Lighting is updated asynchronously after modifications, so all above claims about
    /// the meaning of this value are actually “will eventually be, if no more changes are
    /// made”.
    #[inline(always)]
    pub(crate) fn get_lighting(&self, position: impl Into<GridPoint>) -> PackedLight {
        loop {}
    }
    /// Replace the block in this space at the given position.
    ///
    /// If the position is out of bounds, there is no effect.
    ///
    /// ```
    /// use all_is_cubes::block::*;
    /// use all_is_cubes::math::Rgba;
    /// use all_is_cubes::space::Space;
    /// let mut space = Space::empty_positive(1, 1, 1);
    /// let a_block = Block::builder().color(Rgba::new(1.0, 0.0, 0.0, 1.0)).build();
    /// space.set((0, 0, 0), &a_block);
    /// assert_eq!(space[(0, 0, 0)], a_block);
    /// ```
    pub(crate) fn set<'a>(
        &mut self,
        position: impl Into<GridPoint>,
        block: impl Into<Cow<'a, Block>>,
    ) -> Result<bool, SetCubeError> {
        loop {}
    }
    fn set_impl(
        &mut self,
        position: GridPoint,
        block: Cow<'_, Block>,
    ) -> Result<bool, SetCubeError> {
        loop {}
    }
    /// Implement the consequences of changing a block.
    ///
    /// `content_index` is redundant with `position` but saves computation.
    #[inline]
    fn side_effects_of_set(
        &mut self,
        block_index: BlockIndex,
        position: GridPoint,
        contents_index: usize,
    ) {
        loop {}
    }
    /// Replace blocks in `region` with a block computed by the function.
    ///
    /// The function may return a reference to a block or a block. If it returns [`None`],
    /// the existing block is left unchanged.
    ///
    /// The operation will stop on the first error, potentially leaving some blocks
    /// replaced. (Exception: If the `region` extends outside of
    /// [`self.bounds()`](Self::bounds), that will always be rejected before any changes
    /// are made.)
    ///
    /// ```
    /// use all_is_cubes::block::{AIR, Block};
    /// use all_is_cubes::math::{GridAab, Rgba};
    /// use all_is_cubes::space::Space;
    ///
    /// let mut space = Space::empty_positive(10, 10, 10);
    /// let a_block: Block = Rgba::new(1.0, 0.0, 0.0, 1.0).into();
    ///
    /// space.fill(GridAab::from_lower_size([0, 0, 0], [2, 1, 1]), |_point| Some(&a_block)).unwrap();
    ///
    /// assert_eq!(space[(0, 0, 0)], a_block);
    /// assert_eq!(space[(1, 0, 0)], a_block);
    /// assert_eq!(space[(0, 1, 0)], AIR);
    /// ```
    ///
    /// TODO: Support providing the previous block as a parameter (take cues from `extract`).
    ///
    /// See also [`Space::fill_uniform`] for filling a region with one block.
    pub(crate) fn fill<F, B>(
        &mut self,
        region: GridAab,
        mut function: F,
    ) -> Result<(), SetCubeError>
    where
        F: FnMut(GridPoint) -> Option<B>,
        B: std::borrow::Borrow<Block>,
    {
        loop {}
    }
    /// Replace blocks in `region` with the given block.
    ///
    /// TODO: Document error behavior
    ///
    /// ```
    /// use all_is_cubes::block::{AIR, Block};
    /// use all_is_cubes::math::{GridAab, Rgba};
    /// use all_is_cubes::space::Space;
    ///
    /// let mut space = Space::empty_positive(10, 10, 10);
    /// let a_block: Block = Rgba::new(1.0, 0.0, 0.0, 1.0).into();
    ///
    /// space.fill_uniform(GridAab::from_lower_size([0, 0, 0], [2, 1, 1]), &a_block).unwrap();
    ///
    /// assert_eq!(&space[(0, 0, 0)], &a_block);
    /// assert_eq!(&space[(1, 0, 0)], &a_block);
    /// assert_eq!(&space[(0, 1, 0)], &AIR);
    /// ```
    ///
    /// See also [`Space::fill`] for non-uniform fill and bulk copies.
    pub(crate) fn fill_uniform<'b>(
        &mut self,
        region: GridAab,
        block: impl Into<Cow<'b, Block>>,
    ) -> Result<(), SetCubeError> {
        loop {}
    }
    /// Provides an [`DrawTarget`](embedded_graphics::prelude::DrawTarget)
    /// adapter for 2.5D drawing.
    ///
    /// For more information on how to use this, see
    /// [`all_is_cubes::drawing`](crate::drawing).
    pub(crate) fn draw_target<C>(
        &mut self,
        transform: GridMatrix,
    ) -> DrawingPlane<'_, Space, C> {
        loop {}
    }
    /// Returns all distinct block types found in the space.
    ///
    /// TODO: This was invented for testing the indexing of blocks and should
    /// be replaced with something else *if* it only gets used for testing.
    pub(crate) fn distinct_blocks(&self) -> Vec<Block> {
        loop {}
    }
    /// Returns data about all the blocks assigned internal IDs (indices) in the space,
    /// as well as placeholder data for any deallocated indices.
    ///
    /// The indices of this slice correspond to the results of [`Space::get_block_index`].
    pub(crate) fn block_data(&self) -> &[SpaceBlockData] {
        loop {}
    }
    /// Advance time in the space.
    pub(crate) fn step(
        &mut self,
        self_ref: Option<&URef<Space>>,
        tick: Tick,
    ) -> (SpaceStepInfo, UniverseTransaction) {
        loop {}
    }
    /// Perform lighting updates until there are none left to do. Returns the number of
    /// updates performed.
    ///
    /// This may take a while. It is appropriate for when the goal is to
    /// render a fully lit scene non-interactively.
    ///
    /// `epsilon` specifies a threshold at which to stop doing updates.
    /// Zero means to run to full completion; one is the smallest unit of light level
    /// difference; and so on.
    pub(crate) fn evaluate_light(
        &mut self,
        epsilon: u8,
        mut progress_callback: impl FnMut(LightUpdatesInfo),
    ) -> usize {
        loop {}
    }
    /// Returns the current [`SpacePhysics`] data, which determines global characteristics
    /// such as the behavior of light and gravity.
    pub(crate) fn physics(&self) -> &SpacePhysics {
        loop {}
    }
    /// Sets the physics parameters, as per [`physics`](Self::physics).
    ///
    /// This may cause recomputation of lighting.
    pub(crate) fn set_physics(&mut self, physics: SpacePhysics) {
        loop {}
    }
    /// Returns the current default [`Spawn`], which determines where new [`Character`]s
    /// are placed in the space if no alternative applies.
    pub(crate) fn spawn(&self) -> &Spawn {
        loop {}
    }
    /// Sets the default [`Spawn`], which determines where new [`Character`]s are placed
    /// in the space if no alternative applies.
    pub(crate) fn set_spawn(&mut self, spawn: Spawn) {
        loop {}
    }
    /// Returns the [`BehaviorSet`] of behaviors attached to this space.
    pub(crate) fn behaviors(&self) -> &BehaviorSet<Space> {
        loop {}
    }
    /// Finds or assigns an index to denote the block.
    ///
    /// The caller is responsible for incrementing `self.block_data[index].count`.
    #[inline]
    fn ensure_block_index(
        &mut self,
        block: Cow<'_, Block>,
    ) -> Result<BlockIndex, SetCubeError> {
        loop {}
    }
    fn listener_for_block(&self, index: BlockIndex) -> SpaceBlockChangeListener {
        loop {}
    }
}
impl<T: Into<GridPoint>> std::ops::Index<T> for Space {
    type Output = Block;
    /// Gets a reference to the block in this space at the given position.
    ///
    /// If the position is out of bounds, returns [`AIR`].
    ///
    /// Note that [`Space`] does not implement [`IndexMut`](std::ops::IndexMut);
    /// use [`Space::set`] or [`Space::fill`] to modify blocks.
    #[inline(always)]
    fn index(&self, position: T) -> &Self::Output {
        loop {}
    }
}
impl VisitRefs for Space {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
impl Listen for Space {
    type Msg = SpaceChange;
    /// Registers a listener for mutations of this space.
    fn listen<L: Listener<SpaceChange> + Send + Sync + 'static>(&self, listener: L) {
        loop {}
    }
}
impl crate::behavior::BehaviorHost for Space {
    type Attachment = SpaceBehaviorAttachment;
}
impl SpaceBlockData {
    /// A `SpaceBlockData` value used to represent out-of-bounds or placeholder
    /// situations. The block is [`AIR`] and the count is always zero.
    pub(crate) const NOTHING: Self = Self {
        block: AIR,
        count: 0,
        evaluated: AIR_EVALUATED,
        block_listen_gate: None,
    };
    /// Value used to fill empty entries in the block data vector.
    /// This is the same value as [`SpaceBlockData::NOTHING`] but is not merely done
    /// by `.clone()` because I haven't decided whether providing [`Clone`] for
    /// `SpaceBlockData` is a good long-term API design decision.
    fn tombstone() -> Self {
        loop {}
    }
    fn new(
        block: Block,
        listener: impl Listener<BlockChange> + Clone + Send + Sync + 'static,
    ) -> Result<Self, SetCubeError> {
        loop {}
    }
    /// Returns the [`Block`] this data is about.
    pub(crate) fn block(&self) -> &Block {
        loop {}
    }
    /// Returns the [`EvaluatedBlock`] representation of the block.
    ///
    /// TODO: Describe when this may be stale.
    pub(crate) fn evaluated(&self) -> &EvaluatedBlock {
        loop {}
    }
}
/// Description of where in a [`Space`] a [`Behavior<Space>`](crate::behavior::Behavior)
/// exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpaceBehaviorAttachment {
    bounds: GridAab,
    rotation: GridRotation,
}
impl SpaceBehaviorAttachment {
    /// Constructs a new [`SpaceBehaviorAttachment`] with no rotation.
    pub(crate) fn new(bounds: GridAab) -> Self {
        loop {}
    }
    /// Returns the bounds of this attachment, which specify (without mandating) what
    /// region the behavior should affect.
    pub(crate) fn bounds(&self) -> GridAab {
        loop {}
    }
    /// Returns the rotation of this attachment, which specifies, if applicable, which
    /// orientation the behavior should operate in relative to the space.
    /// The exact meaning of this is up to the behavior.
    ///
    /// TODO: explain with an example once we have a good one
    pub(crate) fn rotation(&self) -> GridRotation {
        loop {}
    }
}
/// The global characteristics of a [`Space`], more or less independent of location within
/// the block grid.
///
/// This is a separate type so that [`Space`] does not need many miscellaneous accessors,
/// and so an instance of it can be reused for similar spaces (e.g.
/// [`DEFAULT_FOR_BLOCK`](Self::DEFAULT_FOR_BLOCK)).
#[derive(Clone, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct SpacePhysics {
    /// Gravity vector for moving objects, in cubes/s².
    ///
    /// TODO: Expand this to an enum which allows non-uniform gravity patterns.
    pub(crate) gravity: Vector3<NotNan<FreeCoordinate>>,
    /// Color of light arriving from outside the space, used for light calculation
    /// and rendering.
    ///
    /// TODO: Consider replacing this with some sort of cube map, spherical harmonics,
    /// or some such to allow for non-uniform illumination.
    pub(crate) sky_color: Rgb,
    /// Method used to compute the illumination of individual blocks.
    pub(crate) light: LightPhysics,
}
impl SpacePhysics {
    pub(crate) const DEFAULT: Self = Self {
        gravity: Vector3::new(notnan!(0.), notnan!(- 20.), notnan!(0.)),
        sky_color: palette::DAY_SKY_COLOR,
        light: LightPhysics::DEFAULT,
    };
    /// Recommended defaults for spaces which are going to define a [`Block`]'s voxels.
    /// In particular, disables light since it will not be used.
    pub(crate) const DEFAULT_FOR_BLOCK: Self = Self {
        gravity: Vector3::new(notnan!(0.), notnan!(0.), notnan!(0.)),
        sky_color: rgb_const!(0.5, 0.5, 0.5),
        light: LightPhysics::None,
    };
}
impl fmt::Debug for SpacePhysics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Default for SpacePhysics {
    fn default() -> Self {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for SpacePhysics {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// Method used to compute the illumination of individual blocks in a [`Space`].
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub(crate) enum LightPhysics {
    /// No light. All surface colors are taken exactly as displayed colors. The
    /// [`SpacePhysics::sky_color`] is used solely as a background color.
    None,
    /// Raycast-based light propagation and diffuse reflections.
    ///
    /// TODO: Need to provide a builder so that this can be constructed
    /// even when more parameters are added.
    Rays {
        /// The maximum distance a simulated light ray will travel; blocks farther than
        /// that distance apart will never have direct influence on each other.
        maximum_distance: u16,
    },
}
impl LightPhysics {
    pub(crate) const DEFAULT: Self = Self::Rays { maximum_distance: 30 };
}
impl Default for LightPhysics {
    fn default() -> Self {
        loop {}
    }
}
/// Ways that [`Space::set`] can fail to make a change.
///
/// Note that "already contained the given block" is considered a success.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum SetCubeError {
    /// The given cube or region is not within the bounds of this Space.
    #[error("{:?} is outside of the bounds {:?}", .modification, .space_bounds)]
    OutOfBounds {
        /// The cube or region where modification was attempted.
        modification: GridAab,
        /// The bounds of the space.
        space_bounds: GridAab,
    },
    /// [`Block::evaluate`] failed on a new block type.
    #[error("block evaluation failed: {0}")]
    EvalBlock(#[from] EvalBlockError),
    /// More distinct blocks were added than currently supported.
    #[error("more than {} block types is not yet supported", BlockIndex::MAX as usize+1)]
    TooManyBlocks(),
}
/// Description of a change to a [`Space`] for use in listeners.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum SpaceChange {
    /// The block at the given location was replaced.
    Block(GridPoint),
    /// The light level value at the given location changed.
    Lighting(GridPoint),
    /// The given block index number was reassigned and now refers to a different
    /// [`Block`] value.
    Number(BlockIndex),
    /// The definition of the block referred to by the given block index number was
    /// changed; the result of [`Space::get_evaluated`] may differ.
    BlockValue(BlockIndex),
    /// Equivalent to [`SpaceChange::Block`] for every cube and [`SpaceChange::Number`]
    /// for every index.
    EveryBlock,
}
/// Performance data returned by [`Space::step`]. The exact contents of this structure
/// are unstable; use only `Debug` formatting to examine its contents unless you have
/// a specific need for one of the values.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub(crate) struct SpaceStepInfo {
    /// Number of spaces whose updates were aggregated into this value.
    pub(crate) spaces: usize,
    /// Time and count of block re-evaluations.
    ///
    /// Note that this does not count evaluations resulting from modifications
    /// that add new blocks to the space.
    pub(crate) evaluations: TimeStats,
    /// Number of individual cubes processed (`tick_action`).
    cube_ticks: usize,
    /// Time spent on processing individual cube updates
    /// (measured as a whole because transaction conflict checking is needed),
    cube_time: Duration,
    /// Time spent on processing behaviors.
    behaviors_time: Duration,
    /// Performance data about light updates within the space.
    pub(crate) light: LightUpdatesInfo,
}
impl std::ops::AddAssign<SpaceStepInfo> for SpaceStepInfo {
    fn add_assign(&mut self, other: Self) {
        loop {}
    }
}
impl CustomFormat<StatusText> for SpaceStepInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: StatusText) -> fmt::Result {
        loop {}
    }
}
/// [`Space`]'s set of things that need recomputing based on notifications.
///
/// Currently this is responsible for counting block changes.
/// In the future it might be used for side effects in the world, or we might
/// want to handle that differently.
#[derive(Debug, Default)]
struct SpaceTodo {
    blocks: HashSet<BlockIndex>,
}
#[derive(Clone, Debug)]
struct SpaceBlockChangeListener {
    todo: Weak<Mutex<SpaceTodo>>,
    index: BlockIndex,
}
impl Listener<BlockChange> for SpaceBlockChangeListener {
    fn receive(&self, _: BlockChange) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
