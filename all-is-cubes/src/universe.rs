//! [`Universe`], the top-level game-world container.
//!
//! ## Thread-safety
//!
//! [`Universe`], [`URef`], and their contents implement [`Send`] and [`Sync`],
//! such that it is possible to access a universe from multiple threads.
//! However, they do not (currently) provide any ability to wait to obtain a lock,
//! because there is not yet a policy about how one would avoid the possibility of
//! deadlock. Instead, you may only _attempt_ to acquire a lock, receiving an error if it
//! is already held. (TODO: Improve this.)
//!
//! For the time being, if you wish to use a [`Universe`] from multiple threads, you must
//! bring your own synchronization mechanisms to ensure that readers and writers do not
//! run at the same time.
use std::fmt;
mod members;

mod universe_txn;
pub(crate) use universe_txn::*;
mod uref;
pub(crate) use uref::*;
mod visit;
pub(crate) use visit::*;
/// Name/key of an object in a [`Universe`].
///
/// Internally uses [`Arc`] to be cheap to clone. Might be interned in future versions.
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub(crate) enum Name {
    /// An explicitly set name.
    Specific(),
    /// An automatically assigned name.
    Anonym(),
    /// Not yet been assigned a name; this may be replaced with `Anonym` but not `Specific`.
    Pending,
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Copiable unique (within this process) identifier for a [`Universe`].
///
/// Used to check whether [`URef`]s belong to particular [`Universe`]s.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) struct UniverseId();
/// A collection of named objects which can refer to each other via [`URef`]. In the
/// future, it will enable garbage collection and inter-object invariants.
///
/// See also the [`UniverseIndex`] trait for methods for adding and removing objects.
///
/// **Thread-safety caveat:** See the documentation on [avoiding deadlock].
///
/// [avoiding deadlock]: crate::universe#thread-safety
pub(crate) struct Universe {}
/// Errors resulting from attempting to insert an object in a [`Universe`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct InsertError {}
