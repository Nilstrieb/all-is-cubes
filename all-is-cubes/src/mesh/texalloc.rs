//! Traits for texture atlas/array allocator for block textures.
use std::fmt;
use std::sync::atomic::{AtomicUsize};
use cgmath::Point3;
use crate::block::{Evoxels};

use crate::math::{GridAab};
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
impl<T: TextureAllocator> TextureAllocator for &T {
    type Tile = T::Tile;
    type Point = T::Point;
    #[mutants::skip]
    fn allocate(&self, bounds: GridAab) -> Option<Self::Tile> {
        loop {}
    }
}
impl<T: TextureAllocator> TextureAllocator for std::sync::Arc<T> {
    type Tile = T::Tile;
    type Point = T::Point;
    #[mutants::skip]
    fn allocate(&self, bounds: GridAab) -> Option<Self::Tile> {
        loop {}
    }
}
impl<T: TextureAllocator> TextureAllocator for std::rc::Rc<T> {
    type Tile = T::Tile;
    type Point = T::Point;
    #[mutants::skip]
    fn allocate(&self, bounds: GridAab) -> Option<Self::Tile> {
        loop {}
    }
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
/// Null [`TextureAllocator`]; rejects all allocations.
///
/// Used for generating textureless meshes. TODO: Modify triangulator to actually
/// generate separate triangles when textures are unavailable.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct NoTextures;
impl TextureAllocator for NoTextures {
    type Tile = NoTexture;
    type Point = NoTexture;
    fn allocate(&self, _: GridAab) -> Option<Self::Tile> {
        loop {}
    }
}
/// Uninhabited [`TextureTile`] type; no instance of this ever exists.
///
/// TODO: this can and should be just ! (never) when that's available in stable Rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum NoTexture {}
impl TextureTile for NoTexture {
    type Point = Self;
    fn bounds(&self) -> GridAab {
        loop {}
    }
    fn grid_to_texcoord(&self, _in_tile: Point3<TextureCoordinate>) -> NoTexture {
        loop {}
    }
    fn write(&mut self, _data: &[Texel]) {
        loop {}
    }
}
impl CustomFormat<ConciseDebug> for NoTexture {
    fn fmt(&self, _: &mut fmt::Formatter<'_>, _: ConciseDebug) -> fmt::Result {
        loop {}
    }
}
/// [`TextureAllocator`] which discards all input except for counting calls; for testing.
///
/// This type is public so that it may be used in benchmarks and such, but not intended to be used
/// outside of All is Cubes itself.
#[doc(hidden)]
#[derive(Debug)]
pub struct TestTextureAllocator {
    capacity: usize,
    count_allocated: AtomicUsize,
}
impl TestTextureAllocator {
    pub const fn new() -> Self {
        loop {}
    }
    /// Fail after allocating this many tiles. (Currently does not track deallocations.)
    pub fn set_capacity(&mut self, capacity: usize) {
        loop {}
    }
    /// Number of tiles allocated. Does not decrement for deallocations.
    pub fn count_allocated(&self) -> usize {
        loop {}
    }
}
impl Default for TestTextureAllocator {
    fn default() -> Self {
        loop {}
    }
}
impl TextureAllocator for TestTextureAllocator {
    type Tile = TestTextureTile;
    type Point = TtPoint;
    fn allocate(&self, bounds: GridAab) -> Option<Self::Tile> {
        loop {}
    }
}
/// Tile type for [`TestTextureAllocator`].
///
/// This type is public so that it may be used in benchmarks and such.
#[derive(Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestTextureTile {
    bounds: GridAab,
}
impl TextureTile for TestTextureTile {
    type Point = TtPoint;
    fn bounds(&self) -> GridAab {
        loop {}
    }
    fn grid_to_texcoord(&self, in_tile: Point3<TextureCoordinate>) -> Self::Point {
        loop {}
    }
    fn write(&mut self, data: &[Texel]) {
        loop {}
    }
}
/// Texture point for [`TestTextureAllocator`]
#[doc(hidden)]
pub type TtPoint = Point3<TextureCoordinate>;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Resolution::*;
    /// Test the [`TestTextureAllocator`].
    #[test]
    fn test_texture_allocator() {
        loop {}
    }
}
