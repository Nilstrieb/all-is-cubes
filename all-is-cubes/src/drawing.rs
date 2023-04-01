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

use std::marker::PhantomData;


/// Adapter to use a [`Space`] or [`SpaceTransaction`] as a [`DrawTarget`].
/// Use [`Space::draw_target`] to construct this.
///
/// `'s` is the lifetime of the [`Space`].
/// `C` is the “color” type to use, which should implement [`VoxelColor`].
#[derive(Debug)]
pub(crate) struct DrawingPlane<'s, T, C> {
    space: &'s mut T,
    _color: PhantomData<fn(C)>,
}
