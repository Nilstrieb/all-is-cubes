//! Algorithm for converting individual blocks to triangle meshes.
//!
//! This module is internal and reexported by its parent.
use std::fmt::Debug;
use crate::block::EvaluatedBlock;
use crate::camera::Flaws;
use crate::math::{Face7, FaceMap, GridArray, OpacityCategory};
use crate::mesh::{BlockVertex, MeshOptions, TextureAllocator, TextureTile};
use crate::space::Space;
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
    /// Indices into `self.vertices` that form triangles (i.e. length is a multiple of 3)
    /// in counterclockwise order, for vertices whose coloring is fully opaque (or
    /// textured with binary opacity).
    pub(super) indices_opaque: Vec<u32>,
    /// Indices for partially transparent (alpha neither 0 nor 1) vertices.
    pub(super) indices_transparent: Vec<u32>,
    /// Whether the graphic entirely fills its cube face, such that nothing can be seen
    /// through it and faces of adjacent blocks may be removed.
    pub(super) fully_opaque: bool,
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
    /// Vertices grouped by which face being obscured would obscure those vertices.
    pub(super) face_vertices: FaceMap<BlockFaceMesh<V>>,
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
    /// The [`EvaluatedBlock::voxel_opacity_mask`] that the mesh was constructed from;
    /// if new block data has the same mask, then it is safe to replace the texture
    /// without recalculating the mesh, via [`BlockMesh::try_update_texture_only`].
    ///
    /// If this is [`None`], then either there is no texture to update or some of the
    /// colors have been embedded in the mesh vertices, making a mesh update required.
    /// (TODO: We could be more precise about which voxels are so frozen -- revisit
    /// whether that's worthwhile.)
    pub(super) voxel_opacity_mask: Option<GridArray<OpacityCategory>>,
    /// Flaws in this mesh, that should be reported as flaws in any rendering containing it.
    flaws: Flaws,
}
/// Computes [`BlockMeshes`] for blocks currently present in a [`Space`].
/// Pass the result to [`SpaceMesh::new()`](super::SpaceMesh::new) to use it.
///
/// The resulting array is indexed by the `Space`'s
/// [`BlockIndex`](crate::space::BlockIndex) values.
pub(crate) fn block_meshes_for_space<V, A>(
    space: &Space,
    texture_allocator: &A,
    options: &MeshOptions,
) -> BlockMeshes<V, A::Tile>
where
    V: From<BlockVertex<<<A as TextureAllocator>::Tile as TextureTile>::Point>>,
    A: TextureAllocator,
{
    loop {}
}
/// Array of [`BlockMesh`] indexed by a [`Space`]'s block indices; a convenience
/// alias for the return type of [`block_meshes_for_space`].
/// Pass it to [`SpaceMesh::new()`](super::SpaceMesh::new) to use it.
pub(crate) type BlockMeshes<V, A> = Box<[BlockMesh<V, A>]>;
