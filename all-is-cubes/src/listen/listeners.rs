use std::collections::VecDeque;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, Weak};
use crate::listen::{Listen, Listener};
/// A [`Listener`] which discards all messages and is suitable for filling
/// listener parameters when no listener is needed.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NullListener;
impl<M> Listener<M> for NullListener {
    fn receive(&self, _message: M) {}
    fn alive(&self) -> bool {
        loop {}
    }
}
/// A [`Listener`] which delivers messages by calling a function on a [`Weak`] reference's
/// referent, and stops when the weak reference breaks.
#[derive(Clone, Debug)]
pub struct FnListener<F, T> {
    function: F,
    weak_target: Weak<T>,
}
impl<F, T> FnListener<F, T> {
    #[allow(missing_docs)]
    pub fn new(target: &Arc<T>, function: F) -> Self {
        loop {}
    }
}
impl<M, F, T> Listener<M> for FnListener<F, T>
where
    F: Fn(&T, M),
{
    fn receive(&self, message: M) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
/// A [`Listener`] which stores all the messages it receives.
///
/// This is only intended for testing.
#[derive(Debug)]
pub struct Sink<M> {
    messages: Arc<RwLock<VecDeque<M>>>,
}
/// [`Sink::listener()`] implementation.
#[derive(Debug)]
pub struct SinkListener<M> {
    weak_messages: Weak<RwLock<VecDeque<M>>>,
}
impl<M> Sink<M> {
    /// Constructs a new empty [`Sink`].
    pub fn new() -> Self {
        loop {}
    }
    /// Returns a [`Listener`] which records the messages it receives in this Sink.
    pub fn listener(&self) -> SinkListener<M> {
        loop {}
    }
    /// If the given message was received, remove the first occurrence of it and return true.
    ///
    /// ```
    /// use all_is_cubes::listen::{Listener, Sink};
    ///
    /// let sink = Sink::new();
    /// sink.listener().receive(2);
    /// assert!(!sink.take_equal(1));  // No match
    /// assert!(sink.take_equal(2));   // Match
    /// assert!(!sink.take_equal(2));  // Now removed
    /// ```
    ///
    /// TODO: This is never used and therefore a candidate for removal.
    pub fn take_equal(&self, message: M) -> bool
    where
        M: Eq,
    {
        loop {}
    }
    /// Remove and return all messages returned so far.
    ///
    /// ```
    /// use all_is_cubes::listen::{Listener, Sink};
    ///
    /// let sink = Sink::new();
    /// sink.listener().receive(1);
    /// sink.listener().receive(2);
    /// assert_eq!(sink.drain(), vec![1, 2]);
    /// sink.listener().receive(3);
    /// assert_eq!(sink.drain(), vec![3]);
    /// ```
    pub fn drain(&self) -> Vec<M> {
        loop {}
    }
}
impl<M> Listener<M> for SinkListener<M> {
    fn receive(&self, message: M) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
impl<M> Clone for SinkListener<M> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<M> Default for Sink<M>
where
    M: Send + Sync,
{
    fn default() -> Self {
        loop {}
    }
}
/// A [`Listener`] destination which only stores a single flag indicating if any messages
/// were received.
pub struct DirtyFlag {
    flag: Arc<AtomicBool>,
}
impl fmt::Debug for DirtyFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// [`DirtyFlag::listener()`] implementation.
#[derive(Clone, Debug)]
pub struct DirtyFlagListener {
    weak_flag: Weak<AtomicBool>,
}
impl DirtyFlag {
    /// Constructs a new [`DirtyFlag`] with the given initial value.
    pub fn new(value: bool) -> Self {
        loop {}
    }
    /// Constructs a new [`DirtyFlag`] with the given initial value and call
    /// [`Listen::listen()`] with its listener.
    ///
    /// This is a convenience for calling `new()` followed by `listener()`.
    pub fn listening(value: bool, source: impl Listen) -> Self {
        loop {}
    }
    /// Returns a [`Listener`] which will set this flag to [`true`] when it receives any
    /// message.
    pub fn listener(&self) -> DirtyFlagListener {
        loop {}
    }
    /// Returns the flag value, setting it to [`false`] at the same time.
    pub fn get_and_clear(&self) -> bool {
        loop {}
    }
    /// Set the flag value to [`true`].
    ///
    /// Usually a [`DirtyFlagListener`] is used instead of this, but it may be useful
    /// in complex situations.
    pub fn set(&self) {
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::Notifier;
    #[test]
    fn null_alive() {
        loop {}
    }
    #[test]
    fn sink_alive() {
        loop {}
    }
    #[test]
    fn dirty_flag_alive() {
        loop {}
    }
    #[test]
    fn dirty_flag_debug() {
        loop {}
    }
}
