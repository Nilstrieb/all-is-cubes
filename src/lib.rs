use std::fmt::Debug;

#[derive(Debug)]
pub struct ChunkMesh<const CHUNK_SIZE: usize> {
    position: [(); CHUNK_SIZE],
}

/////// MOVE START
impl<const CHUNK_SIZE: usize> ChunkMesh<CHUNK_SIZE> {}
struct ChunkTodo {}

impl core::fmt::Debug for ChunkTodo {
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        loop {}
    }
}
/////// MOVE END
