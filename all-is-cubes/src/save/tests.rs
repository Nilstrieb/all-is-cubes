//! Tests of serialization and deserialization.
use std::fmt;
use pretty_assertions::assert_eq;
use serde_json::{from_value, json, to_value};
use crate::block::{self, Block, BlockDef, Modifier, Resolution};
use crate::character::Character;
use crate::content::make_some_blocks;
use crate::math::{GridAab, GridRotation, Rgb, Rgba};
use crate::space::Space;
use crate::universe::{Name, URef, Universe, UniverseIndex};
#[track_caller]
/// Serialize and deserialize and assert the value is equal.
fn assert_round_trip_value<T>(value: &T, expected_json: serde_json::Value)
where
    T: PartialEq + fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    loop {}
}
#[track_caller]
/// Deserialize and serialize and assert the JSON is equal.
fn assert_round_trip_json<T>(json: serde_json::Value)
where
    T: fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    loop {}
}
/// Serialize the value, then deserialize it and serialize that to confirm the JSON is
/// equal.
///
/// This is useful in lieu of [`assert_round_trip_value`] for when the values are
/// necessarily unequal (anything involving [`URef`]s).
#[track_caller]
fn assert_serdeser<T>(value: &T, expected_json: serde_json::Value)
where
    T: fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    loop {}
}
#[test]
fn block_air() {
    loop {}
}
#[test]
fn block_atom_default() {
    loop {}
}
#[test]
fn block_atom_with_all_attributes() {
    loop {}
}
#[test]
fn block_with_modifiers() {
    loop {}
}
#[test]
fn space() {
    loop {}
}
#[test]
fn universe_with_one_of_each() {
    loop {}
}
#[test]
fn uref_de_named() {
    loop {}
}
#[test]
fn uref_de_anon() {
    loop {}
}
#[test]
fn uref_ser_named() {
    loop {}
}
#[test]
fn uref_ser_anon() {
    loop {}
}
