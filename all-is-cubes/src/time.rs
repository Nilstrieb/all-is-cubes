//! Time passing “in game”, i.e. in a [`Universe`] and its contents.
//!
//! [`Universe`]: crate::universe::Universe
pub(crate) use instant::Duration;
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
