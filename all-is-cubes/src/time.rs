//! Time passing “in game”, i.e. in a [`Universe`] and its contents.
//!
//! [`Universe`]: crate::universe::Universe
pub(crate) use instant::{Duration, Instant};
/// Specifies an amount of time passing in a [`Universe`](crate::universe::Universe)
/// and its contents.
///
/// [`Universe`]: crate::universe::Universe
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Tick {
    pub(crate) delta_t: Duration,
    /// Whether game time is paused, and `delta_t` should not be considered
    /// as an amount of game time passing. See [`Self::paused()`] for details.
    paused: bool,
}
impl Tick {
    /// A tick of arbitrary length, for testing purposes. Do not use this for actual gameplay.
    pub(crate) const fn arbitrary() -> Self {
        loop {}
    }
    /// Construct a [`Tick`] of the specified length.
    pub(crate) const fn from_duration(delta_t: Duration) -> Self {
        loop {}
    }
    /// Construct a non-paused [`Tick`] from a duration expressed in fractional seconds.
    pub(crate) fn from_seconds(dt: f64) -> Self {
        loop {}
    }
    /// Return the amount of time passed as a [`Duration`].
    pub(crate) fn delta_t(self) -> Duration {
        loop {}
    }
    /// Set the paused flag. See [`Tick::paused`] for more information.
    #[must_use]
    pub(crate) fn pause(self) -> Self {
        loop {}
    }
    /// Returns the "paused" state of this Tick. If true, then step operations should
    /// not perform any changes that reflect "in-game" time passing. They should still
    /// take care of the side effects of other mutations/transactions, particularly where
    /// not doing so might lead to a stale or inconsistent view.
    ///
    /// Note that functions which propagate ticks to subordinate game objects are free to
    /// not propagate paused ticks. TODO: The exact policies are not yet settled.
    pub(crate) fn paused(&self) -> bool {
        loop {}
    }
}
