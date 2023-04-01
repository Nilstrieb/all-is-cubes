//! Mathematical utilities and decisions.

#[macro_use]
mod color;

mod coord;
pub(crate) use coord::*;
mod face;

mod grid_aab;
pub(crate) use grid_aab::*;
mod matrix;

mod rotation;
