//! That which contains many blocks.
use crate::behavior::BehaviorSet;
use crate::block::{Block, EvalBlockError, EvaluatedBlock};
#[cfg(doc)]
use crate::character::Character;
use crate::character::Spawn;
use cgmath::Vector3;
use instant::Duration;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::listen::{Gate, Listen, Listener, Notifier};
use crate::math::{FreeCoordinate, GridAab, GridPoint, GridRotation, NotNan, Rgb};
use crate::transaction::{Merge, Transaction as _};
use crate::universe::{RefVisitor, VisitRefs};
use crate::util::TimeStats;
use crate::util::{CustomFormat, StatusText};
mod builder;
pub(crate) use builder::SpaceBuilderBounds;
mod light;
use light::LightUpdateQueue;
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
/// Description of where in a [`Space`] a [`Behavior<Space>`](crate::behavior::Behavior)
/// exists.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpaceBehaviorAttachment {
    bounds: GridAab,
    rotation: GridRotation,
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
