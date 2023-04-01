//! Broadcasting of notifications of state changes, and other messages.
//!
//! Objects which wish to send notifications use [`Notifier`]s, which manage a collection
//! of [`Listener`]s. Each listener reports when it is no longer needed and may be
//! discarded.
//!
//! When [`Notifier::notify`] is called to send a message, it is synchronously delivered
//! to all listeners; therefore, listeners are obligated to avoid making further
//! significant state changes. The typical pattern is for a listener to contain a
//! `Weak<Mutex<...>>` or similar multiply-owned mutable structure to aggregate incoming
//! messages, which will then be read and cleared by a later task.
use std::sync::Arc;
mod util;
pub(crate) use util::*;

pub(crate) trait Listener<M> {
    fn receive(&self, message: M);

    fn alive(&self) -> bool;

    fn erased(self) -> DynListener<M>
    where
        Self: Sized + Send + Sync + 'static,
    {
        loop {}
    }

    fn filter<MI, F>(self, function: F) -> Filter<F, Self>
    where
        Self: Sized,
        F: Fn(MI) -> Option<M> + Sync,
    {
        loop {}
    }

    fn gate(self) -> (Gate, GateListener<Self>)
    where
        Self: Sized,
    {
        loop {}
    }
}

pub(crate) type DynListener<M> = Arc<dyn Listener<M> + Send + Sync>;
