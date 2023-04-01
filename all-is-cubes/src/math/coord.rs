//! Numeric types used for coordinates and related quantities.
use cgmath::{Point3};
/// Coordinates that are locked to the cube grid.
pub(crate) type GridCoordinate = usize;
/// Positions that are locked to the cube grid.
pub(crate) type GridPoint = Point3<GridCoordinate>;
