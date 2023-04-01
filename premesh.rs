use std::num::NonZeroU32;
use std::sync::{Arc, Mutex, Weak};

use fnv::{FnvHashMap, FnvHashSet};

use crate::chunking::ChunkPos;
use crate::listen::Listener;
use crate::math::GridCoordinate;
use crate::mesh::{GfxVertex, SpaceMesh, TextureAllocator};
use crate::space::{BlockIndex, SpaceChange};

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
pub struct ChunkedSpaceMesh<D, const CHUNK_SIZE: GridCoordinate> {
    /// Dirty flags listening to `space`.
    todo: Arc<Mutex<CsmTodo<CHUNK_SIZE>>>,

    block_meshes: ((), ()),

    /// Invariant: the set of present chunks (keys here) is the same as the set of keys
    /// in `todo.read().unwrap().chunks`.
    chunks: FnvHashMap<ChunkPos<CHUNK_SIZE>, ChunkMesh<D, (), (), CHUNK_SIZE>>,
}

#[derive(Debug)]
struct VersionedBlockMesh<Vert, Tile> {
    mesh: (Vert, Tile),
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
pub struct ChunkMesh<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate> {
    position: ChunkPos<CHUNK_SIZE>,
    mesh: SpaceMesh<Vert, Tex>,
    pub render_data: D,
    block_dependencies: Vec<(BlockIndex, BlockMeshVersion)>,

    /// Toggled whenever the mesh is updated. Value is arbitrary (this is a looping
    /// 2-state counter).
    update_debug: bool,
}

/// [`ChunkedSpaceMesh`]'s set of things that need recomputing.
#[derive(Debug, Default)]
struct CsmTodo<const CHUNK_SIZE: GridCoordinate> {
    // TODO: Benchmark using a BitVec instead.
    blocks: FnvHashSet<BlockIndex>,
    /// Membership in this table indicates that the chunk *exists;* todos for chunks
    /// outside of the view area are not tracked.
    chunks: FnvHashMap<ChunkPos<CHUNK_SIZE>, ChunkTodo>,
}

/// [`Listener`] adapter for [`CsmTodo`].
#[derive(Clone, Debug)]
struct TodoListener<const CHUNK_SIZE: GridCoordinate>(Weak<Mutex<CsmTodo<CHUNK_SIZE>>>);

impl<const CHUNK_SIZE: GridCoordinate> Listener<SpaceChange> for TodoListener<CHUNK_SIZE> {}

/////// MOVE START

impl<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate> ChunkMesh<D, Vert, Tex, CHUNK_SIZE>
where
    D: Default, // TODO: This is used for initializing `render_data`, but it might not be ideal.
    Vert: GfxVertex,
    Tex: TextureAllocator,
{
}

/// What might be dirty about a single chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct ChunkTodo {}

/////// MOVE END
