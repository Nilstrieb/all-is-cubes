#[derive(Debug)]
pub struct ChunkMesh<const CHUNK_SIZE: usize> {
    position: [(); CHUNK_SIZE],
}

/////// MOVE START

impl<const CHUNK_SIZE: usize> ChunkMesh<CHUNK_SIZE> {}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct ChunkTodo {}

/////// MOVE END
