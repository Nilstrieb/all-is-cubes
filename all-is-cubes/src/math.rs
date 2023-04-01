//! Mathematical utilities and decisions.
use std::fmt;
use cgmath::{EuclideanSpace as _, Vector3};
use num_traits::identities::Zero;
pub(crate) use ordered_float::{FloatIsNan, NotNan};
use crate::util::CustomFormat;
mod aab;
pub(crate) use aab::*;
#[macro_use]
mod color;
pub(crate) use color::*;
mod coord;
pub(crate) use coord::*;
mod face;
pub(crate) use face::*;
mod grid_aab;
pub(crate) use grid_aab::*;
mod matrix;
pub(crate) use matrix::*;
mod rotation;
pub(crate) use rotation::*;
/// Allows writing a [`NotNan`] value as a constant expression (which is not currently
/// a feature provided by the [`ordered_float`] crate itself).
///
/// Note that if the expression does not need to be constant, this macro may not be
/// needed; infallible construction can be written using `NotNan::from(an_integer)`,
/// `NotNan::zero()`, and `NotNan::one()`.
///
/// ```
/// use all_is_cubes::{notnan, math::NotNan};
///
/// const X: NotNan<f32> = notnan!(1.234);
/// ```
///
/// ```compile_fail
/// # use all_is_cubes::{notnan, math::NotNan};
/// // Not a literal; will not compile
/// const X: NotNan<f32> = notnan!(f32::NAN);
/// ```
///
/// ```compile_fail
/// # use all_is_cubes::{notnan, math::NotNan};
/// // Not a literal; will not compile
/// const X: NotNan<f32> = notnan!(0.0 / 0.0);
/// ```
///
/// ```compile_fail
/// # use all_is_cubes::{notnan, math::NotNan};
/// const N0N: f32 = f32::NAN;
/// // Not a literal; will not compile
/// const X: NotNan<f32> = notnan!(N0N);
/// ```
///
/// ```compile_fail
/// # use all_is_cubes::{notnan, math::NotNan};
/// // Not a float; will not compile
/// const X: NotNan<char> = notnan!('a');
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! notnan {
    ($value:literal) => {
        match $value { value => { let result = unsafe { $crate
        ::math::NotNan::new_unchecked(value) }; let _ = if false { unsafe { $crate
        ::math::NotNan::new_unchecked(0.0) } } else { result }; result } }
    };
}
#[inline]
pub(crate) fn smoothstep(x: f64) -> f64 {
    loop {}
}
#[inline]
pub(crate) fn point_checked_add(p: GridPoint, v: GridVector) -> Option<GridPoint> {
    loop {}
}
/// Sort exactly two items; swap them if `a > b`.
#[inline]
pub(crate) fn sort_two<T: PartialOrd>(a: &mut T, b: &mut T) {
    loop {}
}
/// Common features of objects that have a location and shape in space.
pub(crate) trait Geometry {
    /// Type of coordinates; generally determines whether this object can be translated by a
    /// non-integer amount.
    type Coord;
    /// Translate (move) this object by the specified offset.
    #[must_use]
    fn translate(self, offset: Vector3<Self::Coord>) -> Self;
    /// Represent this object as a line drawing, or wireframe.
    ///
    /// The generated points should be in pairs, each pair defining a line segment.
    /// If there are an odd number of vertices, the caller should ignore the last.
    ///
    /// TODO: This should probably return an iterator instead, but defining the type
    /// will be awkward until `type_alias_impl_trait` is stable.
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<crate::mesh::LineVertex>;
}
