//! Definition of blocks, which are the game objects which occupy the grid of a
//! [`Space`]. See [`Block`] for details.
//!
//! The types of most interest in this module are [`Block`], [`Primitive`],
//! [`BlockAttributes`], and [`Modifier`].
use std::fmt;


mod attributes;

mod block_def;
pub(crate) use block_def::*;
mod evaluated;
pub(crate) use evaluated::*;
mod modifier;

mod resolution;
pub(crate) use resolution::*;
/// A [`Block`] is something that can exist in the grid of a [`Space`]; it occupies one
/// unit cube of simulated physical space, and has a specified appearance and behavior.
///
/// A [`Block`] is made up of a [`Primitive`] and zero or more [`Modifier`]s.
///
/// In general, when a block appears multiple times from an in-game perspective, that may
/// or may not be the the same copy; `Block`s are "by value" and any block [`Eq`] to
/// another will behave identically and should be treated identically. However, some
/// blocks are defined by reference to shared mutable data, and [`Block`] containers such
/// as [`Space`] must follow those changes.
///
/// To obtain the concrete appearance and behavior of a block, use [`Block::evaluate()`]
/// to obtain an [`EvaluatedBlock`] value, preferably with caching.
/// Use [`Block::listen()`] to be informed of possible changes to the result of
/// evaluation.
#[derive(Clone)]
pub(crate) struct Block();
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
impl Eq for Block {}
impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        loop {}
    }
}
/// Notification when an [`EvaluatedBlock`] result changes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct BlockChange {}
