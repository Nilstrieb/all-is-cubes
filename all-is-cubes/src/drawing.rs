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
impl<'s, T, C> DrawingPlane<'s, T, C> {
    pub(crate) fn new(space: &'s mut T, transform: GridMatrix) -> Self {
        loop {}
    }
    /// Converts 2D point to 3D point. Helper for multiple `impl DrawTarget`s.
    fn convert_point(&self, point: Point) -> GridPoint {
        loop {}
    }
}
/// A [`DrawingPlane`] accepts any color type that implements [`VoxelColor`].
impl<'c, C> DrawTarget for DrawingPlane<'_, Space, C>
where
    C: VoxelColor<'c>,
{
    type Color = C;
    type Error = SetCubeError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        loop {}
    }
}
/// A [`DrawingPlane`] accepts any color type that implements [`VoxelColor`].
impl<'c, C> DrawTarget for DrawingPlane<'_, SpaceTransaction, C>
where
    C: VoxelColor<'c>,
{
    type Color = C;
    type Error = SetCubeError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        loop {}
    }
}
impl<C> Dimensions for DrawingPlane<'_, Space, C> {
    fn bounding_box(&self) -> Rectangle {
        loop {}
    }
}
impl<C> Dimensions for DrawingPlane<'_, SpaceTransaction, C> {
    fn bounding_box(&self) -> Rectangle {
        loop {}
    }
}
/// Adapt [`embedded_graphics`]'s most general color type to ours.
impl From<Rgb888> for Rgb {
    #[inline]
    fn from(color: Rgb888) -> Rgb {
        loop {}
    }
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
impl<'a> PixelColor for &'a Block {
    type Raw = ();
}
impl<'a> VoxelColor<'a> for &'a Block {
    fn into_blocks(self) -> VoxelBrush<'a> {
        loop {}
    }
}
impl PixelColor for Rgb {
    type Raw = ();
}
impl<'a> VoxelColor<'a> for Rgb {
    fn into_blocks(self) -> VoxelBrush<'a> {
        loop {}
    }
}
impl PixelColor for Rgba {
    type Raw = ();
}
impl<'a> VoxelColor<'a> for Rgba {
    fn into_blocks(self) -> VoxelBrush<'a> {
        loop {}
    }
}
/// Adapt [`embedded_graphics`]'s most general color type to ours.
impl<'a> VoxelColor<'a> for Rgb888 {
    fn into_blocks(self) -> VoxelBrush<'a> {
        loop {}
    }
}
/// A shape of multiple blocks to “paint” with. This may be used to make copies of a
/// simple shape, or to make multi-layered "2.5D" drawings using [`DrawingPlane`].
///
/// Note that only `&VoxelBrush` implements [`PixelColor`]; this is because `PixelColor`
/// requires a value implementing [`Copy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoxelBrush<'a>(Vec<(GridPoint, Cow<'a, Block>)>);
impl<'a> VoxelBrush<'a> {
    /// Makes a [`VoxelBrush`] which paints the specified blocks at the specified offsets
    /// from each pixel position.
    pub(crate) fn new<V, B>(blocks: impl IntoIterator<Item = (V, B)>) -> Self
    where
        V: Into<GridPoint>,
        B: Into<Cow<'a, Block>>,
    {
        loop {}
    }
    /// Makes a [`VoxelBrush`] which paints the specified block with no offset.
    pub(crate) fn single<B>(block: B) -> Self
    where
        B: Into<Cow<'a, Block>>,
    {
        loop {}
    }
    /// Makes a [`VoxelBrush`] which paints the specified block within the specified Z-axis range.
    pub(crate) fn with_thickness<B>(block: B, range: Range<GridCoordinate>) -> Self
    where
        B: Into<Cow<'a, Block>>,
    {
        loop {}
    }
    /// Copies each of the brush's blocks into the `Space` relative to the given origin
    /// point.
    ///
    /// Unlike [`Space::set`], it is not considered an error if any of the affected cubes
    /// fall outside of the `Space`'s bounds.
    pub(crate) fn paint(
        &self,
        space: &mut Space,
        origin: GridPoint,
    ) -> Result<(), SetCubeError> {
        loop {}
    }
    /// Creates a transaction equivalent to [`VoxelBrush::paint`].
    ///
    /// Note that [`VoxelBrush::paint`] or using it in a [`DrawTarget`] ignores
    /// out-of-bounds drawing, but transactions do not support this and will fail instead.
    pub(crate) fn paint_transaction(&self, origin: GridPoint) -> SpaceTransaction {
        loop {}
    }
    /// Like [`Self::paint_transaction()`] but modifies an existing transaction (as per
    /// [`SpaceTransaction::set_overwrite()`]).
    ///
    /// Note that [`VoxelBrush::paint`] or using it in a [`DrawTarget`] ignores
    /// out-of-bounds drawing, but transactions do not support this and will fail instead.
    pub(crate) fn paint_transaction_mut(
        &self,
        transaction: &mut SpaceTransaction,
        origin: GridPoint,
    ) {
        loop {}
    }
    /// Converts a `&VoxelBrush` into a `VoxelBrush` that borrows it.
    pub(crate) fn as_ref(&self) -> VoxelBrush<'_> {
        loop {}
    }
    /// Converts a `VoxelBrush` with borrowed blocks to one with owned blocks.
    pub(crate) fn into_owned(self) -> VoxelBrush<'static> {
        loop {}
    }
    /// Add the given offset to the offset of each block, offsetting everything drawn.
    #[must_use]
    pub(crate) fn translate<V: Into<GridVector>>(mut self, offset: V) -> Self {
        loop {}
    }
    /// Apply the given transform to the position of each block.
    #[must_use]
    pub(crate) fn transform(mut self, transform: GridMatrix) -> Self {
        loop {}
    }
    /// Computes the region affected by this brush, as if it were painted at the origin.
    ///
    /// Returns [`None`] if the brush is empty.
    pub(crate) fn bounds(&self) -> Option<GridAab> {
        loop {}
    }
}
impl<'a> PixelColor for &'a VoxelBrush<'a> {
    type Raw = ();
}
impl<'a> VoxelColor<'a> for &'a VoxelBrush<'a> {
    fn into_blocks(self) -> VoxelBrush<'a> {
        loop {}
    }
}
impl<'a> From<&'a VoxelBrush<'a>> for SpaceTransaction {
    /// Converts the brush into an equivalent transaction, as by
    /// [`VoxelBrush::paint_transaction`] at the origin.
    fn from(brush: &'a VoxelBrush<'a>) -> Self {
        loop {}
    }
}
impl<'a> From<VoxelBrush<'a>> for SpaceTransaction {
    /// Converts the brush into an equivalent transaction, as by
    /// [`VoxelBrush::paint_transaction`] at the origin.
    fn from(brush: VoxelBrush<'a>) -> Self {
        loop {}
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::EvalBlockError;
    use crate::block::{
        self, Resolution::{R1, R16},
        AIR,
    };
    use crate::content::make_some_blocks;
    use crate::math::{GridRotation, Rgba};
    use crate::raytracer::print_space;
    use crate::universe::{Name, RefError, URef, Universe};
    use cgmath::{One, Zero};
    use embedded_graphics::primitives::{Primitive, PrimitiveStyle};
    /// With identity transform, `rectangle_to_aab`'s output matches exactly as one might
    /// expect.
    #[test]
    fn rectangle_to_aab_simple() {
        loop {}
    }
    #[test]
    fn rectangle_to_aab_y_flipped() {
        loop {}
    }
    #[test]
    fn rectangle_to_aab_with_brush() {
        loop {}
    }
    #[test]
    #[ignore = "TODO: make this succeed"]
    fn rectangle_to_aab_empty_rects_no_transform() {
        loop {}
    }
    /// Test consistency between [`rectangle_to_aab`], the cubes affected by actual drawing,
    /// and `<DrawingPlane as Dimensions>::bounding_box()`.
    #[test]
    fn rectangle_to_aab_consistent_with_drawing_and_bounding_box() {
        loop {}
    }
    /// Test using a particular color type with [`DrawingPlane`].
    fn test_color_drawing<'c, C>(color_value: C, expected_block: &Block)
    where
        C: VoxelColor<'c>,
    {
        loop {}
    }
    #[test]
    fn draw_with_block_ref() {
        loop {}
    }
    #[test]
    fn draw_with_eg_rgb888() {
        loop {}
    }
    #[test]
    fn draw_with_our_rgb() {
        loop {}
    }
    #[test]
    fn draw_with_our_rgba() {
        loop {}
    }
    #[test]
    fn draw_with_brush() -> Result<(), SetCubeError> {
        loop {}
    }
    #[test]
    fn draw_out_of_bounds_is_ok() -> Result<(), SetCubeError> {
        loop {}
    }
    #[test]
    fn draw_set_failure() {
        loop {}
    }
    fn a_primitive_style() -> PrimitiveStyle<Rgba> {
        loop {}
    }
    /// Cube color corresponding to `a_primitive_style()`.
    fn a_primitive_color() -> Rgba {
        loop {}
    }
    #[test]
    fn draw_to_blocks_bounds_one_block() {
        loop {}
    }
    #[test]
    fn draw_to_blocks_bounds_negative_coords_one_block() {
        loop {}
    }
    #[test]
    fn voxel_brush_single() {
        loop {}
    }
    #[test]
    fn voxel_brush_translate() {
        loop {}
    }
    /// Test that `VoxelBrush::bounds()` gives the same result as `SpaceTransaction::bounds()`.
    #[test]
    fn voxel_brush_bounds() {
        loop {}
    }
}
