use std::sync::Arc;
use cgmath::{Angle as _, Deg, Point3, Vector3};
use crate::block::{Block, AIR};
use crate::character::{
    cursor_raycast, Character, CharacterChange, CharacterTransaction, Spawn,
};
use crate::inv::{InventoryChange, InventoryTransaction, Slot, Tool, ToolError};
use crate::listen::{Listen as _, Sink};
use crate::math::{Aab, Face6, GridAab, Rgb};
use crate::physics::BodyTransaction;
use crate::raycast::Ray;
use crate::space::Space;
use crate::time::Tick;
use crate::transaction::{self, Transaction as _, TransactionTester};
use crate::universe::Universe;
fn test_spawn(f: impl Fn(&mut Space) -> Spawn) -> Character {
    loop {}
}
#[test]
fn spawn_inferred_position() {
    loop {}
}
#[test]
fn spawn_inventory() {
    loop {}
}
#[test]
fn spawn_look_direction_default() {
    loop {}
}
#[test]
fn spawn_look_direction() {
    loop {}
}
#[test]
fn inventory_transaction() {
    loop {}
}
#[test]
fn transaction_systematic() {
    loop {}
}
#[test]
fn no_superjumping() {
    loop {}
}
#[test]
fn click_wrong_space_or_correct_space() {
    loop {}
}
