//! Loading images for use as game assets (i.e. [`Block`]s).
//!
//! TODO: stuff in this module is kind of duplicative of [`crate::drawing`]...
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::prelude::{DrawTarget, Size};
use embedded_graphics::primitives::Rectangle;
use image::{DynamicImage, GenericImageView};
use crate::drawing::VoxelBrush;
use crate::math::{GridAab, GridRotation};
use crate::space::{SetCubeError, Space};
/// Take the pixels of the image and construct a [`Space`] from it.
///
/// The `block_function` will be memoized.
///
/// TODO: Allow `SpaceBuilder` controls somehow. Maybe this belongs as a method on SpaceBuilder.
/// TODO: pixel_function should have a Result return
#[doc(hidden)]
pub(crate) fn space_from_image<'b, I, F>(
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
pub(crate) fn default_srgb<P: image::Pixel<Subpixel = u8>>(
    pixel: P,
) -> VoxelBrush<'static> {
    loop {}
}
/// Helper for [`include_image`] macro.
#[doc(hidden)]
pub(crate) fn load_png_from_bytes(name: &str, bytes: &'static [u8]) -> DynamicImage {
    loop {}
}
#[doc(hidden)]
pub(crate) use ::image::DynamicImage as DynamicImageForIncludeImage;
#[doc(hidden)]
pub(crate) use ::once_cell::sync::Lazy as LazyForIncludeImage;
