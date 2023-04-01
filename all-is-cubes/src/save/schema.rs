//! Data types which represent game state in formats explicitly designed for
//! serialization, and versioned to ensure ability to deserialize older data.
//!
//! As a general rule, all types in this file should avoid referring to types outside
//! this file, except where specifically intended. This ensures that changes to internal
//! representations will not accidentally leak to the serialization/save-game format via
//! `#[derive(Serialize, Deserialize)]`.
//!
//! An additional purpose of keeping all such types in this file is so that they can be
//! reviewed together to comprehend the formats.
//!
//! General properties of the serialization schema:
//!
//! * 3D vectors/points are represented as 3-element arrays
//!   (and not, say, as structures with named fields).
use serde::{Deserialize, Serialize};
use crate::math::{Face6, GridRotation};
use crate::universe::URef;
use crate::{block, space, universe};
fn return_true() -> bool {
    loop {}
}
fn is_true(value: &bool) -> bool {
    loop {}
}
fn is_default<T: Default + PartialEq + Copy>(value: &T) -> bool {
    loop {}
}
