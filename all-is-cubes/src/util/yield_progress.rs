use std::fmt;
use std::panic::Location;
use std::sync::{Arc, Mutex};
use futures_core::future::{BoxFuture, Future};
use instant::{Duration, Instant};
/// Allows a long-running async task to report its progress, while also yielding to the
/// scheduler (e.g. for single-threaded web environment) and introducing cancellation
/// points.
///
/// These go together because the rate at which it makes sense to yield (to avoid event
/// loop hangs) is similar to the rate at which it makes sense to report progress.
///
/// Note that while a [`YieldProgress`] is `Send` and `Sync`, it does not currently
/// support meaningfully being used from multiple threads or futures at once — only
/// reporting the progress of, and yielding periodically within, a fully sequential
/// operation. This might change in the future, but for now, it will just output
/// inconsistent results if you try to use it otherwise.
pub struct YieldProgress {
    start: f32,
    end: f32,
    /// Name given to this specific portion of work. Inherited from the parent if not
    /// overridden.
    ///
    /// TODO: Eventually we will want to have things like "label this segment as a
    /// fallback if it has no better label", which will require some notion of distinguishing
    /// inheritance from having been explicitly set.
    label: Option<Arc<str>>,
    yielding: Arc<Yielding<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>>,
    #[allow(clippy::type_complexity)]
    progressor: Arc<dyn Fn(f32, &str) + Send + Sync>,
}
/// Piggyback on the `Arc` we need to store the `dyn Fn` anyway to also store some state.
struct Yielding<F: ?Sized> {
    state: Mutex<YieldState>,
    yielder: F,
}
#[derive(Clone)]
struct YieldState {
    /// The most recent instant at which `yielder`'s future completed.
    /// Used to detect overlong time periods between yields.
    last_finished_yielding: Instant,
    last_yield_location: &'static Location<'static>,
    last_yield_label: Option<Arc<str>>,
}
impl fmt::Debug for YieldProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl YieldProgress {
    /// Construct a new [`YieldProgress`], which will call `yielder` to yield and
    /// `progressor` to report progress.
    ///
    /// Note that it measures time intervals between yields starting from when this
    /// function is called, as if this is the first yield.
    #[track_caller]
    pub fn new<Y, YFut, P>(yielder: Y, progressor: P) -> Self
    where
        Y: Fn() -> YFut + Send + Sync + 'static,
        YFut: Future<Output = ()> + Send + 'static,
        P: Fn(f32, &str) + Send + Sync + 'static,
    {
        loop {}
    }
    /// Returns a [`YieldProgress`] that does no progress reporting and no yielding.
    pub fn noop() -> Self {
        loop {}
    }
    /// Add a name for the portion of work this [`YieldProgress`] covers.
    ///
    /// If there is already a label, it will be overwritten.
    pub fn set_label(&mut self, label: impl fmt::Display) {
        loop {}
    }
    /// Map a `0..=1` value to `self.start..=self.end`.
    #[track_caller]
    fn point_in_range(&self, mut x: f32) -> f32 {
        loop {}
    }
    /// Report the current amount of progress (a number from 0 to 1) and yield.
    #[track_caller]
    pub fn progress(
        &self,
        progress_fraction: f32,
    ) -> impl Future<Output = ()> + Send + 'static {
        let location = Location::caller();
        let label = self.label.clone();
        (self
            .progressor)(
            self.point_in_range(progress_fraction),
            self.label.as_ref().map_or("", |arc_str_ref| -> &str { arc_str_ref }),
        );
        self.yielding.clone().yield_only(location, label)
    }
    /// Report that 100% of progress has been made.
    ///
    /// This is identical to `.progress(1.0)` but consumes the `YieldProgress` object.
    #[track_caller]
    pub fn finish(self) -> impl Future<Output = ()> + Send + 'static {
        self.progress(1.0)
    }
    /// Report that the given amount of progress has been made, then return
    /// a [`YieldProgress`] covering the remaining range.
    #[track_caller]
    pub fn finish_and_cut(
        self,
        progress_fraction: f32,
    ) -> impl Future<Output = Self> + Send + 'static {
        let [a, b] = self.split(progress_fraction);
        let progress_future = a.finish();
        async move {
            progress_future.await;
            b
        }
    }
    fn with_new_range(&self, start: f32, end: f32) -> Self {
        loop {}
    }
    /// Construct two new [`YieldProgress`] which divide the progress value into two
    /// subranges.
    ///
    /// The returned instances should be used in sequence, but this is not enforced.
    pub fn split(self, cut: f32) -> [Self; 2] {
        loop {}
    }
    /// Split into even subdivisions.
    pub fn split_evenly(self, count: usize) -> impl Iterator<Item = YieldProgress> {
        assert!(count < usize::MAX);
        (0..count)
            .map(move |index| {
                self.with_new_range(
                    self.point_in_range(index as f32 / count as f32),
                    self.point_in_range((index + 1) as f32 / count as f32),
                )
            })
    }
}
impl<F: ?Sized + Fn() -> BoxFuture<'static, ()> + Send + Sync> Yielding<F> {
    async fn yield_only(
        self: Arc<Self>,
        location: &'static Location<'static>,
        label: Option<Arc<str>>,
    ) {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_send_sync;
    #[test]
    fn yield_progress_is_sync() {
        loop {}
    }
}
