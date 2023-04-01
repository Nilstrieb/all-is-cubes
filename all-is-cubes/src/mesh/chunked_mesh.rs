use std::num::NonZeroU32;
use std::sync::{Mutex, Weak};

use fnv::{FnvHashMap, FnvHashSet};

use crate::chunking::ChunkPos;
use crate::listen::Listener;
use crate::math::GridCoordinate;
use crate::space::{BlockIndex, SpaceChange};

/// If true, enables reporting chunk update timing at [`log::trace`] level.
const LOG_CHUNK_UPDATES: bool = false;

/// Stores a [`SpaceMesh`] covering one chunk of a [`Space`], caller-provided rendering
/// data, and incidentals.
#[derive(Debug, Eq, PartialEq)]
pub struct ChunkMesh<const CHUNK_SIZE: GridCoordinate> {
    position: [(); CHUNK_SIZE],

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



/////// MOVE START

impl<const CHUNK_SIZE: GridCoordinate> ChunkMesh<CHUNK_SIZE> {}

/// What might be dirty about a single chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct ChunkTodo {}

/////// MOVE END
