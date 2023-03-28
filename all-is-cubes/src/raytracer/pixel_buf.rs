//! [`PixelBuf`] and output formats of the raytracer.
use std::convert::TryFrom as _;
use cgmath::{Vector3, Zero as _};
use crate::camera::GraphicsOptions;
use crate::math::Rgba;
use crate::space::SpaceBlockData;
