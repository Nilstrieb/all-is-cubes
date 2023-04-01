//! Time passing “in game”, i.e. in a [`Universe`] and its contents.
//!
//! [`Universe`]: crate::universe::Universe
pub(crate) use instant::Duration;
/// Specifies an amount of time passing in a [`Universe`](crate::universe::Universe)
/// and its contents.
///
/// [`Universe`]: crate::universe::Universe
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Tick {}
