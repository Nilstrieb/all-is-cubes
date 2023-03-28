//! Draw 2D graphics and text into [`Space`]s, using a general adapter for
//! [`embedded_graphics`]'s drawing algorithms.
//!
//! The [`VoxelBrush`] type can also be useful in direct 3D drawing.
//!
//! ## Coordinate system differences
//!
//! [`embedded_graphics`] uses coordinates which are different from ours in
//! two ways that should be kept in mind when trying to align 2D and 3D shapes:
//!
//! *   Text drawing presumes that +X is rightward and +Y is downward. Hence,
//!     text will be upside-down unless the chosen transformation inverts Y (or
//!     otherwise transforms to suit the orientation the text is being viewed from).
//! *   Coordinates are considered to refer to pixel centers rather than low corners,
//!     and rectangles have inclusive upper bounds (whereas our [`GridAab`]s have
//!     exclusive upper bounds).
use cgmath::{EuclideanSpace as _, Transform as _};
use embedded_graphics::geometry::{Dimensions, Point};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888, RgbColor};
use embedded_graphics::prelude::{DrawTarget, Drawable, Pixel};
use embedded_graphics::primitives::Rectangle;
use std::borrow::{Borrow, Cow};
use std::marker::PhantomData;
use std::ops::Range;
/// Re-export the version of the [`embedded_graphics`] crate we're using.
pub(crate) use embedded_graphics;
use crate::block::{Block, BlockAttributes, Resolution};
use crate::math::{GridAab, GridCoordinate, GridMatrix, GridPoint, GridVector, Rgb, Rgba};
use crate::space::{SetCubeError, Space, SpaceTransaction};
use crate::universe::Universe;
/// Convert a bounding-box rectangle, as from [`embedded_graphics::geometry::Dimensions`],
/// to a [`GridAab`] which encloses the voxels that would be affected by drawing a
/// [`Drawable`] with those bounds on a [`DrawingPlane`] with the given `transform`.
///
/// `max_brush` should be the union of bounds of [`VoxelBrush`]es used by the drawable.
/// If using plain colors, `GridAab::ORIGIN_CUBE` is the appropriate
/// input.
///
/// Please note that coordinate behavior may be surprising. [`embedded_graphics`]
/// considers coordinates to refer to pixel centers, which is similar but not identical
/// to our use of [`GridPoint`] to identify a cube by its low corner. The `transform` is
/// then applied to those coordinates. So, for example, applying [`GridMatrix::FLIP_Y`]
/// to a [`Rectangle`] whose top-left corner is `[0, 0]` will result in a [`GridAab`]
/// which *includes* the <var>y</var> = 0 row — not one which abuts it and is strictly in
/// the negative y range.
///
/// TODO: This function still has some bugs to work out
///
/// TODO: This function needs a better name
///
/// TODO: Handling zero-area rectangles is not implemented
pub(crate) fn rectangle_to_aab(
    rectangle: Rectangle,
    transform: GridMatrix,
    max_brush: GridAab,
) -> GridAab {
    loop {}
}
/// Adapter to use a [`Space`] or [`SpaceTransaction`] as a [`DrawTarget`].
/// Use [`Space::draw_target`] to construct this.
///
/// `'s` is the lifetime of the [`Space`].
/// `C` is the “color” type to use, which should implement [`VoxelColor`].
#[derive(Debug)]
pub struct DrawingPlane<'s, T, C> {
    space: &'s mut T,
    /// Defines the coordinate transformation from 2D graphics to the [`Space`].
    transform: GridMatrix,
    _color: PhantomData<fn(C)>,
}
/// Allows “drawing” blocks onto a [`DrawingPlane`], a two-dimensional coordinate system
/// established within a [`Space`].
///
/// Builds on [`PixelColor`] by defining a conversion to [`Block`]s and tracking depth.
/// [`PixelColor::Raw`] is ignored; the supertrait is present only because
/// [`embedded_graphics`] requires it.
pub trait VoxelColor<'a>: PixelColor {
    /// Returns a corresponding [`VoxelBrush`], the most general form of blocky drawing.
    fn into_blocks(self) -> VoxelBrush<'a>;
}
/// A shape of multiple blocks to “paint” with. This may be used to make copies of a
/// simple shape, or to make multi-layered "2.5D" drawings using [`DrawingPlane`].
///
/// Note that only `&VoxelBrush` implements [`PixelColor`]; this is because `PixelColor`
/// requires a value implementing [`Copy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoxelBrush<'a>(Vec<(GridPoint, Cow<'a, Block>)>);
/// Converts the return value of [`Space::set`] to the return value of
/// [`DrawTarget::draw_pixel`], by making out-of-bounds not an error.
fn ignore_out_of_bounds(result: Result<bool, SetCubeError>) -> Result<(), SetCubeError> {
    loop {}
}
/// Generate a set of blocks which together display the given [`Drawable`] which may be
/// larger than one block.
///
/// `z` specifies the origin z-coordinate within the blocks.
/// `z_range` specifies the range which is available for drawing; keeping this small
/// increases performance due to not processing many empty voxels.
///
/// Returns a `Space` containing all the blocks properly arranged, or an error if reading
/// the `Drawable`'s color-blocks fails.
pub(crate) fn draw_to_blocks<'c, D, C>(
    universe: &mut Universe,
    resolution: Resolution,
    z: GridCoordinate,
    z_range: Range<GridCoordinate>,
    attributes: BlockAttributes,
    object: &D,
) -> Result<Space, SetCubeError>
where
    D: Drawable<Color = C> + Dimensions,
    C: VoxelColor<'c>,
{
    loop {}
}
