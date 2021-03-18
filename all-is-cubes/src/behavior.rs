// Copyright 2020-2021 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

//! Dynamic add-ons to game objects; we might also have called them “components”.

use instant::Duration;
use std::cell::RefCell;
use std::fmt::Debug;

use crate::transactions::{Transaction, Transactional, UTransactional, UniverseTransaction};

/// TODO: explain
///
/// `H` is the type of the host object or the context it provides.
pub trait Behavior<H: Transactional>: Debug {
    /// Computes a transaction to apply the effects of this behavior for one timestep.
    ///
    /// TODO: Define what happens if the transaction fails.
    fn step(
        &mut self,
        _context: &BehaviorContext<'_, H>,
        _duration: Duration,
    ) -> UniverseTransaction {
        UniverseTransaction::default()
    }

    /// Returns [`false`] if the [`Behavior`] should be dropped because conditions under
    /// which it is useful no longer apply.
    fn alive(&self, context: &BehaviorContext<'_, H>) -> bool;

    /// Whether the behavior should never be persisted/saved to disk, because it will be
    /// reconstructed as needed (e.g. collision, occupancy, user interaction, particles).
    ///
    /// If a behavior changes its answer over its lifetime, which outcome will occur is
    /// unspecified.
    fn ephemeral(&self) -> bool;

    // TODO: serialization, quiescence, incoming events...
}

#[non_exhaustive]
pub struct BehaviorContext<'a, H: Transactional> {
    pub host: &'a H,
    host_transaction_binder: &'a dyn Fn(H::Transaction) -> UniverseTransaction,
}

impl<'a, H: UTransactional> BehaviorContext<'a, H> {
    fn bind_host(&self, transaction: H::Transaction) -> UniverseTransaction {
        (self.host_transaction_binder)(transaction)
    }
}

/// Collects [`Behavior`]s and invokes them.
///
/// `BehaviorSet` has interior mutability so that its behaviors can straightforwardly
/// mutate themselves, while holding a reference to the host object `H` that contains
/// the set.
#[derive(Debug)]
pub(crate) struct BehaviorSet<H> {
    items: RefCell<Vec<Box<dyn Behavior<H>>>>,
}

impl<H: Transactional> BehaviorSet<H> {
    pub fn new() -> Self {
        BehaviorSet {
            items: RefCell::new(Vec::new()),
        }
    }

    /// Add a behavior to the set.
    ///
    /// Design note: This method does not require `&mut self`, but the owner of the set
    /// should probably count it as a mutation for API purposes anyway.
    pub fn insert<B>(&self, behavior: B)
    where
        B: Behavior<H> + 'static,
    {
        self.items.borrow_mut().push(Box::new(behavior));
    }

    pub fn step(
        &self,
        host: &H,
        host_transaction_binder: &dyn Fn(H::Transaction) -> UniverseTransaction,
        duration: Duration,
    ) -> UniverseTransaction {
        let mut transactions = Vec::new();
        for behavior in self.items.borrow_mut().iter_mut() {
            let context = &BehaviorContext {
                host: &*host,
                host_transaction_binder,
            };
            if behavior.alive(context) {
                transactions.push(behavior.step(context, duration));
            } else {
                // TODO: mark for removal and prove it was done
            }
        }
        let transaction = transactions.into_iter().reduce(|a, b| a.merge(b).unwrap());
        transaction.unwrap_or_else(UniverseTransaction::default)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: more tests
}
