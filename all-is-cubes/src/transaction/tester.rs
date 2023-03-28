use super::*;
use std::error::Error;
use std::rc::Rc;
/// Tool for testing that a type of transaction obeys the rules:
///
/// * `Transaction::commit` should not actually produce errors; they should instead be
///   caught by `Transaction::check`.
/// * Two transactions, when merged, should have all the effects of both, or they should
///   fail to merge.
///
/// This test utility follows the builder pattern: call methods to add test data, then
/// finish with [`Self::test`].
#[must_use]
#[allow(missing_debug_implementations)]
pub struct TransactionTester<'a, Tr, Ta> {
    transactions: Vec<TransactionAndPredicate<'a, Tr, Ta>>,
    target_factories: Vec<Box<dyn Fn() -> Ta + 'a>>,
}
impl<'a, Tr, Ta> TransactionTester<'a, Tr, Ta>
where
    Tr: Transaction<Ta> + Clone + Debug + 'a,
    Ta: Debug + 'a,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        loop {}
    }
    /// Add a transaction to be checked.
    ///
    /// In addition to the explicitly provided transactions, each possible merge of
    /// two transactions will be tested.
    ///
    /// The `predicate` is given a copy of the target before and after executing the
    /// transaction and should verify that the transaction had the expected effects.
    /// There may be effects from other transactions.
    pub fn transaction(
        mut self,
        transaction: Tr,
        predicate: impl Fn(&Ta, &Ta) -> PredicateRes + 'a,
    ) -> Self {
        loop {}
    }
    /// Add a target to apply the tested transactions to.
    ///
    /// To avoid requiring the targets to implement [`Clone`], a factory function is
    /// required here.
    pub fn target(mut self, factory: impl Fn() -> Ta + 'a) -> Self {
        loop {}
    }
    /// Executes the tests and panics on failure.
    pub fn test(self) {
        loop {}
    }
    fn derived_transactions<'b: 'a>(
        &'b self,
    ) -> impl Iterator<Item = TransactionAndPredicate<'a, Tr, Ta>> + 'b {
        loop {}
    }
}
type PredicateRes = Result<(), Box<dyn Error>>;
struct TransactionAndPredicate<'a, Tr, Ta> {
    transaction: Tr,
    #[allow(clippy::type_complexity)]
    predicate: Rc<dyn Fn(&Ta, &Ta) -> PredicateRes + 'a>,
}
impl<'a, Tr: Clone, Ta> Clone for TransactionAndPredicate<'a, Tr, Ta> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<'a, Tr, Ta> TransactionAndPredicate<'a, Tr, Ta>
where
    Tr: Transaction<Ta>,
    Ta: 'a,
{
    fn try_merge(self, other: Self) -> Option<Self> {
        loop {}
    }
}
