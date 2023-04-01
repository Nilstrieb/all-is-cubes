use std::collections::VecDeque;
use std::fmt;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock, Weak};
use crate::listen::{Listen, Listener};
/// A [`Listener`] destination which only stores a single flag indicating if any messages
/// were received.
pub(crate) struct DirtyFlag {
    flag: Arc<AtomicBool>,
}
impl fmt::Debug for DirtyFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// [`DirtyFlag::listener()`] implementation.
#[derive(Clone, Debug)]
pub(crate) struct DirtyFlagListener {
    weak_flag: Weak<AtomicBool>,
}
impl DirtyFlag {
    /// Constructs a new [`DirtyFlag`] with the given initial value.
    pub(crate) fn new(value: bool) -> Self {
        loop {}
    }
    /// Constructs a new [`DirtyFlag`] with the given initial value and call
    /// [`Listen::listen()`] with its listener.
    ///
    /// This is a convenience for calling `new()` followed by `listener()`.
    pub(crate) fn listening(value: bool, source: impl Listen) -> Self {
        loop {}
    }
    /// Returns a [`Listener`] which will set this flag to [`true`] when it receives any
    /// message.
    pub(crate) fn listener(&self) -> DirtyFlagListener {
        loop {}
    }
    /// Returns the flag value, setting it to [`false`] at the same time.
    pub(crate) fn get_and_clear(&self) -> bool {
        loop {}
    }
    /// Set the flag value to [`true`].
    ///
    /// Usually a [`DirtyFlagListener`] is used instead of this, but it may be useful
    /// in complex situations.
    pub(crate) fn set(&self) {
        loop {}
    }
}
impl<M> Listener<M> for DirtyFlagListener {
    fn receive(&self, _message: M) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
