use std::collections::{HashMap, HashSet};
use std::fmt;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex, Weak};

use cgmath::Point3;
use fnv::{FnvHashMap, FnvHashSet};
use indoc::indoc;
use instant::{Duration, Instant};

use crate::block::{EvaluatedBlock, Resolution};
use crate::camera::{Camera, Flaws};
use crate::chunking::{cube_to_chunk, ChunkChart, ChunkPos};
use crate::listen::Listener;
use crate::math::{GridCoordinate, GridPoint};
use crate::mesh::{
    BlockMesh, BlockMeshProvider, GfxVertex, LineVertex, MeshOptions, SpaceMesh, TextureAllocator,
    TextureTile,
};
use crate::space::{BlockIndex, Space, SpaceChange};
use crate::universe::URef;
use crate::util::{CustomFormat, StatusText, TimeStats};

/// If true, enables reporting chunk update timing at [`log::trace`] level.
const LOG_CHUNK_UPDATES: bool = false;

/// The large-scale analogue of [`SpaceMesh`]: subdivides a [`Space`] into
/// [chunks](crate::chunking) which are individually recomputed as the space changes or
/// its contained blocks do.
///
/// Each chunk, a [`ChunkMesh`], owns a data value of type `D`, which is
/// initialized using `D::default()`. This value may be a reference to a corresponding
/// GPU buffer, for example. It will usually need to be an [`Option`] of something.
#[derive(Debug)]
pub struct ChunkedSpaceMesh<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate>
where
    Tex: TextureAllocator,
{
    space: URef<Space>,

    /// Dirty flags listening to `space`.
    todo: Arc<Mutex<CsmTodo<CHUNK_SIZE>>>,

    block_meshes: VersionedBlockMeshes<Vert, Tex::Tile>,

    /// Invariant: the set of present chunks (keys here) is the same as the set of keys
    /// in `todo.read().unwrap().chunks`.
    chunks: FnvHashMap<ChunkPos<CHUNK_SIZE>, ChunkMesh<D, Vert, Tex, CHUNK_SIZE>>,

    /// Resized as needed upon each [`Self::update_blocks_and_some_chunks()`].
    chunk_chart: ChunkChart<CHUNK_SIZE>,

    /// The chunk in which the last [`Camera`] provided is located.
    view_chunk: ChunkPos<CHUNK_SIZE>,

    /// Whether, on the previous frame, we did not finish updating all visible chunks.
    ///
    /// If so, then we prioritize adding new chunks over updating existing ones,
    /// because blank world is a worse outcome than slightly stale world.
    did_not_finish_chunks: bool,

    /// The [`MeshOptions`] specified by the last [`Camera`] provided.
    last_mesh_options: Option<MeshOptions>,

    /// Most recent time at which we reset to no data.
    zero_time: Instant,
    /// Earliest time prior to `zero_time` at which we finished everything in the queues.
    complete_time: Option<Instant>,
}

impl<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate> ChunkedSpaceMesh<D, Vert, Tex, CHUNK_SIZE>
where
    D: Default,
    Vert: GfxVertex<TexPoint = <<Tex as TextureAllocator>::Tile as TextureTile>::Point> + PartialEq,
    Tex: TextureAllocator,
    Tex::Tile: PartialEq,
{
    pub fn new(space: URef<Space>) -> Self {
        loop {}
    }

    /// Returns a reference to the [`Space`] this watches.
    pub fn space(&self) -> &URef<Space> {
        loop {}
    }

    /// Returns a [`ChunkChart`] for the view distance used by the most recent
    /// [`Self::update_blocks_and_some_chunks`].
    pub fn chunk_chart(&self) -> &ChunkChart<CHUNK_SIZE> {
        loop {}
    }

    /// Iterate over the [`ChunkMesh`]es of all chunks, in arbitrary order.
    ///
    /// Empty chunks are included; in particular, this will iterate over every `D` value
    /// owned by this [`ChunkedSpaceMesh`].
    ///
    /// TODO: Define the order
    pub fn iter_chunks(&self) -> impl Iterator<Item = &ChunkMesh<D, Vert, Tex, CHUNK_SIZE>> {
        [].into_iter()
    }

    /// Iterate over the [`ChunkMesh`]es that are in view from the given camera,
    /// in front-to-back order. (Use `.rev()` to iterate in back-to-front order.)
    ///
    /// Uses `camera`'s position, rotation, and options to decide which chunks to return.
    pub fn iter_in_view<'a>(
        &'a self,
        camera: &'a Camera,
    ) -> impl Iterator<Item = &'a ChunkMesh<D, Vert, Tex, CHUNK_SIZE>> + DoubleEndedIterator + 'a
    {
        [].into_iter()
    }

    /// Retrieves a [`ChunkMesh`] for the specified chunk position, if one exists.
    ///
    /// Call this while drawing, after [`Self::update_blocks_and_some_chunks`]
    /// has updated/created chunks.
    pub fn chunk(
        &self,
        position: ChunkPos<CHUNK_SIZE>,
    ) -> Option<&ChunkMesh<D, Vert, Tex, CHUNK_SIZE>> {
        todo!()
    }

    /// Recompute meshes of all blocks that need it, and the nearest chunks that need it.
    ///
    /// * `camera`'s view position is used to choose what to update and for depth
    ///    ordering; its graphics options are used for triangulation and view distance.
    /// * `deadline` is the approximate time at which this should stop.
    /// * `chunk_render_updater` is called for every re-meshed or depth-sorted chunk.
    ///
    /// Returns performance information and the chunk the camera is located in.
    ///
    /// TODO: The updaters should be changed to be one value instead of two, so that
    /// they can share mutable state if needed. This will benefit the wgpu renderer.
    pub fn update_blocks_and_some_chunks<F>(
        &mut self,
        camera: &Camera,
        block_texture_allocator: &Tex,
        deadline: Instant,
        mut chunk_render_updater: F,
    ) -> CsmUpdateInfo
    where
        F: FnMut(ChunkMeshUpdate<'_, D, Vert, Tex::Tile, CHUNK_SIZE>),
    {
        todo!()
    }

    /// Returns the chunk in which the camera from the most recent
    /// [`Self::update_blocks_and_some_chunks`] was located.
    /// This may be used as the origin point to iterate over chunks in view.
    pub fn view_chunk(&self) -> ChunkPos<CHUNK_SIZE> {
        self.view_chunk
    }

    /// Produces lines that visualize the boundaries of visible nonempty chunks.
    #[doc(hidden)] // TODO: good public API?
    pub fn chunk_debug_lines(&self, camera: &Camera, output: &mut impl Extend<LineVertex>) {
        todo!()
    }
}

/// Performance info from a [`ChunkedSpaceMesh`]'s per-frame update.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CsmUpdateInfo {
    /// Flaws detected during update.
    /// Note that this does not include mesh flaws; the caller must gather those when
    /// drawing the chunks.
    pub flaws: Flaws,
    pub total_time: Duration,
    /// Time spent on gathering information before starting the chunk scan.
    pub prep_time: Duration,
    /// Time spent on traversing chunks in view this frame,
    /// excluding the actual chunk mesh generation operations.
    pub chunk_scan_time: Duration,
    /// Time spent on building chunk meshes this frame.
    pub chunk_mesh_generation_times: TimeStats,
    /// Time spent on `chunk_mesh_updater` callbacks this frame.
    pub chunk_mesh_callback_times: TimeStats,
    depth_sort_time: Option<Duration>,
    /// Time spent on building block meshes this frame.
    pub block_updates: TimeStats,

    /// Number of chunks that currently exist.
    pub chunk_count: usize,
    /// Total in-memory size of chunk data (not counting [`ChunkMesh::render_data`]).
    pub chunk_total_cpu_byte_size: usize,
}

impl CustomFormat<StatusText> for CsmUpdateInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, _: StatusText) -> fmt::Result {
        loop {}
    }
}

#[derive(Debug)]
struct VersionedBlockMeshes<Vert, Tile> {
    /// Indices of this vector are block IDs in the Space.
    meshes: Vec<VersionedBlockMesh<Vert, Tile>>,

    last_version_counter: NonZeroU32,
}

impl<Vert, Tile> VersionedBlockMeshes<Vert, Tile>
where
    Vert: GfxVertex<TexPoint = <Tile as TextureTile>::Point> + PartialEq,
    Tile: TextureTile + PartialEq,
{
    fn new() -> Self {
        loop {}
    }

    /// Discard all meshes.
    /// Use this to ensure that in case of “everything changes” we don't store
    /// extra data.
    fn clear(&mut self) {
        loop {}
    }

    /// Update block meshes based on the given [`Space`].
    ///
    /// After this method returns, `self.meshes.len()` will
    /// always equal `space.block_data().len()`. It may not be fully updated yet, but
    /// it will be the correct length.
    ///
    /// TODO: Missing handling for `mesh_options` changing.
    fn update<A>(
        &mut self,
        todo: &mut FnvHashSet<BlockIndex>,
        space: &Space,
        block_texture_allocator: &A,
        mesh_options: &MeshOptions,
        deadline: Instant,
    ) -> TimeStats
    where
        A: TextureAllocator<Tile = Tile>,
    {
        loop {}
    }
}

impl<'a, Vert, Tile> BlockMeshProvider<'a, Vert, Tile> for &'a VersionedBlockMeshes<Vert, Tile> {
    fn get(&mut self, index: BlockIndex) -> Option<&'a BlockMesh<Vert, Tile>> {
        loop {}
    }
}

/// Entry in [`VersionedBlockMeshes`].
#[derive(Debug)]
struct VersionedBlockMesh<Vert, Tile> {
    mesh: BlockMesh<Vert, Tile>,
    /// Version ID used to track whether chunks have stale block meshes (ones that don't
    /// match the current definition of that block-index in the space).
    version: BlockMeshVersion,
}

/// Together with a [`BlockIndex`], uniquely identifies a block mesh.
/// Used to determine when chunk meshes need updating.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BlockMeshVersion {
    /// The block mesh hasn't been computed yet and this is the placeholder mesh.
    /// Special because it's never assigned as a "good" version number.
    NotReady,
    /// A specific version.
    /// u32 is sufficient size because we are extremely unlikely to wrap around u32 space
    /// in the course of a single batch of updates unless we're perpetually behind.
    Numbered(NonZeroU32),
}

/// Stores a [`SpaceMesh`] covering one chunk of a [`Space`], caller-provided rendering
/// data, and incidentals.
#[derive(Debug, Eq, PartialEq)]
pub struct ChunkMesh<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate>
where
    Tex: TextureAllocator,
{
    position: ChunkPos<CHUNK_SIZE>,
    mesh: SpaceMesh<Vert, Tex::Tile>,
    pub render_data: D,
    block_dependencies: Vec<(BlockIndex, BlockMeshVersion)>,

    /// Toggled whenever the mesh is updated. Value is arbitrary (this is a looping
    /// 2-state counter).
    update_debug: bool,
}

/// Provides mutable access to the render data of type `D` in a [`ChunkMesh`].
///
/// This struct is provided to the callbacks of
/// [`ChunkedSpaceMesh::update_blocks_and_some_chunks()`].
#[derive(Debug)]
#[non_exhaustive]
pub struct ChunkMeshUpdate<'a, D, V, T, const CHUNK_SIZE: GridCoordinate> {
    pub position: ChunkPos<CHUNK_SIZE>,
    pub mesh: &'a SpaceMesh<V, T>,
    pub render_data: &'a mut D,
    /// Whether *only* the indices need to be copied (and their length has not changed).
    pub indices_only: bool,
}

/// [`ChunkedSpaceMesh`]'s set of things that need recomputing.
#[derive(Debug, Default)]
struct CsmTodo<const CHUNK_SIZE: GridCoordinate> {
    all_blocks_and_chunks: bool,
    // TODO: Benchmark using a BitVec instead.
    blocks: FnvHashSet<BlockIndex>,
    /// Membership in this table indicates that the chunk *exists;* todos for chunks
    /// outside of the view area are not tracked.
    chunks: FnvHashMap<ChunkPos<CHUNK_SIZE>, ChunkTodo>,
}

impl<const CHUNK_SIZE: GridCoordinate> CsmTodo<CHUNK_SIZE> {
    fn initially_dirty() -> Self {
        loop {}
    }

    fn modify_block_and_adjacent<F>(&mut self, cube: GridPoint, mut f: F)
    where
        F: FnMut(&mut ChunkTodo),
    {
        loop {}
    }
}

/// [`Listener`] adapter for [`CsmTodo`].
#[derive(Clone, Debug)]
struct TodoListener<const CHUNK_SIZE: GridCoordinate>(Weak<Mutex<CsmTodo<CHUNK_SIZE>>>);

impl<const CHUNK_SIZE: GridCoordinate> Listener<SpaceChange> for TodoListener<CHUNK_SIZE> {
    fn receive(&self, message: SpaceChange) {
        loop {}
    }

    fn alive(&self) -> bool {
        self.0.strong_count() > 0
    }
}

/////// MOVE START

impl<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate> ChunkMesh<D, Vert, Tex, CHUNK_SIZE>
where
    D: Default, // TODO: This is used for initializing `render_data`, but it might not be ideal.
    Vert: GfxVertex,
    Tex: TextureAllocator,
{
    fn new(position: ChunkPos<CHUNK_SIZE>) -> Self {
        todo!()
    }

    #[inline]
    pub fn mesh(&self) -> &SpaceMesh<Vert, Tex::Tile> {
        loop {}
    }

    #[inline]
    pub fn position(&self) -> ChunkPos<CHUNK_SIZE> {
        loop {}
    }

    fn borrow_for_update(
        &mut self,
        indices_only: bool,
    ) -> ChunkMeshUpdate<'_, D, Vert, Tex::Tile, CHUNK_SIZE> {
        todo!()
    }

    fn recompute_mesh(
        &mut self,
        chunk_todo: &mut ChunkTodo,
        space: &Space,
        options: &MeshOptions,
        block_meshes: &VersionedBlockMeshes<Vert, Tex::Tile>,
    ) {
        todo!()
    }

    /// Sort the existing indices of `self.transparent_range(DepthOrdering::Within)` for
    /// the given view position in world coordinates.
    ///
    /// This is intended to be cheap enough to do every frame.
    ///
    /// Returns whether anything was done, i.e. whether the new indices should be copied
    /// to the GPU.
    pub fn depth_sort_for_view(&mut self, view_position: Point3<Vert::Coordinate>) -> bool {
        todo!()
    }

    fn stale_blocks(&self, block_meshes: &VersionedBlockMeshes<Vert, Tex::Tile>) -> bool {
        todo!()
    }
}

/// What might be dirty about a single chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct ChunkTodo {
    recompute_mesh: bool,
}

impl ChunkTodo {
    const CLEAN: Self = Self {
        recompute_mesh: false,
    };
}

/////// MOVE END
