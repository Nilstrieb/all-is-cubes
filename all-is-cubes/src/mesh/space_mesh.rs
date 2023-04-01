use bitvec::vec::BitVec;
use std::fmt::Debug;
use std::ops::Range;
use crate::camera::Flaws;
use crate::math::GridRotation;
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
/// Identifies a back-to-front order in which to draw triangles (of a [`SpaceMesh`]),
/// based on the direction from which they are being viewed.
#[allow(clippy::exhaustive_enums)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum DepthOrdering {
    /// Any ordering is acceptable.
    Any,
    /// The viewpoint is within the volume; therefore dynamic rather than precomputed
    /// sorting must be used.
    Within,
    /// The viewpoint is outside the volume in a particular direction.
    ///
    /// The [`GridRotation`] is a rotation which will rotate the vector pointing from
    /// the viewpoint to the triangles such that it lies in the crooked-pyramid-shaped
    /// 48th of space where <var>x</var> &ge; <var>y</var> &ge; <var>z</var> &ge; 0.
    ///
    /// For more information on this classification scheme and the sort that uses it,
    /// see [volumetric sort (2006)].
    ///
    /// [volumetric sort (2006)]: https://iquilezles.org/www/articles/volumesort/volumesort.htm
    Direction(),
}
impl DepthOrdering {
    const ROT_COUNT: usize = GridRotation::ALL.len();
    const COUNT: usize = Self::ROT_COUNT + 1;
    #[allow(dead_code)]
    fn from_index(index: usize) -> Self {
        loop {}
    }
    fn to_index(self) -> usize {
        loop {}
    }
    fn rev(self) -> Self {
        loop {}
    }
}
