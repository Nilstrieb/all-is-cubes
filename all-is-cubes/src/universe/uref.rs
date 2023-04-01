use std::fmt;
use std::hash;
use std::sync::Mutex;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use ouroboros::self_referencing;
use crate::universe::{Name, UniverseId};
/// Type of a strong reference to an entry in a [`Universe`]. Defined to make types
/// parameterized with this somewhat less hairy.
type StrongEntryRef<T> = Arc<RwLock<UEntry<T>>>;
/// A reference from an object in a [`Universe`] to another.
///
/// If they are held by objects outside of the [`Universe`], it is not guaranteed
/// that they will remain valid (in which case trying to use the `URef` to read or write
/// the object will return an error).
///
/// **Thread-safety caveat:** See the documentation on [avoiding deadlock].
///
/// [avoiding deadlock]: crate::universe#thread-safety
pub struct URef<T> {
    state: Arc<Mutex<State<T>>>,
}
/// Strongly-referenced mutable state shared by all clones of a [`URef`].
/// This is modified by operations such as inserting into a [`Universe`].
#[derive(Debug)]
enum State<T> {
    /// Not yet (or never will be) inserted into a [`Universe`].
    ///
    /// May transition to [`State::Member`].
    Pending {
        /// Contains a strong reference to the same target as [`URef::weak_ref`].
        /// This is used to allow constructing `URef`s with targets *before* they are
        /// inserted into a [`Universe`], and thus inserting entire trees into the
        /// Universe. Upon that insertion, these strong references are dropped by
        /// changing the state.
        strong: StrongEntryRef<T>,
    },
    /// In a [`Universe`] (or has been deleted from one).
    Member {},
    /// State of [`URef::new_gone()`].
    Gone {},
}
/// The data of an entry in a `Universe`.
#[derive(Debug)]
struct UEntry<T> {
    data: T,
}
/// Object-safe trait implemented for [`URef`], to allow code to operate on `URef<T>`
/// regardless of `T`.
pub(crate) trait URefErased: core::any::Any {
    /// Same as [`URef::name()`].
    fn name(&self) -> Name;
    /// Same as [`URef::universe_id()`].
    fn universe_id(&self) -> Option<UniverseId>;
}
