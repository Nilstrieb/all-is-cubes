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
