//! Data structures for light storage and algorithms.
use std::fmt;
use cgmath::{Vector3, Vector4};
use crate::math::*;
/// One component of a `PackedLight`.
pub(crate) type PackedLightScalar = u8;
/// Special reasons for a cube having zero light in it.
/// These may be used to help compute smoothed lighting across blocks.
///
/// The numeric values of this enum are used to transmit it to shaders by packing
/// it into an "RGBA" color value. They should not be considered a stable API element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum LightStatus {
    /// The cube's light value has never been computed.
    #[allow(unused)]
    Uninitialized = 0,
    /// The cube has no surfaces to catch light and therefore the light value is not tracked.
    NoRays = 1,
    /// The cube contains an opaque block and therefore does not have any light entering.
    Opaque = 128,
    /// No special situation: if it's black then it's just dark.
    Visible = 255,
}
/// Lighting within a [`Space`]; an [`Rgb`] value stored with reduced precision and range.
///
/// TODO: This now stores additional information. Rename to '`SpaceLight`' or some such.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct PackedLight {
    value: Vector3<PackedLightScalar>,
    status: LightStatus,
}
/// A priority queue for [`LightUpdateRequest`]s which contains cubes
/// at most once, even when added with different priorities.
pub(crate) struct LightUpdateQueue {}
