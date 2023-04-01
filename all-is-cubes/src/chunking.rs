//! Algorithms for grouping cubes into cubical batches (chunks).
//!
//! Note: this module is currently private and a little crufty.
//! We will probably want to expose it but clean up the API first, particularly
//! clarifying the treatment of distances and squared distances.
use std::ops::RangeTo;
use std::sync::Arc;
use cgmath::Vector3;
use crate::math::{FreeCoordinate, GridAab, GridCoordinate, GridPoint, GridVector};
/// Type to distinguish chunk coordinates from cube coordinates.
///
/// Parameter `CHUNK_SIZE` is the number of cubes along the edge of a chunk.
/// The consequences are unspecified if it is not positive.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct ChunkPos<const CHUNK_SIZE: GridCoordinate>(pub GridPoint);
impl<const CHUNK_SIZE: GridCoordinate> std::fmt::Debug for ChunkPos<CHUNK_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl<const CHUNK_SIZE: GridCoordinate> ChunkPos<CHUNK_SIZE> {
    pub(crate) const ZERO: Self = Self(GridPoint::new(0, 0, 0));
    /// Returns the bounds of this chunk as a [`GridAab`].
    pub(crate) fn bounds(self) -> GridAab {
        loop {}
    }
    pub(crate) fn distance(self, other: Self) -> Distance {
        loop {}
    }
    /// Returns the squared distance along the shortest line from `origin_chunk`'s bounds
    /// to this chunk's bounds.
    ///
    /// This is the same criterion that [`ChunkChart`] uses for
    /// deciding whether a chunk is included in the chart or not.
    pub(crate) fn min_distance_squared_from(
        self,
        origin_chunk: ChunkPos<CHUNK_SIZE>,
    ) -> GridCoordinate {
        loop {}
    }
}
/// A distance between two chunks, taking into consideration their entire volume.
///
/// Implements [`Ord`] to be comparable as a distance value, with the following properties:
///
/// * It matches [`ChunkChart`]'s concept of view distance: the minimum Euclidean distance
///   from any point of two chunks, so that if nothing farther away than D can be seen
///   then this chunk cannot be seen from any point within the origin chunk.
/// * It is usable as a depth sort: chunks sorted by this distance from the chunk with
///   [`ChunkPos`] coordinates `[0, 0, 0]` will be sorted in back-to-front or front-to-back
///   order.
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Distance {}
impl std::fmt::Debug for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
/// Precomputed information about the spherical pattern of chunks within view distance.
///
/// In order to use the same pattern for all possible view positions, the view position is
/// rounded to enclosing chunk position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChunkChart<const CHUNK_SIZE: GridCoordinate> {}
/// A specification of which octants to include in [`ChunkChart::chunks()`].
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) struct OctantMask {
    /// A bit-mask of octants, where the bit positions are, LSB first, [-X-Y-Z, -X-Y+Z, ..., +X+Y+Z]
    flags: u8,
}
impl std::fmt::Debug for OctantMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl OctantMask {
    /// The mask including all octants.
    pub(crate) const ALL: Self = Self { flags: 0xFF };
    /// The mask including no octants.
    pub(crate) const NONE: Self = Self { flags: 0x00 };
    /// Set the flag for the octant the given vector occupies.
    pub(crate) fn set_octant_of(&mut self, vector: Vector3<FreeCoordinate>) {
        loop {}
    }
    /// Returns the index of the first octant included in the mask.
    ///
    /// Here “first” means the arbitrary ordering [`ChunkChart`] uses, which corresponds
    /// to the binary-counting ordering with X as MSB and Z as LSB:
    ///
    /// ```text
    /// 0 = -X -Y -Z
    /// 1 = -X -Y +Z
    /// 2 = -X +Y -Z
    /// 3 = -X +Y +Z
    /// 4 = +X -Y -Z
    /// 5 = +X -Y +Z
    /// 6 = +X +Y -Z
    /// 7 = +X +Y +Z
    /// ```
    #[inline(always)]
    fn first(self) -> Option<u8> {
        loop {}
    }
    /// As [`Self::first()`], but the opposite ordering.
    #[inline(always)]
    fn last(self) -> Option<u8> {
        loop {}
    }
}
