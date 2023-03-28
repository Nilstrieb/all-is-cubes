//! Continuously moving objects and collision.
use crate::math::FreeCoordinate;
mod body;
pub(crate) use body::*;
mod collision;
pub(crate) use collision::*;
