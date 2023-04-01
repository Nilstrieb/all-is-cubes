//! Algorithms for grouping cubes into cubical batches (chunks).
//!
//! Note: this module is currently private and a little crufty.
//! We will probably want to expose it but clean up the API first, particularly
//! clarifying the treatment of distances and squared distances.
use crate::math::GridCoordinate;
