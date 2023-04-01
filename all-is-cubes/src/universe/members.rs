use crate::block::BlockDef;
use crate::character::Character;
use crate::space::Space;
use crate::universe::{URef};

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
