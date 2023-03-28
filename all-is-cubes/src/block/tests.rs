//! Tests for [`Block`] as a whole.
//! The following modules also have their own tests:
//!
//! * [`super::attributes`]
//! * [`super::builder`]
//! * [`super::modifier`]
#![allow(clippy::bool_assert_comparison)]
use std::borrow::Cow;
use cgmath::EuclideanSpace as _;
use pretty_assertions::assert_eq;
use crate::block::{
    Block, BlockAttributes, BlockCollision, BlockDef, BlockDefTransaction,
    EvalBlockError, Evoxel, Evoxels, Modifier, Primitive, Resolution, Resolution::*, AIR,
    AIR_EVALUATED,
};
use crate::content::make_some_blocks;
use crate::listen::{NullListener, Sink};
use crate::math::{
    Face6, FaceMap, GridAab, GridArray, GridCoordinate, GridPoint, GridRotation,
    GridVector, OpacityCategory, Rgb, Rgba,
};
use crate::space::{Space, SpaceTransaction};
use crate::transaction;
use crate::universe::Universe;
#[test]
fn block_is_approximately_a_pointer() {
    loop {}
}
#[test]
fn block_static_eq_to_non_static() {
    loop {}
}
#[test]
fn block_debug_air() {
    loop {}
}
#[test]
fn block_debug_with_modifiers() {
    loop {}
}
#[test]
fn evaluate_air_self_consistent() {
    loop {}
}
#[test]
fn evaluate_air_consistent_with_constant() {
    loop {}
}
/// TODO: For now, AIR is processed like an `Atom` block and has selectable and collision
/// fields that are the default. It would probably be better to arrange so that all
/// `Evoxel`s end up with the attributes they reasonably should have, but for now, for
/// consistency with the rest of the architecture (designed before _all_ blocks had
/// `Evoxel`s rather than just attributes and color), this test is expected to “fail”
/// (find unequal evoxels).
#[test]
#[should_panic = "unequal air"]
fn evaluate_air_vs_evoxel_air() {
    loop {}
}
#[test]
fn evaluate_opaque_atom_and_attributes() {
    loop {}
}
#[test]
fn evaluate_transparent_atom() {
    loop {}
}
#[test]
fn evaluate_invisible_atom() {
    loop {}
}
#[test]
fn evaluate_voxels_checked_individually() {
    loop {}
}
#[test]
fn evaluate_transparent_voxels() {
    loop {}
}
#[test]
fn evaluate_voxels_not_filling_block() {
    loop {}
}
/// Test the situation where the space is smaller than the block: in particular,
/// even if the space is all opaque, the block should not be counted as opaque.
#[test]
fn evaluate_voxels_partial_not_filling() {
    loop {}
}
/// Tests that the `offset` field of `Primitive::Recur` is respected.
#[test]
fn recur_with_offset() {
    loop {}
}
/// Fuzzer-derived regression test for numeric overflow
#[test]
fn recur_offset_negative_overflow() {
    loop {}
}
#[test]
fn indirect_equivalence() {
    loop {}
}
#[test]
fn listen_atom() {
    loop {}
}
#[test]
fn listen_indirect_atom() {
    loop {}
}
/// Testing double indirection not because it's a case we expect to use routinely,
/// but because it exercises the generality of the notification mechanism.
#[test]
fn listen_indirect_double() {
    loop {}
}
/// Test that changes to a `Space` propagate to block listeners.
#[test]
fn listen_recur() {
    loop {}
}
#[test]
fn overflow_evaluate() {
    loop {}
}
#[test]
fn overflow_listen() {
    loop {}
}
/// Helper for overflow_ tests
fn self_referential_block(universe: &mut Universe) -> Block {
    loop {}
}
mod txn {
    use super::*;
    use crate::block::BlockDefTransaction;
    use crate::transaction::{Merge, TransactionTester};
    use pretty_assertions::assert_eq;
    #[test]
    fn causes_notification() {
        loop {}
    }
    #[test]
    fn merge_allows_same_new() {
        loop {}
    }
    #[test]
    fn merge_rejects_different_new() {
        loop {}
    }
    #[test]
    fn merge_rejects_different_old() {
        loop {}
    }
    #[test]
    fn merge_allows_same_old() {
        loop {}
    }
    #[test]
    fn systematic() {
        loop {}
    }
}
