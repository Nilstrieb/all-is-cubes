//! [`Inventory`] for storing items.
use crate::transaction::{
    CommitError, Merge, PreconditionFailed, Transaction, TransactionConflict,
};
