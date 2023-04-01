use crate::block::{Block, BlockChange};
use crate::listen::{Listen, Listener};
use crate::transaction::{self, Transaction};
use crate::universe::{RefVisitor, VisitRefs};
use std::ops::Deref;
/// Contains a [`Block`] and can be stored in a [`Universe`](crate::universe::Universe).
/// Together with [`Primitive::Indirect`], this allows mutation of a block definition such
/// that all its usages follow.
///
/// It is a distinct type from [`Block`] in order to ensure that change notifications
/// will be delivered on any mutation.
///
/// To perform a mutation, use [`BlockDefTransaction`].
#[derive(Debug)]
pub struct BlockDef {}
