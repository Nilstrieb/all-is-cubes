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
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::math::{Face6, GridAab, GridCoordinate, GridRotation};
use crate::universe::URef;
use crate::{block, space, universe};
/// Placeholder type for when we want to serialize the *contents* of a `URef`,
/// without cloning or referencing those contents immediately.
pub(crate) struct SerializeRef<T>(pub(crate) URef<T>);
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum BlockSer {
    BlockV1 {
        primitive: PrimitiveSer,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        modifiers: Vec<ModifierSer>,
    },
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum PrimitiveSer {
    AirV1,
    AtomV1 { color: RgbaSer, #[serde(flatten)] attributes: BlockAttributesV1Ser },
    RecurV1 {
        #[serde(flatten)]
        attributes: BlockAttributesV1Ser,
        space: universe::URef<space::Space>,
        #[serde(default, skip_serializing_if = "is_default")]
        offset: [i32; 3],
        resolution: block::Resolution,
    },
    IndirectV1 { definition: universe::URef<block::BlockDef> },
}
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BlockAttributesV1Ser {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub(crate) display_name: String,
    #[serde(default = "return_true", skip_serializing_if = "is_true")]
    pub(crate) selectable: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub(crate) light_emission: RgbSer,
}
fn return_true() -> bool {
    loop {}
}
fn is_true(value: &bool) -> bool {
    loop {}
}
fn is_default<T: Default + PartialEq + Copy>(value: &T) -> bool {
    loop {}
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum ModifierSer {
    QuoteV1 { suppress_ambient: bool },
    RotateV1 { rotation: GridRotation },
    CompositeV1 {
        source: block::Block,
        operator: block::CompositeOperator,
        reverse: bool,
        disassemblable: bool,
    },
    ZoomV1 { scale: block::Resolution, offset: [u8; 3] },
    MoveV1 { direction: Face6, distance: u16, velocity: i16 },
}
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct GridAabSer {
    pub(crate) lower: [GridCoordinate; 3],
    pub(crate) upper: [GridCoordinate; 3],
}
type RgbSer = [ordered_float::NotNan<f32>; 3];
type RgbaSer = [ordered_float::NotNan<f32>; 4];
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum SpaceSer {
    SpaceV1 {
        bounds: GridAab,
        blocks: Vec<block::Block>,
        contents: Box<[space::BlockIndex]>,
    },
}
/// Schema for `Universe` serialization and deserialization.
/// The type parameters allow for the different data types wanted in the serialization
/// case vs. the deserialization case.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum UniverseSchema<S> {
    UniverseV1 {
        /// Note: We are currently targeting JSON output, which cannot use non-string keys.
        /// Therefore, this is not expressed as a map.
        members: Vec<MemberEntrySer<MemberSchema<S>>>,
    },
}
pub(crate) type UniverseSer = UniverseSchema<SerializeRef<space::Space>>;
pub(crate) type UniverseDe = UniverseSchema<space::Space>;
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MemberEntrySer<T> {
    pub name: universe::Name,
    pub value: T,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum MemberSchema<S> {
    BlockDef(block::Block),
    Space(S),
}
pub(crate) type MemberSer = MemberSchema<SerializeRef<space::Space>>;
pub(crate) type MemberDe = MemberSchema<space::Space>;
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum URefSer {
    URefV1 { #[serde(flatten)] name: universe::Name },
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub(crate) enum NameSer {
    Specific(Arc<str>),
    Anonym(usize),
}
