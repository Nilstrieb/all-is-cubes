//! Algorithms for grouping cubes into cubical batches (chunks).
//!
//! Note: this module is currently private and a little crufty.
//! We will probably want to expose it but clean up the API first, particularly
//! clarifying the treatment of distances and squared distances.

use crate::math::{GridCoordinate};
/// Type to distinguish chunk coordinates from cube coordinates.
///
/// Parameter `CHUNK_SIZE` is the number of cubes along the edge of a chunk.
/// The consequences are unspecified if it is not positive.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct ChunkPos<const CHUNK_SIZE: GridCoordinate>();
impl<const CHUNK_SIZE: GridCoordinate> std::fmt::Debug for ChunkPos<CHUNK_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
/// Precomputed information about the spherical pattern of chunks within view distance.
///
/// In order to use the same pattern for all possible view positions, the view position is
/// rounded to enclosing chunk position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ChunkChart<const CHUNK_SIZE: GridCoordinate> {}
