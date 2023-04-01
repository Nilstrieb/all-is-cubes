//! Built-in “game content”: basic shapes and colors used in the UI and tests.
//!
//! This module is private; the public interface to this stuff is the separate
//! `all-is-cubes-content` crate.
use crate::block::{Block, Resolution};
use crate::inv::Slot;
use crate::math::{GridCoordinate, Rgba};
use crate::space::{SetCubeError, Space};
use crate::universe::Universe;
