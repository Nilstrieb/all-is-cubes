use std::sync::{Arc, Mutex};
use crate::listen::{Listen, Listener, Notifier};
/// A interior-mutable container for a value which can notify that the value changed,
/// and which has reference-counted read-only handles to read it.
#[derive(Debug)]
pub(crate) struct ListenableCell<T> {
    storage: Arc<ListenableCellStorage<T>>,
}
/// Access to a value that might change (provided by a [`ListenableCell`]) or be [a
/// constant](ListenableSource::constant), and which can be listened to.
#[derive(Clone, Debug)]
pub(crate) struct ListenableSource<T> {
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
