//! Tests for [`crate::mesh`].
use cgmath::{MetricSpace as _, Point3, Transform as _, Vector3};
use pretty_assertions::assert_eq;
use super::*;
use crate::block::{Block, BlockAttributes, Primitive, Resolution::*, AIR};
use crate::camera::{Flaws, GraphicsOptions, TransparencyOption};
use crate::content::{make_some_blocks, make_some_voxel_blocks};
use crate::math::{
    Face6::{self, *},
    FaceMap, FreeCoordinate, GridAab, GridPoint, GridRotation, Rgba,
};
use crate::mesh::BlockMesh;
use crate::space::{Space, SpacePhysics};
use crate::universe::Universe;
/// Shorthand for writing out an entire [`BlockVertex`] with solid color.
fn v_c<T>(
    position: [FreeCoordinate; 3],
    face: Face6,
    color: [f32; 4],
) -> BlockVertex<T> {
    loop {}
}
/// Shorthand for writing out an entire [`BlockVertex`] with texturing.
fn v_t(
    position: [FreeCoordinate; 3],
    face: Face6,
    texture: [TextureCoordinate; 3],
) -> BlockVertex<TtPoint> {
    loop {}
}
/// Test helper to create [`BlockMesh`] alone without a `Space`.
fn test_block_mesh(block: Block) -> BlockMesh<BlockVertex<TtPoint>, TestTextureTile> {
    loop {}
}
/// Test helper to create [`BlockMesh`] alone without a `Space`,
/// and with the transparency option set to `Threshold(0.5)`.
fn test_block_mesh_threshold(
    block: Block,
) -> BlockMesh<BlockVertex<TtPoint>, TestTextureTile> {
    loop {}
}
/// Test helper to call [`block_meshes_for_space`] followed directly by [`SpaceMesh::new`].
#[allow(clippy::type_complexity)]
pub(crate) fn mesh_blocks_and_space(
    space: &Space,
) -> (
    TestTextureAllocator,
    BlockMeshes<BlockVertex<TtPoint>, TestTextureTile>,
    SpaceMesh<BlockVertex<TtPoint>, TestTextureTile>,
) {
    loop {}
}
fn non_uniform_fill(cube: GridPoint) -> &'static Block {
    loop {}
}
#[test]
fn excludes_hidden_faces_of_blocks() {
    loop {}
}
/// Run [`SpaceMesh::new`] with stale block data and confirm it does not panic.
#[test]
fn no_panic_on_missing_blocks() {
    loop {}
}
/// Construct a 1x1 recursive block and test that this is equivalent in geometry
/// to an atom block.
#[test]
fn trivial_voxels_equals_atom() {
    loop {}
}
/// [`SpaceMesh`] of a 1×1×1 space has the same geometry as the contents.
///
/// This test compares 3 different values:
///
/// * A [`BlockMesh`] (with texture).
/// * A [`SpaceMesh`] produced from it via normal construction.
/// * A [`SpaceMesh`] produced from it via [`SpaceMesh::from`].
#[test]
fn space_mesh_equals_block_mesh() {
    loop {}
}
/// TODO: This test stops being meaningful when we finish migrating the texture allocator to use arbitrary-sized tiles
#[test]
fn block_resolution_greater_than_tile() {
    loop {}
}
/// Check for hidden surfaces being given internal geometry.
/// Exercise the “shrinkwrap” logic that generates geometry no larger than necessary.
#[test]
#[rustfmt::skip]
fn shrunken_box_has_no_extras() {
    loop {}
}
/// Exercise the case where textures are skipped because the color is uniform.
/// TODO: There are more subcases such as still using textures for irregular
/// shapes.
#[test]
#[rustfmt::skip]
fn shrunken_box_uniform_color() {
    loop {}
}
fn opacities<V, T>(mesh: &BlockMesh<V, T>) -> FaceMap<bool> {
    loop {}
}
#[test]
fn atom_transparency() {
    loop {}
}
#[test]
fn atom_transparency_thresholded() {
    loop {}
}
/// Test [`BlockMesh::fully_opaque`] results from basic voxels.
#[test]
fn fully_opaque_voxels() {
    loop {}
}
/// Test [`BlockMesh::fully_opaque`] when the voxels are all individually opaque,
/// but don't fill the cube.
#[test]
fn fully_opaque_partial_block() {
    loop {}
}
#[test]
fn transparency_split() {
    loop {}
}
#[test]
fn handling_allocation_failure() {
    loop {}
}
#[test]
fn space_mesh_empty() {
    loop {}
}
#[test]
fn depth_ordering_from_view_direction() {
    loop {}
}
/// Test that `clamp_min < clamp_max`, and that `pos` is within the range (± 0.5) too.
///
/// (It'd be nice if this was instead a debug assertion when constructing vertices, but
/// the data is a fully `pub` struct and enum.)
#[test]
fn texture_clamp_coordinate_ordering() {
    loop {}
}
