use std::fmt;
use std::sync::{Arc, Weak};
use crate::listen::{Listener, Notifier};
/// A [`Listener`] which transforms or discards messages before passing them on.
/// Construct this using [`Listener::filter`].
///
/// This may be used to drop uninteresting messages or reduce their granularity.
///
/// TODO: add doc test
#[derive(Debug)]
pub struct Filter<F, T> {
    /// The function to transform and possibly discard each message.
    pub(super) function: F,
    /// The recipient of the messages.
    pub(super) target: T,
}
impl<MI, MO, F, T> Listener<MI> for Filter<F, T>
where
    F: Fn(MI) -> Option<MO> + Send + Sync,
    T: Listener<MO>,
{
    fn receive(&self, message: MI) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
/// Controls a [`Listener`] chain by discarding messages when this gate is dropped.
///
/// Construct this using [`Listener::gate`], or if a placeholder instance with no
/// effect is required, [`Gate::default`].
#[derive(Clone, Default)]
pub struct Gate(Arc<()>);
impl fmt::Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Gate {
    pub(super) fn new<L>(listener: L) -> (Gate, GateListener<L>) {
        loop {}
    }
}
/// [`Listener`] implementation which discards messages when the corresponding [`Gate`]
/// is dropped. Construct this using [`Listener::gate()`].
#[derive(Clone, Debug)]
pub struct GateListener<T> {
    weak: Weak<()>,
    target: T,
}
impl<M, T> Listener<M> for GateListener<T>
where
    T: Listener<M>,
{
    fn receive(&self, message: M) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
/// A [`Listener`] which forwards messages through a [`Notifier`] to its listeners.
/// Constructed by [`Notifier::forwarder()`].
#[derive(Debug)]
pub struct NotifierForwarder<M>(pub(super) Weak<Notifier<M>>);
impl<M: Clone + Send> Listener<M> for NotifierForwarder<M> {
    fn receive(&self, message: M) {
        loop {}
    }
    fn alive(&self) -> bool {
        loop {}
    }
}
impl<M> Clone for NotifierForwarder<M> {
    fn clone(&self) -> Self {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::{Listen as _, Sink};
    #[test]
    fn filter() {
        loop {}
    }
    #[test]
    fn gate() {
        loop {}
    }
}
