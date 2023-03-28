use std::sync::{Arc, Mutex};
use crate::listen::{Listen, Listener, Notifier};
/// A interior-mutable container for a value which can notify that the value changed,
/// and which has reference-counted read-only handles to read it.
#[derive(Debug)]
pub struct ListenableCell<T> {
    storage: Arc<ListenableCellStorage<T>>,
}
/// Access to a value that might change (provided by a [`ListenableCell`]) or be [a
/// constant](ListenableSource::constant), and which can be listened to.
#[derive(Clone, Debug)]
pub struct ListenableSource<T> {
    storage: Arc<ListenableCellStorage<T>>,
}
#[derive(Debug)]
struct ListenableCellStorage<T> {
    /// Mutex because it's mutable; Arc because we want to be able to clone out of it to
    /// avoid holding the cell borrowed.
    /// TODO: Look into strategies to make this cheaper?
    cell: Mutex<Arc<T>>,
    /// Notifier to track listeners.
    /// `None` if this is a constant cell.
    ///
    /// TODO: Add ability to diff the value and distribute that.
    /// TODO: If the ListenableCell is dropped, drop this.
    notifier: Option<Notifier<()>>,
}
impl<T: Sync> ListenableCell<T> {
    /// Creates a new [`ListenableCell`] containing the given value.
    pub fn new(value: impl Into<Arc<T>>) -> Self {
        loop {}
    }
    /// Returns a reference to the current value of the cell.
    pub fn get(&self) -> Arc<T> {
        loop {}
    }
    /// Sets the contained value and sends out a change notification.
    ///
    /// Note that this does not test whether the current value is equal to avoid redundant
    /// notifications.
    ///
    /// Caution: While listeners are *expected* not to have immediate side effects on
    /// notification, this cannot be enforced.
    pub fn set(&self, value: impl Into<Arc<T>>) {
        loop {}
    }
    /// Sets the contained value by modifying a clone of the old value using the provided
    /// function.
    ///
    /// Note: this function is not atomic, in that other modifications can be made between
    /// the time this function reads the current value and writes the new one.
    pub fn update_mut<F>(&self, f: F)
    where
        T: Clone,
        F: FnOnce(&mut T),
    {
        loop {}
    }
    /// Returns a [`ListenableSource`] which provides read-only access to the value
    /// managed by this cell.
    pub fn as_source(&self) -> ListenableSource<T> {
        loop {}
    }
}
impl<T: Clone + Sync> ListenableSource<T> {
    /// Creates a new [`ListenableSource`] containing the given value, which will
    /// never change.
    pub fn constant(value: T) -> Self {
        loop {}
    }
    /// Returns a reference to the current value of the cell.
    pub fn get(&self) -> Arc<T> {
        loop {}
    }
    /// Returns a clone of the current value of the cell.
    pub fn snapshot(&self) -> T {
        loop {}
    }
}
impl<T: Clone + Sync> Listen for ListenableSource<T> {
    type Msg = ();
    fn listen<L: Listener<Self::Msg> + Send + Sync + 'static>(&self, listener: L) {
        loop {}
    }
}
/// Convenience wrapper around [`ListenableCell`] which allows borrowing the current
/// value, at the cost of requiring `&mut` access to set it.
#[derive(Debug)]
#[doc(hidden)]
pub struct ListenableCellWithLocal<T> {
    cell: ListenableCell<T>,
    value: Arc<T>,
}
impl<T: Sync> ListenableCellWithLocal<T> {
    pub fn new(value: impl Into<Arc<T>>) -> Self {
        loop {}
    }
    pub fn set(&mut self, value: impl Into<Arc<T>>) {
        loop {}
    }
    #[allow(clippy::should_implement_trait)]
    pub fn borrow(&self) -> &T {
        loop {}
    }
    /// Returns a [`ListenableSource`] which provides read-only access to the value
    /// managed by this cell.
    pub fn as_source(&self) -> ListenableSource<T> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::Sink;
    #[test]
    fn listenable_cell() {
        loop {}
    }
    #[test]
    fn listenable_source_constant() {
        loop {}
    }
    #[test]
    fn listenable_source_clone() {
        loop {}
    }
}
