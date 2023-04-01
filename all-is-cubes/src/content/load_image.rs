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
#[doc(hidden)]
pub(crate) use ::image::DynamicImage as DynamicImageForIncludeImage;
#[doc(hidden)]
pub(crate) use ::once_cell::sync::Lazy as LazyForIncludeImage;
