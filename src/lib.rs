use std::fmt::Debug;

#[derive(Debug)]
pub struct ChunkMesh<const CHUNK_SIZE: usize> {
    position: [(); CHUNK_SIZE],
}

/////// MOVE START
impl<const CHUNK_SIZE: usize> ChunkMesh<CHUNK_SIZE> {}
struct ChunkTodo {}

#[automatically_derived]
impl ::core::fmt::Debug for ChunkTodo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "ChunkTodo")
    }
}
/////// MOVE END
