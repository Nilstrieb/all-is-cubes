//! Tests for [`crate::space`].
//!
//! Note that some sub-modules have their own test modules.
use cgmath::EuclideanSpace as _;
use indoc::indoc;
use crate::block::{
    Block, BlockDef, BlockDefTransaction, EvalBlockError, Primitive, Resolution::*, AIR,
};
use crate::content::make_some_blocks;
use crate::drawing::VoxelBrush;
use crate::listen::{Listen as _, Sink};
use crate::math::{GridCoordinate, GridPoint, Rgba};
use crate::space::{
    GridAab, LightPhysics, PackedLight, SetCubeError, Space, SpaceChange, SpacePhysics,
};
use crate::time::Tick;
use crate::transaction;
use crate::universe::{Name, RefError, Universe, UniverseIndex as _, UniverseTransaction};
#[test]
fn initial_state_consistency() {
    loop {}
}
/// set() returns Ok when the cube was changed or already equal.
#[test]
fn set_success() {
    loop {}
}
#[test]
fn set_failure_out_of_bounds() {
    loop {}
}
/// This test case should also cover `RefError::Gone`.
#[test]
fn set_failure_borrow() {
    loop {}
}
#[test]
fn set_failure_too_many() {
    loop {}
}
#[test]
fn set_error_format() {
    loop {}
}
/// `EvaluatedBlock` data is updated when a new block index is allocated.
#[test]
fn set_updates_evaluated_on_added_block() {
    loop {}
}
/// `EvaluatedBlock` data is updated when a block index is reused.
#[test]
fn set_updates_evaluated_on_replaced_block() {
    loop {}
}
/// No arithmetic overflow when modifying a block at the numeric range upper bound.
#[test]
fn set_no_neighbor_overflow_high() {
    loop {}
}
/// No arithmetic overflow when modifying a block at the numeric range lower bound.
#[test]
fn set_no_neighbor_overflow_low() {
    loop {}
}
#[test]
fn removed_blocks_are_forgotten() {
    loop {}
}
#[test]
fn change_listener() {
    loop {}
}
#[test]
fn extract_out_of_bounds() {
    loop {}
}
#[test]
fn fill_out_of_bounds() {
    loop {}
}
/// Test filling an entire space with one block using [`Space::fill`].
#[test]
fn fill_entire_space() {
    loop {}
}
/// Test filling an entire space with one block using [`Space::fill_uniform`].
#[test]
fn fill_uniform_entire_space() {
    loop {}
}
/// There was a bug triggered when the last instance of a block was replaced with
/// a block already in the space. This specifically runs a consistency check in that
/// case.
#[test]
fn replace_last_block_regression() {
    loop {}
}
#[test]
fn listens_to_block_changes() {
    loop {}
}
#[test]
fn indirect_becomes_evaluation_error() {
    loop {}
}
#[test]
fn space_debug() {
    loop {}
}
#[test]
fn set_physics_light_none() {
    loop {}
}
#[test]
fn set_physics_light_rays() {
    loop {}
}
#[test]
fn block_tick_action() {
    loop {}
}
