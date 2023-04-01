//! [`Universe`], the top-level game-world container.
//!
//! ## Thread-safety
//!
//! [`Universe`], [`URef`], and their contents implement [`Send`] and [`Sync`],
//! such that it is possible to access a universe from multiple threads.
//! However, they do not (currently) provide any ability to wait to obtain a lock,
//! because there is not yet a policy about how one would avoid the possibility of
//! deadlock. Instead, you may only _attempt_ to acquire a lock, receiving an error if it
//! is already held. (TODO: Improve this.)
//!
//! For the time being, if you wish to use a [`Universe`] from multiple threads, you must
//! bring your own synchronization mechanisms to ensure that readers and writers do not
//! run at the same time.
pub(crate) use universe_txn::*;
pub(crate) use uref::*;
pub(crate) use visit::*;
