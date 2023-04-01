use std::fmt::Debug;

use crate::mesh::BlockMesh;
use crate::space::BlockIndex;
/// A triangle mesh representation of a [`Space`] (or part of it) which may
/// then be rasterized.
///
/// A [`SpaceMesh`] may be used multiple times as a [`Space`] is modified.
/// Currently, the only benefit of this is avoiding reallocating memory.
///
/// The type parameters allow adaptation to the target graphics API:
/// * `V` is the type of vertices.
/// * `T` is the type of textures, which come from a [`TextureAllocator`].
///
/// [`TextureAllocator`]: super::TextureAllocator
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaceMesh<V, T> {
    vertices: Vec<V>,
    /// Texture tiles used by the vertices; holding these objects is intended to ensure
    /// the texture coordinates stay valid.
    textures_used: Vec<T>,
}
/// Source of [`BlockMesh`] values for [`SpaceMesh::compute`].
///
/// This trait allows the caller of [`SpaceMesh::compute`] to provide an
/// implementation which e.g. lazily computes meshes.
///
/// TODO: This currently only has one implementation and should be discarded if it is not
/// a useful abstraction.
pub(crate) trait BlockMeshProvider<'a, V, T> {
    /// Obtain a mesh for the given block index, or `None` if the index is out of range.
    fn get(&mut self, index: BlockIndex) -> Option<&'a BlockMesh<V, T>>;
}
