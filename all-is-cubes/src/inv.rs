//! Characters' [`Inventory`] which contains [`Tool`]s for modifying the world.
//!
//! TODO: This module needs a better name; I'd be calling it `inventory` if that weren't
//! also the name of one of its internal modules.
mod icons;
pub(crate) use icons::*;
mod inventory;
pub(crate) use inventory::*;
mod tool;
pub(crate) use tool::*;
