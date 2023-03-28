use std::borrow::Borrow;
use std::fmt;
use std::hash;
use std::ops::Deref;
use std::sync::Mutex;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
use ouroboros::self_referencing;
use crate::transaction::{ExecuteError, PreconditionFailed, Transaction, Transactional};
use crate::universe::InsertError;
use crate::universe::Universe;
use crate::universe::VisitRefs;
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
    /// Reference to the object. Weak because we don't want to create reference cycles;
    /// the assumption is that the overall game system will keep the [`Universe`] alive
    /// and that [`Universe`] will ensure no entry goes away while referenced.
    weak_ref: Weak<RwLock<UEntry<T>>>,
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
        /// Name that will apply once the ref is in a [`Universe`].
        ///
        /// * May be [`Name::Specific`].
        /// * May be [`Name::Pending`] to assign a [`Name::Anonym`] later.
        /// * May not be [`Name::Anonym`].
        name: Name,
        /// Contains a strong reference to the same target as [`URef::weak_ref`].
        /// This is used to allow constructing `URef`s with targets *before* they are
        /// inserted into a [`Universe`], and thus inserting entire trees into the
        /// Universe. Upon that insertion, these strong references are dropped by
        /// changing the state.
        strong: StrongEntryRef<T>,
    },
    /// In a [`Universe`] (or has been deleted from one).
    Member {
        /// Name of this member within the [`Universe`].
        ///
        /// * May be [`Name::Specific`].
        /// * May be [`Name::Anonym`].
        /// * May not be [`Name::Pending`].
        name: Name,
        /// ID of the universe this ref belongs to.
        ///
        /// None or not yet inserted into a universe.
        universe_id: UniverseId,
    },
    /// State of [`URef::new_gone()`].
    Gone { name: Name },
}
impl<T: 'static> URef<T> {
    /// Constructs a new [`URef`] that is not yet associated with any [`Universe`],
    /// and strongly references its value (until inserted into a universe).
    ///
    /// This may be used to construct subtrees that are later inserted into a
    /// [`Universe`]. Caution: creating cyclic structure and never inserting it
    /// will result in a memory leak.
    ///
    /// Note that specifying a [`Name::Anonym`] will create a `URef` which cannot actually
    /// be inserted into another [`Universe`], even if the specified number is free.
    ///
    /// TODO: Actually inserting these into a [`Universe`] is not yet implemented.
    pub(crate) fn new_pending(name: Name, initial_value: T) -> Self {
        loop {}
    }
    /// Constructs a [`URef`] that does not refer to a value, as if it used to but
    /// is now defunct.
    ///
    /// When dereferenced, this will always produce the error [`RefError::Gone`].
    /// When compared, this will be equal only to clones of itself.
    ///
    /// This may be used in tests to exercise error handling.
    #[doc(hidden)]
    pub(crate) fn new_gone(name: Name) -> URef<T> {
        loop {}
    }
    /// Name by which the [`Universe`] knows this ref.
    ///
    /// This may change from [`Name::Pending`] to another name when the ref is inserted into
    /// a [`Universe`].
    pub(crate) fn name(&self) -> Name {
        loop {}
    }
    /// Returns the unique ID of the universe this reference belongs to.
    ///
    /// This may be used to confirm that two [`URef`]s belong to the same universe.
    ///
    /// Returns [`None`] if this [`URef`] is not yet associated with a universe, or if
    ///  if it was created by [`Self::new_gone()`].
    pub(crate) fn universe_id(&self) -> Option<UniverseId> {
        loop {}
    }
    /// Acquire temporary read access the value, in the sense of [`RwLock::try_read()`].
    ///
    /// TODO: There is not currently any way to block on / wait for read access.
    pub(crate) fn read(&self) -> Result<UBorrow<T>, RefError> {
        loop {}
    }
    /// Apply the given function to the `&mut T` inside.
    ///
    /// **Warning:** Misusing this operation can disrupt connections between objects in
    /// the [`Universe`]; prefer [`URef::execute()`] if the desired mutation can be
    /// expressed as a [`Transaction`]. If you must use this, the requirement for
    /// correctness is that you must not replace the referent with a different value;
    /// only use the mutation operations provided by `T`.
    ///
    /// TODO: If possible, completely replace this operation with transactions.
    pub(crate) fn try_modify<F, Out>(&self, function: F) -> Result<Out, RefError>
    where
        F: FnOnce(&mut T) -> Out,
    {
        loop {}
    }
    /// Gain mutable access but don't use it immediately.
    ///
    /// This function is not exposed publicly, but only used in transactions to allow
    /// the check-then-commit pattern; use [`URef::try_modify`] instead for other
    /// purposes.
    pub(crate) fn try_borrow_mut(&self) -> Result<UBorrowMutImpl<T>, RefError> {
        loop {}
    }
    /// Execute the given transaction on the referent.
    ///
    /// Returns an error if the transaction's preconditions were not met, if the
    /// referent was already borrowed (which is denoted as an [`ExecuteError::Check`]),
    /// or if the transaction encountered an unexpected error.
    pub(crate) fn execute(
        &self,
        transaction: &<T as Transactional>::Transaction,
        outputs: &mut dyn FnMut(
            <<T as Transactional>::Transaction as Transaction<T>>::Output,
        ),
    ) -> Result<(), ExecuteError>
    where
        T: Transactional,
    {
        loop {}
    }
    fn upgrade(&self) -> Result<StrongEntryRef<T>, RefError> {
        loop {}
    }
    /// Returns whether this [`URef`] does not yet belong to a universe and can start.
    /// doing so. Used by [`UniverseTransaction`].
    ///
    /// TODO: There's a TOCTTOU problem here. We should modify the state and return a
    /// ticket (that resets the state if dropped without use), so that other simultaneous
    /// attempted `upgrade_pending()`s cannot succeed.
    pub(in crate::universe) fn check_upgrade_pending(
        &self,
        future_universe_id: UniverseId,
    ) -> Result<(), PreconditionFailed>
    where
        T: VisitRefs,
    {
        loop {}
    }
    /// If this [`URef`] does not yet belong to a universe, create its association with one.
    pub(in crate::universe) fn upgrade_pending(
        &self,
        universe: &mut Universe,
    ) -> Result<URootRef<T>, InsertError> {
        loop {}
    }
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
#[cfg(feature = "arbitrary")]
impl<'a, T: arbitrary::Arbitrary<'a> + 'static> arbitrary::Arbitrary<'a> for URef<T> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// Errors resulting from attempting to borrow/dereference a [`URef`].
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum RefError {
    /// Target was deleted, or its entire universe was dropped.
    #[error("object was deleted: {0}")]
    Gone(Name),
    /// Target is currently incompatibly borrowed.
    #[error("object was in use at the same time: {0}")]
    InUse(Name),
}
/// A wrapper type for an immutably borrowed value from an [`URef`].
pub(crate) struct UBorrow<T: 'static>(UBorrowImpl<T>);
/// Implementation of [`UBorrow`], split out to hide all `self_referencing` details.
#[self_referencing]
struct UBorrowImpl<T: 'static> {
    strong: StrongEntryRef<T>,
    #[borrows(strong)]
    #[covariant]
    guard: RwLockReadGuard<'this, UEntry<T>>,
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
    strong_ref: StrongEntryRef<T>,
    state: Arc<Mutex<State<T>>>,
}
/// Object-safe trait implemented for [`URef`], to allow code to operate on `URef<T>`
/// regardless of `T`.
pub trait URefErased: core::any::Any {
    /// Same as [`URef::name()`].
    fn name(&self) -> Name;
    /// Same as [`URef::universe_id()`].
    fn universe_id(&self) -> Option<UniverseId>;
}
impl<T: 'static> URefErased for URef<T> {
    fn name(&self) -> Name {
        loop {}
    }
    fn universe_id(&self) -> Option<UniverseId> {
        loop {}
    }
}
