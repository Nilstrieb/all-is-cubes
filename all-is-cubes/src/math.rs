//! Mathematical utilities and decisions.
use cgmath::Vector3;
pub(crate) use ordered_float::NotNan;
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
