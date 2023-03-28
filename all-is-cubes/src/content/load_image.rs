//! Loading images for use as game assets (i.e. [`Block`]s).
//!
//! TODO: stuff in this module is kind of duplicative of [`crate::drawing`]...
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::ops::Deref;
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::prelude::{Dimensions as _, DrawTarget, Point, Size};
use embedded_graphics::primitives::{PointsIter, Rectangle};
use embedded_graphics::Drawable;
use image::{DynamicImage, GenericImageView};
use crate::block::{Block, AIR};
use crate::drawing::{rectangle_to_aab, VoxelBrush};
use crate::math::{GridAab, GridRotation, Rgba};
use crate::space::{SetCubeError, Space, SpacePhysics};
/// Adapter from [`image::GenericImageView`] to [`embedded_graphics::Drawable`].
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct ImageAdapter<'b, I>
where
    I: Deref,
    I::Target: GenericImageView,
{
    image_ref: I,
    color_map: HashMap<<I::Target as GenericImageView>::Pixel, VoxelBrush<'b>>,
    max_brush: GridAab,
}
impl<'b, I> ImageAdapter<'b, I>
where
    I: Deref,
    I::Target: GenericImageView + Sized,
    <I::Target as GenericImageView>::Pixel: Eq + Hash,
{
    pub fn adapt(
        image_ref: I,
        mut pixel_function: impl FnMut(
            <I::Target as GenericImageView>::Pixel,
        ) -> VoxelBrush<'b>,
    ) -> Self {
        loop {}
    }
}
/// Note: This implementation is on references so it can return [`VoxelBrush`]es
/// borrowing from itself.
impl<'b, I> ImageDrawable for &'b ImageAdapter<'b, I>
where
    I: Deref,
    I::Target: GenericImageView,
    <I::Target as GenericImageView>::Pixel: Eq + Hash,
{
    type Color = &'b VoxelBrush<'b>;
    fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        loop {}
    }
    fn draw_sub_image<D>(
        &self,
        target: &mut D,
        area: &Rectangle,
    ) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        loop {}
    }
}
impl<I> embedded_graphics::geometry::OriginDimensions for &'_ ImageAdapter<'_, I>
where
    I: Deref,
    I::Target: GenericImageView,
{
    fn size(&self) -> Size {
        loop {}
    }
}
/// Take the pixels of the image and construct a [`Space`] from it.
///
/// The `block_function` will be memoized.
///
/// TODO: Allow `SpaceBuilder` controls somehow. Maybe this belongs as a method on SpaceBuilder.
/// TODO: pixel_function should have a Result return
#[doc(hidden)]
pub fn space_from_image<'b, I, F>(
    image: &I,
    rotation: GridRotation,
    pixel_function: F,
) -> Result<Space, SetCubeError>
where
    I: GenericImageView,
    I::Pixel: Eq + std::hash::Hash,
    F: FnMut(I::Pixel) -> VoxelBrush<'b>,
{
    loop {}
}
/// Simple function for [`space_from_image()`] pixel conversion.
///
/// Special case:
/// All pixels with 0 alpha (regardless of other channel values) are converted to
/// [`AIR`], to meet normal expectations about collision, selection, and equality.
#[doc(hidden)]
pub fn default_srgb<P: image::Pixel<Subpixel = u8>>(pixel: P) -> VoxelBrush<'static> {
    loop {}
}
/// Helper for [`include_image`] macro.
#[doc(hidden)]
pub fn load_png_from_bytes(name: &str, bytes: &'static [u8]) -> DynamicImage {
    loop {}
}
#[doc(hidden)]
pub use ::image::DynamicImage as DynamicImageForIncludeImage;
#[doc(hidden)]
pub use ::once_cell::sync::Lazy as LazyForIncludeImage;
/// Load an image from a relative path, memoized.
#[doc(hidden)]
#[macro_export]
macro_rules! include_image {
    ($path:literal) => {
        { static IMAGE : $crate ::content::load_image::LazyForIncludeImage < $crate
        ::content::load_image::DynamicImageForIncludeImage, > = $crate
        ::content::load_image::LazyForIncludeImage::new(|| { $crate
        ::content::load_image::load_png_from_bytes($path, include_bytes!($path)) }); &*
        IMAGE }
    };
}
pub(crate) use include_image;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Rgb;
    fn test_image() -> image::RgbaImage {
        loop {}
    }
    #[test]
    fn basic_image() {
        loop {}
    }
    #[test]
    fn basic_image_transformed() {
        loop {}
    }
    #[test]
    fn transparent_pixels_are_air() {
        loop {}
    }
    #[test]
    fn bounds_are_affected_by_brush() {
        loop {}
    }
}
