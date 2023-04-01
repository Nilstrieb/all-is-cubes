//! Mesh vertices.
use cgmath::Point3;
use crate::math::{Face6, FreeCoordinate, GridPoint, Rgba};
/// Numeric type used to store texture coordinates in vertices.
///
/// TODO: Delete this type alias now that we're generic over texture coordinates.
pub(crate) type TextureCoordinate = f32;
/// Basic vertex data type for a [`BlockMesh`].
/// Implement <code>[`From`]&lt;[`BlockVertex`]&gt;</code> (and usually [`GfxVertex`])
/// to provide a specialized version fit for the target graphics API.
///
/// `T` is the type of texture-coordinate points being used. That is, one `T` value
/// should identify one point in the block's 3D texture, such as `T = Point3<f32>`).
///
/// [`BlockMesh`]: super::BlockMesh
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, PartialEq)]
pub struct BlockVertex<T> {
    /// Surface color or texture coordinate.
    pub(crate) coloring: Coloring<T>,
}
/// Describes the two ways a [`BlockVertex`] may be colored; by a solid color or by a texture.
///
/// `T` is the type of texture-coordinate points being used. That is, one `T` value
/// should identify one point in the block's 3D texture, such as `T = Point3<f32>`).
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Coloring<T> {
    /// Solid color.
    Solid(),
    /// Texture coordinates provided by the [`TextureAllocator`](super::TextureAllocator)
    /// for this vertex.
    Texture {
        /// Upper bounds for clamping the entire surface's texture coordinates.
        /// Used to avoid texture bleed.
        clamp_max: T,
    },
}
/// A custom representation of [`BlockVertex`] suitable for a specific graphics system.
///
/// The life cycle of a [`GfxVertex`]:
///
/// * First, it is constructed by [`BlockMesh::new()`]
///   for a particular [`Block`] value, and stored in a [`BlockMesh`].
/// * Then, wherever that block appears in a [`Space`], the block vertices are copied
///   to become the [`SpaceMesh`]’s vertices, and [`GfxVertex::instantiate_vertex`] is
///   called on each copy to position it at the particular block's location.
///
/// [`Block`]: crate::block::Block
/// [`BlockMesh`]: crate::mesh::BlockMesh
/// [`BlockMesh::new()`]: crate::mesh::BlockMesh::new()
/// [`Space`]: crate::space::Space
/// [`SpaceMesh`]: crate::mesh::SpaceMesh
pub trait GfxVertex: From<BlockVertex<Self::TexPoint>> + Copy + Sized {
    /// Whether [`SpaceMesh`]es should provide pre-sorted vertex index slices to allow
    /// back-to-front drawing order based on viewing ranges.
    ///
    /// Design note: Strictly speaking, this doesn't need to be static and could be part
    /// of [`MeshOptions`]. However, we currently have no reason to complicate run-time
    /// data flow that way.
    ///
    /// [`SpaceMesh`]: crate::mesh::SpaceMesh
    /// [`MeshOptions`]: crate::mesh::MeshOptions
    const WANTS_DEPTH_SORTING: bool;
    /// Number type for the vertex position coordinates.
    type Coordinate: cgmath::BaseFloat;
    /// Point type identifying a point in the block's texture.
    type TexPoint: Copy;
    /// Type of the data carried from [`Self::instantiate_block`] to
    /// [`Self::instantiate_vertex`].
    type BlockInst: Copy;
    /// Prepare the information needed to instantiate vertices of one block.
    /// Currently, this constitutes the location of that block, and hence this function
    /// is responsible for any necessary numeric conversion.
    fn instantiate_block(cube: GridPoint) -> Self::BlockInst;
    /// Transforms a vertex belonging to a general model of a block to its instantiation
    /// in a specific location in space.
    fn instantiate_vertex(&mut self, block: Self::BlockInst);
    /// Returns the position of this vertex.
    ///
    /// Note: This is used to perform depth sorting for transparent vertices.
    fn position(&self) -> Point3<Self::Coordinate>;
}
