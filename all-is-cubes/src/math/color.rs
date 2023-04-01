//! Color data types. This module is private but reexported by its parent.
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Sub};
use cgmath::{Vector3, Vector4};
pub(crate) use ordered_float::{FloatIsNan, NotNan};
use crate::notnan;
/// A floating-point RGB color value.
///
/// * Each component may be considered to have a nominal range of 0 to 1, but larger
///   values are permitted — corresponding to bright light sources and other such
///   things which it is reasonable to “overexpose”. (No meaning is given to negative
///   values, but they are permitted.)
/// * NaN is banned so that [`Eq`] may be implemented. (Infinities are permitted.)
/// * Color values are linear (gamma = 1), but use the same RGB primaries as sRGB
///   (Rec. 709).
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) struct Rgb(Vector3<NotNan<f32>>);
/// A floating-point RGBA color value.
///
/// * Each color component may be considered to have a nominal range of 0 to 1, but
///   larger values are permitted — corresponding to bright light sources and other such
///   things which it is reasonable to “overexpose”. (No meaning is given to negative
///   values, but they are permitted.)
/// * NaN is banned so that [`Eq`] may be implemented. (Infinities are permitted.)
/// * Color values are linear (gamma = 1), but use the same RGB primaries as sRGB
///   (Rec. 709).
/// * The alpha is not premultiplied.
/// * Alpha values less than zero and greater than one will be treated equivalently to
///   zero and one, respectively, but are preserved rather than clipped.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) struct Rgba(Vector4<NotNan<f32>>);
const NN0: NotNan<f32> = notnan!(0.0);
const NN1: NotNan<f32> = notnan!(1.0);
impl Rgb {
    /// Black; the constant equal to `Rgb::new(0., 0., 0.).unwrap()`.
    pub(crate) const ZERO: Rgb = Rgb(Vector3::new(NN0, NN0, NN0));
    /// Nominal white; the constant equal to `Rgb::new(1., 1., 1.).unwrap()`.
    ///
    /// Note that brighter values may exist; the color system “supports HDR”.
    pub(crate) const ONE: Rgb = Rgb(Vector3::new(NN1, NN1, NN1));
    /// Constructs a color from components. Panics if any component is NaN.
    /// No other range checks are performed.
    #[inline]
    pub(crate) fn new(r: f32, g: f32, b: f32) -> Self {
        loop {}
    }
    /// Constructs a color from components that have already been checked for not being
    /// NaN.
    ///
    /// Note: This exists primarily to assist the [`rgb_const!`] macro and may be renamed
    /// or replaced in future versions.
    #[inline]
    pub(crate) const fn new_nn(r: NotNan<f32>, g: NotNan<f32>, b: NotNan<f32>) -> Self {
        loop {}
    }
    /// Constructs a shade of gray (components all equal). Panics if any component is NaN.
    /// No other range checks are performed.
    #[inline]
    pub(crate) fn from_luminance(luminance: f32) -> Self {
        loop {}
    }
    /// Adds an alpha component to produce an [Rgba] color.
    #[inline]
    pub(crate) const fn with_alpha(self, alpha: NotNan<f32>) -> Rgba {
        loop {}
    }
    /// Adds an alpha component of `1.0` (fully opaque) to produce an [Rgba] color.
    #[inline]
    pub(crate) const fn with_alpha_one(self) -> Rgba {
        loop {}
    }
    /// Returns the red color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn red(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns the green color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn green(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns the blue color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn blue(self) -> NotNan<f32> {
        loop {}
    }
    /// Combines the red, green, and blue components to obtain a [relative luminance]
    /// (“grayscale”) value. This will be equal to 1 if all components are 1.
    ///
    /// ```
    /// use all_is_cubes::math::Rgb;
    ///
    /// assert_eq!(0.0, Rgb::ZERO.luminance());
    /// assert_eq!(0.5, (Rgb::ONE * 0.5).luminance());
    /// assert_eq!(1.0, Rgb::ONE.luminance());
    /// assert_eq!(2.0, (Rgb::ONE * 2.0).luminance());
    ///
    /// assert_eq!(0.2126, Rgb::new(1., 0., 0.).luminance());
    /// assert_eq!(0.7152, Rgb::new(0., 1., 0.).luminance());
    /// assert_eq!(0.0722, Rgb::new(0., 0., 1.).luminance());
    /// ```
    ///
    /// [relative luminance]: https://en.wikipedia.org/wiki/Relative_luminance
    #[inline]
    pub(crate) fn luminance(self) -> f32 {
        loop {}
    }
    /// Converts sRGB 8-bits-per-component color to the corresponding linear [`Rgba`] value.
    #[inline]
    pub(crate) const fn from_srgb8(rgb: [u8; 3]) -> Self {
        loop {}
    }
    /// Clamp each component to lie within the range 0 to 1, inclusive.
    #[inline]
    #[must_use]
    pub(crate) fn clamp(self) -> Self {
        loop {}
    }
}
impl Rgba {
    /// Transparent black (all components zero); identical to
    /// `Rgba::new(0.0, 0.0, 0.0, 0.0)` except for being a constant.
    pub(crate) const TRANSPARENT: Rgba = Rgba(Vector4::new(NN0, NN0, NN0, NN0));
    /// Black; identical to `Rgba::new(0.0, 0.0, 0.0, 1.0)` except for being a constant.
    pub(crate) const BLACK: Rgba = Rgba(Vector4::new(NN0, NN0, NN0, NN1));
    /// White; identical to `Rgba::new(1.0, 1.0, 1.0, 1.0)` except for being a constant.
    pub(crate) const WHITE: Rgba = Rgba(Vector4::new(NN1, NN1, NN1, NN1));
    /// Constructs a color from components. Panics if any component is NaN.
    /// No other range checks are performed.
    #[inline]
    pub(crate) fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        loop {}
    }
    /// Constructs a color from components that have already been checked for not being
    /// NaN.
    ///
    /// Note: This exists primarily to assist the [`rgb_const!`] macro and may be renamed
    /// or replaced in future versions.
    #[inline]
    pub(crate) const fn new_nn(
        r: NotNan<f32>,
        g: NotNan<f32>,
        b: NotNan<f32>,
        a: NotNan<f32>,
    ) -> Self {
        loop {}
    }
    /// Constructs a shade of gray (components all equal). Panics if any component is NaN.
    /// No other range checks are performed.
    #[inline]
    pub(crate) fn from_luminance(luminance: f32) -> Self {
        loop {}
    }
    /// Returns the red color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn red(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns the green color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn green(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns the blue color component. Values are linear (gamma = 1).
    #[inline]
    pub(crate) const fn blue(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns the alpha component.
    ///
    /// Alpha is not premultiplied. Alpha values less than zero and greater than one are
    /// allowed and may be returned by this method, but alpha test methods will treat
    #[inline]
    pub(crate) const fn alpha(self) -> NotNan<f32> {
        loop {}
    }
    /// Returns whether this color is fully transparent, or has an alpha component of
    /// zero or less.
    #[inline]
    pub(crate) fn fully_transparent(self) -> bool {
        loop {}
    }
    /// Returns whether this color is fully opaque, or has an alpha component of
    /// one or greater.
    #[inline]
    pub(crate) fn fully_opaque(self) -> bool {
        loop {}
    }
    /// Returns the [`OpacityCategory`] which this color's alpha fits into.
    /// This returns the same information as [`Rgba::fully_transparent`] combined with
    /// [`Rgba::fully_opaque`].
    #[inline]
    pub(crate) fn opacity_category(self) -> OpacityCategory {
        loop {}
    }
    /// Discards the alpha component to produce an RGB color.
    ///
    /// Note that if alpha is 0 then the components could be any value and yet be “hidden”
    /// by the transparency.
    #[inline]
    pub(crate) fn to_rgb(self) -> Rgb {
        loop {}
    }
    /// Applies a function to the RGB portion of this color.
    #[must_use]
    pub(crate) fn map_rgb(self, f: impl FnOnce(Rgb) -> Rgb) -> Self {
        loop {}
    }
    /// Combines the red, green, and blue components to obtain a luminance (“grayscale”)
    /// value. This will be equal to 1 if all components are 1.
    ///
    /// This is identical to [`Rgb::luminance`], ignoring the alpha component.
    #[inline]
    pub(crate) fn luminance(self) -> f32 {
        loop {}
    }
    /// Converts this color to sRGB (nonlinear RGB components).
    #[inline]
    #[doc(hidden)]
    pub(crate) fn to_srgb_float(self) -> [f32; 4] {
        loop {}
    }
    /// Converts this color lossily to sRGB 8-bits-per-component color.
    #[inline]
    pub(crate) fn to_srgb8(self) -> [u8; 4] {
        loop {}
    }
    /// Converts sRGB 8-bits-per-component color to the corresponding linear [`Rgba`] value.
    #[inline]
    pub(crate) const fn from_srgb8(rgba: [u8; 4]) -> Self {
        loop {}
    }
    /// Clamp each component to lie within the range 0 to 1, inclusive.
    #[inline]
    #[must_use]
    pub(crate) fn clamp(self) -> Self {
        loop {}
    }
}
impl From<Vector3<NotNan<f32>>> for Rgb {
    #[inline]
    fn from(value: Vector3<NotNan<f32>>) -> Self {
        loop {}
    }
}
impl From<Vector4<NotNan<f32>>> for Rgba {
    #[inline]
    fn from(value: Vector4<NotNan<f32>>) -> Self {
        loop {}
    }
}
impl From<[NotNan<f32>; 3]> for Rgb {
    #[inline]
    fn from(value: [NotNan<f32>; 3]) -> Self {
        loop {}
    }
}
impl From<[NotNan<f32>; 4]> for Rgba {
    #[inline]
    fn from(value: [NotNan<f32>; 4]) -> Self {
        loop {}
    }
}
impl TryFrom<Vector3<f32>> for Rgb {
    type Error = FloatIsNan;
    #[inline]
    fn try_from(value: Vector3<f32>) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl TryFrom<Vector4<f32>> for Rgba {
    type Error = FloatIsNan;
    #[inline]
    fn try_from(value: Vector4<f32>) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl Add<Rgb> for Rgb {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        loop {}
    }
}
impl Add<Rgba> for Rgba {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        loop {}
    }
}
impl AddAssign<Rgb> for Rgb {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        loop {}
    }
}
impl AddAssign<Rgba> for Rgba {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        loop {}
    }
}
impl Sub<Rgb> for Rgb {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        loop {}
    }
}
impl Sub<Rgba> for Rgba {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        loop {}
    }
}
/// Multiplies two color values componentwise.
impl Mul<Rgb> for Rgb {
    type Output = Self;
    /// Multiplies two color values componentwise.
    #[inline]
    fn mul(self, other: Rgb) -> Self {
        loop {}
    }
}
/// Multiplies this color value by a scalar.
impl Mul<NotNan<f32>> for Rgb {
    type Output = Self;
    /// Multiplies this color value by a scalar.
    #[inline]
    fn mul(self, scalar: NotNan<f32>) -> Self {
        loop {}
    }
}
/// Multiplies this color value by a scalar. Panics if the scalar is NaN.
impl Mul<f32> for Rgb {
    type Output = Self;
    /// Multiplies this color value by a scalar. Panics if the scalar is NaN.
    #[inline]
    fn mul(self, scalar: f32) -> Self {
        loop {}
    }
}
/// There is no corresponding `impl Sum for Rgba` because the alpha would
/// not have a universally reasonable interpretation.
impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        loop {}
    }
}
impl fmt::Debug for Rgb {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Debug for Rgba {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for Rgb {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for Rgba {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
#[inline]
fn component_to_srgb(c: NotNan<f32>) -> f32 {
    loop {}
}
#[inline]
fn component_to_srgb8(c: NotNan<f32>) -> u8 {
    loop {}
}
#[inline]
const fn component_from_linear8_const(c: u8) -> NotNan<f32> {
    loop {}
}
/// Implements sRGB decoding using the standard arithmetic.
/// Implements sRGB decoding using a lookup table.
#[inline]
const fn component_from_srgb8_const(c: u8) -> NotNan<f32> {
    loop {}
}
/// Reduces alpha/opacity values to only three possibilities, by conflating all alphas
/// greater than zero and less than one.
///
/// This may be used in rendering algorithms to refer to whether something moved from
/// one category to another, and hence might need different treatment than in the previous
/// frame.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[allow(clippy::exhaustive_enums)]
#[repr(u8)]
pub(crate) enum OpacityCategory {
    /// Alpha of zero; completely transparent; completely invisible; need not be drawn.
    Invisible = 0,
    /// Alpha greater than zero and less than one; requires blending.
    Partial = 1,
    /// Alpha of one; completely hides what is behind it and does not require blending.
    Opaque = 2,
}
