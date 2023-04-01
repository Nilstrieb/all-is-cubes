//! [`Cursor`] type and related items.
//!
//! TODO: It's unclear what the scope of this module should be.
use std::fmt;
use crate::math::{
    Face7, FreeCoordinate, Geometry, GridCoordinate, GridPoint, GridVector,
};
use crate::mesh::LineVertex;
use crate::raycast::Ray;
use crate::space::Space;
use crate::universe::URef;
/// Data collected by [`cursor_raycast`] about the blocks struck by the ray; intended to be
/// sufficient for various player interactions with blocks.
///
/// TODO: Should carry information about both the struck and preceding cubes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cursor {}
