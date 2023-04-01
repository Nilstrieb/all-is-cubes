use std::ops::Deref;
use std::sync::Arc;
use crate::block::{Block, BlockChange};
use crate::listen::{Gate, Listen, Listener, Notifier};
use crate::transaction::{self, Transaction};
use crate::universe::{RefVisitor, VisitRefs};
/// Contains a [`Block`] and can be stored in a [`Universe`](crate::universe::Universe).
/// Together with [`Primitive::Indirect`], this allows mutation of a block definition such
/// that all its usages follow.
///
/// It is a distinct type from [`Block`] in order to ensure that change notifications
/// will be delivered on any mutation.
///
/// To perform a mutation, use [`BlockDefTransaction`].
#[derive(Debug)]
pub struct BlockDef {
    block: Block,
    notifier: Arc<Notifier<BlockChange>>,
    block_listen_gate: Gate,
}
impl BlockDef {
    /// Constructs a new [`BlockDef`] that stores the given block (which may be replaced
    /// in the future).
    pub(crate) fn new(block: Block) -> Self {
        loop {}
    }
}
impl Listen for BlockDef {
    type Msg = BlockChange;
    /// Registers a listener for mutations of any data sources which may affect the
    /// [`Block::evaluate`] result from blocks defined using this block definition.
    fn listen<L: Listener<BlockChange> + Send + Sync + 'static>(&self, listener: L) {
        loop {}
    }
}
impl Deref for BlockDef {
    type Target = Block;
    fn deref(&self) -> &Block {
        loop {}
    }
}
impl AsRef<Block> for BlockDef {
    fn as_ref(&self) -> &Block {
        loop {}
    }
}
impl VisitRefs for BlockDef {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
impl transaction::Transactional for BlockDef {
    type Transaction = BlockDefTransaction;
}
#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for BlockDef {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// A [`Transaction`] which replaces (or checks) the [`Block`] stored in a [`BlockDef`].
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[must_use]
pub struct BlockDefTransaction {
    /// If `None`, no precondition.
    old: Option<Block>,
    /// If `None`, no change is made and this transaction is only a precondition.
    new: Option<Block>,
}
impl BlockDefTransaction {
    /// Returns a transaction which fails if the current value of the [`BlockDef`] is not
    /// equal to `old`.
    pub(crate) fn expect(old: Block) -> Self {
        loop {}
    }
    /// Returns a transaction which replaces the current value of the [`BlockDef`] with `new`.
    pub(crate) fn overwrite(new: Block) -> Self {
        loop {}
    }
    /// Returns a transaction which replaces the value of the [`BlockDef`] with `new`,
    /// if it is equal to `old`, and otherwise fails.
    pub(crate) fn replace(old: Block, new: Block) -> Self {
        loop {}
    }
}
impl Transaction<BlockDef> for BlockDefTransaction {
    type CommitCheck = ();
    type Output = transaction::NoOutput;
    fn check(
        &self,
        target: &BlockDef,
    ) -> Result<Self::CommitCheck, transaction::PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        target: &mut BlockDef,
        (): Self::CommitCheck,
        _outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), transaction::CommitError> {
        loop {}
    }
}
impl transaction::Merge for BlockDefTransaction {
    type MergeCheck = ();
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, crate::transaction::TransactionConflict> {
        loop {}
    }
    fn commit_merge(self, other: Self, (): Self::MergeCheck) -> Self
    where
        Self: Sized,
    {
        loop {}
    }
}
