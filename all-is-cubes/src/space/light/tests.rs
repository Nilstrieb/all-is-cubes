//! Tests for the behavior of light in a [`Space`].
use pretty_assertions::assert_eq;
use super::{data::LightStatus, LightUpdatesInfo, PackedLight};
use crate::block::{AnimationHint, Block, AIR};
use crate::listen::{Listen as _, Listener, Sink};
use crate::math::{FaceMap, GridPoint, Rgb, Rgba};
use crate::space::{GridAab, LightPhysics, Space, SpaceChange, SpacePhysics};
use crate::time::Tick;
#[test]
fn initial_lighting_value() {
    loop {}
}
#[test]
fn out_of_bounds_lighting_value() {
    loop {}
}
#[test]
fn step() {
    loop {}
}
#[test]
fn evaluate_light() {
    loop {}
}
/// There's a special case for setting cubes to opaque. That case must do the usual
/// light update and notification.
#[test]
fn set_cube_opaque_notification() {
    loop {}
}
fn light_source_test_space(block: Block) -> Space {
    loop {}
}
#[test]
fn light_source_self_illumination_transparent() {
    loop {}
}
#[test]
fn light_source_self_illumination_opaque() {
    loop {}
}
/// Check that an animation hint causes a block and its neighbors to be lit even if
/// it isn't visible, to be prepared for changes.
#[test]
fn animation_treated_as_visible() {
    loop {}
}
#[test]
fn reflectance_is_clamped() {
    loop {}
}
/// Helper to construct a space with `LightPhysics` set to None
fn space_with_disabled_light() -> Space {
    loop {}
}
#[test]
fn disabled_lighting_returns_one_always() {
    loop {}
}
#[test]
fn disabled_lighting_does_not_update() {
    loop {}
}
