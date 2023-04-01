//! Algorithm for converting individual blocks to triangle meshes.
//!
//! This module is internal and reexported by its parent.
use std::fmt::Debug;


/// Part of the triangle mesh calculated for a [`Block`], stored in a [`BlockMesh`] keyed
/// by [`Face7`].
///
/// All triangles which are on the surface of the unit cube (such that they may be omitted
/// when a [`opaque`](EvaluatedBlock::opaque) block is adjacent) are grouped
/// under the corresponding face, and all other triangles are grouped under
/// [`Face7::Within`]. In future versions, the triangulator might be improved so that blocks
/// with concavities on their faces have the surface of each concavity included in that
/// face mesh rather than in [`Face7::Within`].
///
/// The texture associated with the contained vertices' texture coordinates is recorded
/// in the [`BlockMesh`] only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BlockFaceMesh<V> {
    /// Vertices, as used by the indices vectors.
    pub(super) vertices: Vec<V>,
}
/// A triangle mesh for a single [`Block`].
///
/// Get it from [`BlockMesh::new()`] or [`block_meshes_for_space`].
/// Pass it to [`SpaceMesh::new()`](super::SpaceMesh::new) to assemble
/// blocks into an entire scene or chunk.
///
/// The type parameters allow adaptation to the target graphics API:
/// * `V` is the type of vertices.
/// * `T` is the type of textures, which come from a [`TextureAllocator`].
///
/// TODO: Add methods so this can be read out directly if you really want to.
///
/// [`Block`]: crate::block::Block
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BlockMesh<V, T> {
    /// Vertices not fitting into [`Self::face_vertices`] because they may be visible
    /// from multiple directions or when the eye position is inside the block.
    pub(super) interior_vertices: BlockFaceMesh<V>,
    /// Texture tiles used by the vertices; holding these objects is intended to ensure
    /// the texture coordinates stay valid.
    ///
    /// TODO: Each block mesh used to require more than one tile, but they no longer
    /// do. Convert this to an Option, unless we decide that e.g. we want the triangulator
    /// to be responsible for optimizing opaque blocks into 6 face textures.
    pub(super) textures_used: Vec<T>,
}
