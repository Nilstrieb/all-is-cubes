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

use std::sync::{Arc, RwLock};
mod util;
pub(crate) use util::*;
/// Ability to subscribe to a source of messages, causing a [`Listener`] to receive them
/// as long as it wishes to.
pub(crate) trait Listen {
    /// The type of message which may be obtained from this source.
    type Msg: Clone + Send;
    /// Subscribe the given [`Listener`] to this source of messages.
    ///
    /// Note that listeners are removed only via their returning false from
    /// [`Listener::alive()`]; there is no `unlisten()` operation, and identical listeners
    /// are not deduplicated.
    fn listen<L: Listener<Self::Msg> + Send + Sync + 'static>(&self, listener: L);
}
/// Mechanism for observing changes to objects. A [`Notifier`] delivers messages
/// of type `M` to a set of listeners, each of which usually holds a weak reference
/// to allow it to be removed when the actual recipient is gone or uninterested.
///
/// TODO: Currently, each message is [`Clone`]d for each recipient. This is fine for
/// most cases, but in some cases it would be cheaper to pass a reference. We could
/// make Notifier and Listener always take `&M`, but it's not clear how to use
/// references *some* of the time — making `M` be a reference type can't have a
/// satisfactory lifetime.
pub(crate) struct Notifier<M> {
    listeners: RwLock<Vec<DynListener<M>>>,
}
/// A receiver of messages (typically from something implementing [`Listen`]) which can
/// indicate when it is no longer interested in them (typically because the associated
/// recipient has been dropped).
///
/// Please note the requirements set out in [`Listener::receive()`].
///
/// Implementors should also implement [`Clone`] whenever possible; this allows
/// for a "listen" operation to be implemented in terms of delegating to several others.
/// This is not required, so that the `Listener` trait remains object-safe.
///
/// Implementors should also implement [`Send`] and [`Sync`], as most usage of listeners
/// might cross threads. However, this is not strictly required.
pub(crate) trait Listener<M> {
    /// Process and store a message.
    ///
    /// Note that, since this method takes `&Self`, a `Listener` must use interior
    /// mutability of some variety to store the message. As a `Listener` may be called
    /// from various contexts, and in particular while the sender is still performing
    /// its work, that mutability should in general be limited to setting dirty flags
    /// or inserting into message queues — not attempting to directly perform further
    /// game state changes, and particularly not taking any locks that are not solely
    /// used by the `Listener` and its destination, as that could result in deadlock.
    ///
    /// The typical pattern is for a listener to contain a `Weak<Mutex<...>>` or similar
    /// multiply-owned mutable structure to aggregate incoming messages, which will
    /// then be read and cleared by a later task; see [`FnListener`] for assistance in
    /// implementing this pattern.
    ///
    /// This method should not panic under any circumstances, or inconsistencies may
    /// result due to further work not being done and messages not being sent.
    fn receive(&self, message: M);
    /// Whether the [`Listener`]'s destination is still interested in receiving messages.
    ///
    /// This method should start returning [`false`] as soon as its destination is no
    /// longer interested in them or they would not have any effects on the rest of the
    /// system; this informs [`Notifier`]s that they should drop this listener and avoid
    /// memory leaks in the form of defunct listeners.
    ///
    /// This method should not panic under any circumstances, or inconsistencies may
    /// result due to further work not being done and messages not being sent.
    fn alive(&self) -> bool;
    /// Convert this listener into trait object form, allowing it to be stored in
    /// collections or passed non-generically.
    ///
    /// The purpose of this method over simply calling [`Arc::new()`] is that it will
    /// avoid double-wrapping of a listener that's already in [`Arc`]. **Other
    /// implementors should not override this.**
    fn erased(self) -> DynListener<M>
    where
        Self: Sized + Send + Sync + 'static,
    {
        loop {}
    }
    /// Apply a map/filter function to incoming messages.
    ///
    /// TODO: Doc test
    fn filter<MI, F>(self, function: F) -> Filter<F, Self>
    where
        Self: Sized,
        F: Fn(MI) -> Option<M> + Sync,
    {
        loop {}
    }
    /// Wraps `self` to pass messages only until the returned [`Gate`], and any clones
    /// of it, are dropped.
    ///
    /// This may be used to stop forwarding messages when a dependency no longer exists.
    ///
    /// ```
    /// use all_is_cubes::listen::{Listen, Listener, Gate, Sink};
    ///
    /// let sink = Sink::new();
    /// let (gate, gated) = sink.listener().gate();
    /// gated.receive("kept");
    /// assert!(sink.take_equal("kept"));
    /// drop(gate);
    /// gated.receive("discarded");
    /// assert!(!sink.take_equal("discarded"));
    /// ```
    fn gate(self) -> (Gate, GateListener<Self>)
    where
        Self: Sized,
    {
        loop {}
    }
}
/// Type-erased form of a [`Listener`] which accepts messages of type `M`.
pub(crate) type DynListener<M> = Arc<dyn Listener<M> + Send + Sync>;
