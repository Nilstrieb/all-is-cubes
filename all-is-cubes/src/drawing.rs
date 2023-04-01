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
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::Drawable;
use embedded_graphics::primitives::Rectangle;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Range;
/// Re-export the version of the [`embedded_graphics`] crate we're using.
pub(crate) use embedded_graphics;
use crate::block::{Block, BlockAttributes, Resolution};
use crate::math::{GridAab, GridCoordinate, GridMatrix, GridPoint};
use crate::space::{SetCubeError, Space};
use crate::universe::Universe;
/// Adapter to use a [`Space`] or [`SpaceTransaction`] as a [`DrawTarget`].
/// Use [`Space::draw_target`] to construct this.
///
/// `'s` is the lifetime of the [`Space`].
/// `C` is the “color” type to use, which should implement [`VoxelColor`].
#[derive(Debug)]
pub(crate) struct DrawingPlane<'s, T, C> {
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
pub(crate) trait VoxelColor<'a>: PixelColor {
    /// Returns a corresponding [`VoxelBrush`], the most general form of blocky drawing.
    fn into_blocks(self) -> VoxelBrush<'a>;
}
/// A shape of multiple blocks to “paint” with. This may be used to make copies of a
/// simple shape, or to make multi-layered "2.5D" drawings using [`DrawingPlane`].
///
/// Note that only `&VoxelBrush` implements [`PixelColor`]; this is because `PixelColor`
/// requires a value implementing [`Copy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct VoxelBrush<'a>(Vec<(GridPoint, Cow<'a, Block>)>);
