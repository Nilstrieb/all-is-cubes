use crate::block::BlockDef;
use crate::character::Character;
use crate::space::Space;
use crate::universe::{InsertError, Name, URef, URootRef, Universe, UniverseIndex, UniverseIter};
use std::collections::BTreeMap;
/// A `BTreeMap` is used to ensure that the iteration order is deterministic across
/// runs/versions.
pub(super) type Storage<T> = BTreeMap<Name, URootRef<T>>;
/// Trait for every type which can be a named member of a universe.
/// This trait is also public-in-private and serves to “seal” the [`UniverseIndex`]
/// trait.
pub(crate) trait UniverseMember: Sized + 'static {
    /// Generic constructor for [`AnyURef`].
    fn into_any_ref(r: URef<Self>) -> AnyURef;
}
/// Trait implemented once for each type of object that can be stored in a [`Universe`]
/// that internally provides the table for that type. This trait differs from
/// [`UniverseIndex`] in that it is not public.
pub(super) trait UniverseTable<T> {
    fn table(&self) -> &Storage<T>;
    fn table_mut(&mut self) -> &mut Storage<T>;
}

/// Generates enums which cover all universe types.
macro_rules! member_enums {
    ($(($member_type:ident),)*) => {
        #[doc = " Holds any one of the [`URef`] types that can be in a [`Universe`]."]
        #[doc = ""] #[doc =
        " See also [`URefErased`], which is implemented by `URef`s rather than owning one."]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)] #[doc(hidden)] pub enum AnyURef {
        $($member_type (URef <$member_type >),)* }
    };
}

member_enums!((BlockDef), (Character), (Space),);
