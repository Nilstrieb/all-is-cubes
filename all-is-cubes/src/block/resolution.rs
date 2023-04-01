use std::ops;
#[cfg(doc)]
use crate::block::{EvaluatedBlock, Modifier, Primitive};
/// Scale factor between a [recursive block](Primitive::Recur) and its component voxels.
///
/// This resolution cubed is the number of voxels making up a block.
///
/// Resolutions are always powers of 2. This ensures that the arithmetic is well-behaved
/// (no division by zero, exact floating-point representation, and the potential of
/// fixed-point representation),
/// and that it is always possible to subdivide a block further (up to the limit) without
/// shifting the existing voxel boundaries.
///
/// Note that while quite high resolutions are permitted, this does not mean that it is
/// practical to routinely use full blocks at that resolution. For example, 64 × 64 × 64
/// = 262,144 voxels, occupying several megabytes just for color data.
/// High resolutions are permitted for special purposes that do not necessarily use the
/// full cube volume:
///
/// * *Thin* blocks (e.g. 128 × 128 × 1) can display high resolution text and other 2D
///   images.
/// * Multi-block structures can be defined using [`Modifier::Zoom`]; their total size
///   is limited by the resolution limit.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, exhaust::Exhaust)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[allow(missing_docs)]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum Resolution {
    R1 = 0,
    R2 = 1,
    R4 = 2,
    R8 = 3,
    R16 = 4,
    R32 = 5,
    R64 = 6,
    R128 = 7,
}
use std::fmt;
use crate::math::GridCoordinate;
impl fmt::Debug for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
macro_rules! impl_try_from {
    ($t:ty) => {
        impl TryFrom <$t > for Resolution { type Error = IntoResolutionError <$t >;
        #[inline] fn try_from(value : $t) -> Result < Self, Self::Error > { match value {
        1 => Ok(Self::R1), 2 => Ok(Self::R2), 4 => Ok(Self::R4), 8 => Ok(Self::R8), 16 =>
        Ok(Self::R16), 32 => Ok(Self::R32), 64 => Ok(Self::R64), 128 => Ok(Self::R128), _
        => Err(IntoResolutionError(value)), } } }
    };
}
impl_try_from!(i16);
impl_try_from!(i32);
impl_try_from!(i64);
impl_try_from!(i128);
impl_try_from!(isize);
impl_try_from!(u16);
impl_try_from!(u32);
impl_try_from!(u64);
impl_try_from!(u128);
impl_try_from!(usize);
/// Error type produced by [`TryFrom`] for [`Resolution`], and deserializing resolutions,
/// when the number is not a permitted resolution value.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, thiserror::Error)]
pub(crate) struct IntoResolutionError<N>(N);
