//! Storing and accessing definitions of standard blocks in a [`Universe`].
//!
//! An enum implementing [`BlockModule`] defines a set of names, and
//! [`BlockProvider`] assists in ensuring that all of those names are defined
//! and storing or retrieving their block values in a specific [`Universe`].
//!
//! In the future this mechanism may grow to become a dynamic linker/dependency injector
//! by becoming aware of dependencies between “modules”. For now, it's just enough to
//! solve bootstrapping needs.
use std::collections::HashMap;

use std::fmt;
use std::hash::Hash;

use exhaust::Exhaust;
use crate::block::Block;


use crate::universe::{Name};

fn name_in_module<E: BlockModule>(key: &E) -> Name {
    loop {}
}
/// Types whose values identify blocks in a set of related blocks, which may be
/// stored in a [`BlockProvider`] or under specific names in a [`Universe`].
///
/// The names of the [`Universe`]'s corresponding [`BlockDef`]s are formed by
/// combining the [`namespace()`](Self::namespace) and `self.to_string()` (the
/// [`Display`](fmt::Display) trait implementation).
///
/// Implement this trait for an enum, then use the functions of
/// [`BlockProvider`] to work with the described set of blocks.
///
/// TODO: consider replacing Display with a separate method so as not to presume its meaning
pub(crate) trait BlockModule: Exhaust + fmt::Debug + fmt::Display + Eq + Hash + Clone {
    /// A namespace for the members of this module; currently, this should be a
    /// `/`-separated path with no trailing slash, but (TODO:) we should have a
    /// more rigorous namespace scheme for [`Name`]s in future versions.
    fn namespace() -> &'static str;
}
/// TODO: document
#[derive(Clone, Debug)]
pub(crate) struct BlockProvider<E> {
    /// Guaranteed to contain an entry for every variant of `E` if `E`'s
    /// [`Exhaust`] implementation is accurate.
    map: HashMap<E, Block>,
}
