use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BlockFaceMesh<V> {
    /// Vertices, as used by the indices vectors.
    pub(super) vertices: Vec<V>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BlockMesh<V, T> {
    pub(super) interior_vertices: BlockFaceMesh<V>,

    pub(super) textures_used: Vec<T>,
}
