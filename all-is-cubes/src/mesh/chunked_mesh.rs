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

#[derive(Debug)]
pub struct ChunkedSpaceMesh<D, const CHUNK_SIZE: GridCoordinate> {
    chunks: FnvHashMap<ChunkPos<CHUNK_SIZE>, ChunkMesh<D, (), (), CHUNK_SIZE>>,
}

#[derive(Debug)]
struct VersionedBlockMesh<Vert, Tile> {
    mesh: (Vert, Tile),
    version: BlockMeshVersion,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BlockMeshVersion {
    NotReady,
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
