//! Definition of blocks, which are the game objects which occupy the grid of a
//! [`Space`]. See [`Block`] for details.
//!
//! The types of most interest in this module are [`Block`], [`Primitive`],
//! [`BlockAttributes`], and [`Modifier`].
use std::fmt;
pub(crate) use block_def::*;
pub(crate) use evaluated::*;
pub(crate) use resolution::*;
