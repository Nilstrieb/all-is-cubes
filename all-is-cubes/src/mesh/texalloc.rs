//! Traits for texture atlas/array allocator for block textures.
use std::fmt;
use std::sync::atomic::AtomicUsize;
use cgmath::Point3;
use crate::block::Evoxels;
use crate::math::GridAab;
use crate::mesh::TextureCoordinate;
use crate::util::{ConciseDebug, CustomFormat};
/// Color data accepted by [`TextureAllocator`].
/// The components are sRGB `[R, G, B, A]`.
pub type Texel = [u8; 4];
/// Allocator of 3D regions ("tiles") in a texture atlas to paint block voxels into.
/// Implement this trait using the target graphics API's 3D texture type.
pub trait TextureAllocator {
    /// Tile handles produced by this allocator.
    type Tile: TextureTile<Point = Self::Point>;
    /// Type of points within the texture.
    type Point;
    /// Allocate a tile, whose range of texels will be reserved for use as long as the
    /// `Tile` value, and its clones, are not dropped.
    ///
    /// The given [`GridAab`] specifies the desired size of the allocation;
    /// its translation does not affect the size but may be used to make the resulting
    /// texture coordinate transformation convenient for the caller.
    ///
    /// Returns [`None`] if no space is available for another tile.
    fn allocate(&self, bounds: GridAab) -> Option<Self::Tile>;
}
/// 3D texture slice to paint a block's voxels in. When all clones of this value are
/// dropped, the texture allocation will be released and the texture coordinates may
/// be reused for different data.
pub trait TextureTile: Clone {
    /// Type of points within the texture.
    type Point: Copy;
    /// Returns the [`GridAab`] originally passed to the texture allocator for this tile.
    fn bounds(&self) -> GridAab;
    /// Transform a point in the coordinate system of, and within, [`Self::bounds()`]
    /// (that is, where 1 unit = 1 texel) into texture coordinates suitable for the
    /// target [`GfxVertex`](super::GfxVertex) type.
    fn grid_to_texcoord(&self, in_tile_grid: Point3<TextureCoordinate>) -> Self::Point;
    /// Write texture data as RGBA color.
    ///
    /// `data` must be of length `self.bounds().volume()`.
    fn write(&mut self, data: &[Texel]);
}
pub(super) fn copy_voxels_to_texture<A: TextureAllocator>(
    texture_allocator: &A,
    voxels: &Evoxels,
) -> Option<A::Tile> {
    loop {}
}
pub(super) fn copy_voxels_into_existing_texture<T: TextureTile>(
    voxels: &Evoxels,
    texture: &mut T,
) {
    loop {}
}
