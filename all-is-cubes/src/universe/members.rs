use std::collections::BTreeMap;
use crate::block::BlockDef;
use crate::character::Character;
use crate::space::Space;
use crate::universe::{
    InsertError, Name, URef, URootRef, Universe, UniverseIndex, UniverseIter,
};
/// A `BTreeMap` is used to ensure that the iteration order is deterministic across
/// runs/versions.
pub(super) type Storage<T> = BTreeMap<Name, URootRef<T>>;
/// Trait for every type which can be a named member of a universe.
/// This trait is also public-in-private and serves to “seal” the [`UniverseIndex`]
/// trait.
pub trait UniverseMember: Sized + 'static {
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
fn index_get<T>(this: &Universe, name: &Name) -> Option<URef<T>>
where
    Universe: UniverseTable<T>,
{
    loop {}
}
/// Implementation of inserting an item in a universe.
/// Note that the same logic also exists in `UniverseTransaction`.
fn index_insert<T>(
    this: &mut Universe,
    mut name: Name,
    value: T,
) -> Result<URef<T>, InsertError>
where
    Universe: UniverseTable<T>,
{
    loop {}
}
/// Generates impls for a specific Universe member type.
macro_rules! impl_universe_for_member {
    ($member_type:ident, $table:ident) => {
        impl UniverseMember for $member_type { fn into_any_ref(r : URef <$member_type >)
        -> AnyURef { AnyURef::$member_type (r) } } impl UniverseTable <$member_type > for
        Universe { fn table(& self) -> & Storage <$member_type > { & self.$table } fn
        table_mut(& mut self) -> & mut Storage <$member_type > { & mut self.$table } }
        impl UniverseIndex <$member_type > for Universe { fn get(& self, name : & Name)
        -> Option < URef <$member_type >> { index_get(self, name) } fn insert(& mut self,
        name : Name, value : $member_type,) -> Result < URef <$member_type >, InsertError
        > { index_insert(self, name, value) } fn iter_by_type(& self) -> UniverseIter
        <'_, $member_type > { UniverseIter(self.table().iter()) } }
    };
}
/// Generates enums which cover all universe types.
macro_rules! member_enums {
    ($(($member_type:ident),)*) => {
        #[doc = " Holds any one of the [`URef`] types that can be in a [`Universe`]."]
        #[doc = ""] #[doc =
        " See also [`URefErased`], which is implemented by `URef`s rather than owning one."]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)] #[doc(hidden)] pub enum AnyURef {
        $($member_type (URef <$member_type >),)* } impl AnyURef { pub (crate) fn
        check_upgrade_pending(& self, universe_id : $crate ::universe::UniverseId,) ->
        Result < (), $crate ::transaction::PreconditionFailed > { match self {
        $(Self::$member_type (r) => r.check_upgrade_pending(universe_id),)* } } fn
        as_erased(& self) -> & dyn $crate ::universe::URefErased { match self {
        $(Self::$member_type (r) => r as & dyn $crate ::universe::URefErased,)* } } }
    };
}
impl_universe_for_member!(BlockDef, blocks);
impl_universe_for_member!(Character, characters);
impl_universe_for_member!(Space, spaces);
member_enums!((BlockDef), (Character), (Space),);
