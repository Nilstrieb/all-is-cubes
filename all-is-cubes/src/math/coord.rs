//! Numeric types used for coordinates and related quantities.
use cgmath::{Point3, Vector3};
/// Coordinates that are locked to the cube grid.
pub(crate) type GridCoordinate = i32;
/// Positions that are locked to the cube grid.
pub(crate) type GridPoint = Point3<GridCoordinate>;
/// Vectors that are locked to the cube grid.
pub(crate) type GridVector = Vector3<GridCoordinate>;
/// Coordinates that are not locked to the cube grid.
///
/// Note: Because `GridCoordinate = i32` and `FreeCoordinate = f64`, which has
/// more than 32 bits of mantissa, the infallible conversion
/// `From<GridCoordinate> for FreeCoordinate` exists, which is often convenient.
pub(crate) type FreeCoordinate = f64;
