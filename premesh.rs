#[derive(Debug, Eq, PartialEq)]
pub struct ChunkMesh<const CHUNK_SIZE: usize> {
    position: [(); CHUNK_SIZE],
}

/////// MOVE START

impl<const CHUNK_SIZE: usize> ChunkMesh<CHUNK_SIZE> {}

/// What might be dirty about a single chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct ChunkTodo {}

/////// MOVE END
