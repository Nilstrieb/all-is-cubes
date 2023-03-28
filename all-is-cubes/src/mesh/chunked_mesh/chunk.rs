use cgmath::{EuclideanSpace as _, Point3};
use instant::Instant;
use super::*;
use crate::chunking::ChunkPos;
use crate::math::GridCoordinate;
use crate::mesh::{GfxVertex, MeshOptions, SpaceMesh, TextureAllocator};
use crate::space::Space;
use crate::util::{ConciseDebug, CustomFormat};
impl<D, Vert, Tex, const CHUNK_SIZE: GridCoordinate> ChunkMesh<D, Vert, Tex, CHUNK_SIZE>
where
    D: Default,
    Vert: GfxVertex,
    Tex: TextureAllocator,
{
    pub(crate) fn new(position: ChunkPos<CHUNK_SIZE>) -> Self {
        loop {}
    }
    #[inline]
    pub fn mesh(&self) -> &SpaceMesh<Vert, Tex::Tile> {
        loop {}
    }
    #[inline]
    pub fn position(&self) -> ChunkPos<CHUNK_SIZE> {
        loop {}
    }
    pub(crate) fn borrow_for_update(
        &mut self,
        indices_only: bool,
    ) -> ChunkMeshUpdate<'_, D, Vert, Tex::Tile, CHUNK_SIZE> {
        loop {}
    }
    pub(super) fn recompute_mesh(
        &mut self,
        chunk_todo: &mut ChunkTodo,
        space: &Space,
        options: &MeshOptions,
        block_meshes: &VersionedBlockMeshes<Vert, Tex::Tile>,
    ) {
        loop {}
    }
    /// Sort the existing indices of `self.transparent_range(DepthOrdering::Within)` for
    /// the given view position in world coordinates.
    ///
    /// This is intended to be cheap enough to do every frame.
    ///
    /// Returns whether anything was done, i.e. whether the new indices should be copied
    /// to the GPU.
    pub fn depth_sort_for_view(
        &mut self,
        view_position: Point3<Vert::Coordinate>,
    ) -> bool {
        loop {}
    }
    pub(crate) fn stale_blocks(
        &self,
        block_meshes: &VersionedBlockMeshes<Vert, Tex::Tile>,
    ) -> bool {
        loop {}
    }
}
/// What might be dirty about a single chunk.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct ChunkTodo {
    pub recompute_mesh: bool,
}
impl ChunkTodo {
    pub const CLEAN: Self = Self { recompute_mesh: false };
}
