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
impl<T: fmt::Debug + 'static> fmt::Debug for URef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// `URef`s are compared by pointer equality: they are equal only if they refer to
/// the same mutable cell.
impl<T> PartialEq for URef<T> {
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
/// `URef`s are compared by pointer equality.
impl<T> Eq for URef<T> {}
impl<T> hash::Hash for URef<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        loop {}
    }
}
impl<T> Clone for URef<T> {
    /// Cloning a [`URef`] clones the reference only.
    fn clone(&self) -> Self {
        loop {}
    }
}
/// Errors resulting from attempting to borrow/dereference a [`URef`].
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub(crate) enum RefError {
    /// Target was deleted, or its entire universe was dropped.
    #[error("object was deleted: {0}")]
    Gone(Name),
    /// Target is currently incompatibly borrowed.
    #[error("object was in use at the same time: {0}")]
    InUse(Name),
}
/// Parallel to [`UBorrowImpl`], but for mutable access.
///
/// This type is not exposed publicly, but only used in transactions to allow
/// the check-then-commit pattern; use [`URef::try_modify`] instead for other
/// purposes.
#[self_referencing]
#[derive(Debug)]
pub(crate) struct UBorrowMutImpl<T: 'static> {
    strong: StrongEntryRef<T>,
    #[borrows(strong)]
    #[not_covariant]
    guard: RwLockWriteGuard<'this, UEntry<T>>,
}
/// The data of an entry in a `Universe`.
#[derive(Debug)]
struct UEntry<T> {
    data: T,
}
/// The unique reference to an entry in a [`Universe`] from that `Universe`.
/// Normal usage is via `URef` instead.
///
/// This is essentially a strong-reference version of [`URef`] (which is weak).
#[derive(Debug)]
pub(super) struct URootRef<T> {
    state: Arc<Mutex<State<T>>>,
}
/// Object-safe trait implemented for [`URef`], to allow code to operate on `URef<T>`
/// regardless of `T`.
pub(crate) trait URefErased: core::any::Any {
    /// Same as [`URef::name()`].
    fn name(&self) -> Name;
    /// Same as [`URef::universe_id()`].
    fn universe_id(&self) -> Option<UniverseId>;
}
