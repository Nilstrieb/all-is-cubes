//! Conversion between the types in [`super::schema`] and those used in
//! normal operation.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use super::schema;
mod block {
    use super::*;
    use crate::block::{Block, BlockAttributes, Modifier, Primitive};
    use schema::ModifierSer;
    impl Serialize for Block {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            loop {}
        }
    }
    impl<'de> Deserialize<'de> for Block {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            loop {}
        }
    }
    impl From<&Primitive> for schema::PrimitiveSer {
        fn from(value: &Primitive) -> Self {
            loop {}
        }
    }
    impl From<schema::PrimitiveSer> for Primitive {
        fn from(value: schema::PrimitiveSer) -> Self {
            loop {}
        }
    }
    impl From<&BlockAttributes> for schema::BlockAttributesV1Ser {
        fn from(value: &BlockAttributes) -> Self {
            loop {}
        }
    }
    impl From<schema::BlockAttributesV1Ser> for BlockAttributes {
        fn from(value: schema::BlockAttributesV1Ser) -> Self {
            loop {}
        }
    }
    impl From<&Modifier> for ModifierSer {
        fn from(value: &Modifier) -> Self {
            loop {}
        }
    }
    impl From<schema::ModifierSer> for Modifier {
        fn from(value: schema::ModifierSer) -> Self {
            loop {}
        }
    }
}
mod universe {
    use super::*;
    use crate::block::BlockDef;
    use crate::universe::{Name, URef, Universe};
    impl From<&BlockDef> for schema::MemberSer {
        fn from(block_def: &BlockDef) -> Self {
            loop {}
        }
    }
    impl Serialize for Universe {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            loop {}
        }
    }
    impl<'de> Deserialize<'de> for Universe {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            loop {}
        }
    }
    impl<T: 'static> Serialize for URef<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            loop {}
        }
    }
    impl<'de, T: 'static> Deserialize<'de> for URef<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            loop {}
        }
    }
    impl Serialize for Name {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            loop {}
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            loop {}
        }
    }
    impl<T: Serialize + 'static> Serialize for schema::SerializeRef<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            loop {}
        }
    }
}
