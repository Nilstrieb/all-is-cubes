use std::any::TypeId;
use crate::block::{Block, BlockDef, BlockDefTransaction, Primitive, Resolution, AIR};
use crate::character::{Character, CharacterTransaction};
use crate::content::make_some_blocks;
use crate::inv::{InventoryTransaction, Tool};
use crate::math::Rgba;
use crate::space::Space;
use crate::time::Tick;
use crate::transaction::{self, Transaction};
use crate::universe::{
    list_refs, InsertError, InsertErrorKind, Name, RefError, URef, Universe,
    UniverseIndex, UniverseTransaction,
};
use crate::util::assert_send_sync;
#[test]
fn thread_safety() {
    loop {}
}
#[test]
fn universe_debug_empty() {
    loop {}
}
/// Universe does not print contents of members, on the assumption this would be too verbose.
#[test]
fn universe_debug_elements() {
    loop {}
}
#[test]
fn get_any() {
    loop {}
}
#[test]
fn insert_anonymous_makes_distinct_names() {
    loop {}
}
#[test]
fn insert_duplicate_name_same_type() {
    loop {}
}
#[test]
fn insert_duplicate_name_different_type() {
    loop {}
}
#[test]
fn insert_duplicate_name_via_txn() {
    loop {}
}
#[test]
fn insert_anonym_prohibited_direct() {
    loop {}
}
#[test]
fn insert_anonym_prohibited_via_txn() {
    loop {}
}
#[test]
fn insert_pending_becomes_anonym_direct() {
    loop {}
}
#[test]
fn insert_pending_becomes_anonym_via_txn() {
    loop {}
}
#[test]
fn delete_success() {
    loop {}
}
/// Anonymous members are strictly garbage collected, and cannot be deleted.
#[test]
fn delete_anonymous_fails() {
    loop {}
}
#[test]
fn delete_twice_fails() {
    loop {}
}
#[test]
fn delete_wrong_universe_fails() {
    loop {}
}
#[test]
fn gc_explicit() {
    loop {}
}
#[test]
fn gc_implicit() {
    loop {}
}
#[test]
fn visit_refs_block_def_no_ref() {
    loop {}
}
#[test]
fn visit_refs_block_def_space() {
    loop {}
}
#[test]
fn visit_refs_block_def_indirect() {
    loop {}
}
#[test]
fn visit_refs_character() {
    loop {}
}
#[test]
fn visit_refs_space() {
    loop {}
}
