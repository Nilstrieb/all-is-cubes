//! Built-in “game content”: basic shapes and colors used in the UI and tests.
//!
//! This module is private; the public interface to this stuff is the separate
//! `all-is-cubes-content` crate.
use crate::block::{Block, Resolution};
use crate::inv::Slot;
use crate::math::{GridCoordinate, Rgba};
use crate::space::{SetCubeError, Space};
use crate::universe::Universe;
/// Generate a set of distinct [`Primitive::Atom`] blocks for use in tests.
/// They will have distinct colors and names, and all other attributes default.
/// They will be fully opaque.
///
/// ```
/// # use all_is_cubes::block::{Block, Resolution};
/// # use all_is_cubes::content::make_some_blocks; // hide because wrong import path
/// #
/// let blocks: [Block; 3] = make_some_blocks();
/// assert_ne!(blocks[0], blocks[1]);
/// assert_ne!(blocks[0], blocks[2]);
/// assert_ne!(blocks[1], blocks[2]);
/// assert_eq!(blocks[0].evaluate().unwrap().resolution(), Resolution::R1);
/// ```
///
/// [`Primitive::Atom`]: crate::block::Primitive::Atom
pub(crate) fn make_some_blocks<const COUNT: usize>() -> [Block; COUNT] {
    loop {}
}
/// Generate a set of distinct [`Primitive::Recur`] blocks for use in tests.
/// They will have distinct appearances and names, and all other attributes default.
/// They will be fully opaque.
///
/// ```
/// # use all_is_cubes::block::{Block, Resolution};
/// # use all_is_cubes::content::make_some_voxel_blocks; // hide because wrong import path
/// # use all_is_cubes::universe::Universe;
/// #
/// let mut universe = Universe::new();
/// let blocks: [Block; 3] = make_some_voxel_blocks(&mut universe);
/// assert_ne!(blocks[0], blocks[1]);
/// assert_ne!(blocks[0], blocks[2]);
/// assert_ne!(blocks[1], blocks[2]);
/// assert_eq!(blocks[0].evaluate().unwrap().resolution(), Resolution::R16);
/// ```
///
/// [`Primitive::Recur`]: crate::block::Primitive::Recur
pub(crate) fn make_some_voxel_blocks<const COUNT: usize>(
    universe: &mut Universe,
) -> [Block; COUNT] {
    loop {}
}
fn color_sequence_for_make_blocks(n: usize) -> impl Iterator<Item = (usize, Rgba)> {
    (0..n)
        .map(move |i| {
            let luminance = if n > 1 { i as f32 / (n - 1) as f32 } else { 0.5 };
            (i, Rgba::new(luminance, luminance, luminance, 1.0))
        })
}
/// Generate a block which fills some fraction of its cube volume, from the bottom (−Y) up.
///
/// (This function exists because of a variety of tests of recursive blocks needing this
/// pattern.)
///
/// TODO: Allow caller-provided colors/pattern.
/// TODO: Consider writing the size on the faces.
#[doc(hidden)]
pub(crate) fn make_slab(
    universe: &mut Universe,
    numerator: GridCoordinate,
    denominator: Resolution,
) -> Block {
    loop {}
}
/// Draw the Space's axes as lines of blocks centered on (0, 0, 0).
///
/// ```
/// use all_is_cubes::block::AIR;
/// use all_is_cubes::math::GridAab;
/// use all_is_cubes::space::Space;
/// use all_is_cubes::content::axes;
///
/// let mut space = Space::empty(GridAab::from_lower_upper([-10, -10, -10], [11, 11, 11]));
/// axes(&mut space);
///
/// assert_ne!(space[[10, 0, 0]], AIR);
/// assert_ne!(space[[0, 10, 0]], AIR);
/// assert_ne!(space[[0, 0, 10]], AIR);
/// assert_ne!(space[[-10, 0, 0]], AIR);
/// assert_ne!(space[[0, -10, 0]], AIR);
/// assert_ne!(space[[0, 0, -10]], AIR);
/// ```
pub(crate) fn axes(space: &mut Space) -> Result<(), SetCubeError> {
    loop {}
}
/// A set of inventory items to give character free movement and modification of
/// everything in the universe. (For the moment, actually just the current space.)
///
/// TODO: ideally `flying` wouldn't be an explicit parameter but determined based on
/// the same inputs as choose the spawn position.
pub(crate) fn free_editing_starter_inventory(flying: bool) -> Vec<Slot> {
    loop {}
}
