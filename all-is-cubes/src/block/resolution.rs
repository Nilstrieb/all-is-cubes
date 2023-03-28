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
pub enum Resolution {
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
impl Resolution {
    /// Returns the [`Resolution`] that’s twice this one, or [`None`] at the limit.
    #[inline]
    pub const fn double(self) -> Option<Self> {
        loop {}
    }
    /// Returns the [`Resolution`] that’s half this one, or [`None`] if `self` is
    /// [`R1`](Self::R1).
    #[inline]
    pub const fn halve(self) -> Option<Self> {
        loop {}
    }
    #[inline]
    #[doc(hidden)]
    pub const fn to_grid(self) -> GridCoordinate {
        loop {}
    }
}
impl fmt::Debug for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for Resolution {
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
impl From<Resolution> for i32 {
    /// ```
    /// use all_is_cubes::block::Resolution;
    ///
    /// assert_eq!(64, i32::from(Resolution::R64));
    /// ```
    #[inline]
    fn from(r: Resolution) -> i32 {
        loop {}
    }
}
impl From<Resolution> for u16 {
    #[inline]
    fn from(r: Resolution) -> u16 {
        loop {}
    }
}
impl From<Resolution> for u32 {
    #[inline]
    fn from(r: Resolution) -> u32 {
        loop {}
    }
}
impl From<Resolution> for usize {
    #[inline]
    fn from(r: Resolution) -> usize {
        loop {}
    }
}
impl From<Resolution> for f32 {
    #[inline]
    fn from(r: Resolution) -> f32 {
        loop {}
    }
}
impl From<Resolution> for f64 {
    #[inline]
    fn from(r: Resolution) -> f64 {
        loop {}
    }
}
impl ops::Mul<Resolution> for Resolution {
    type Output = Option<Resolution>;
    fn mul(self, rhs: Resolution) -> Self::Output {
        loop {}
    }
}
impl ops::Div<Resolution> for Resolution {
    type Output = Option<Resolution>;
    fn div(self, rhs: Resolution) -> Self::Output {
        loop {}
    }
}
impl serde::Serialize for Resolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        loop {}
    }
}
impl<'de> serde::Deserialize<'de> for Resolution {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        loop {}
    }
}
/// Error type produced by [`TryFrom`] for [`Resolution`], and deserializing resolutions,
/// when the number is not a permitted resolution value.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, thiserror::Error)]
pub struct IntoResolutionError<N>(N);
impl<N: fmt::Display> fmt::Display for IntoResolutionError<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use Resolution::*;
    const RS: [Resolution; 8] = [R1, R2, R4, R8, R16, R32, R64, R128];
    #[test]
    fn resolution_steps() {
        loop {}
    }
    #[test]
    fn resolution_values() {
        loop {}
    }
    #[test]
    fn mul() {
        loop {}
    }
    #[test]
    fn div() {
        loop {}
    }
    #[test]
    fn ser_ok() {
        loop {}
    }
    #[test]
    fn de_ok() {
        loop {}
    }
    #[test]
    fn de_err() {
        loop {}
    }
}
