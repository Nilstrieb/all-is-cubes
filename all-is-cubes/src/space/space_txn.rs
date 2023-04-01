//! TODO: Maybe this file is too small
use std::collections::BTreeMap;
use std::fmt;
use crate::behavior::{self, BehaviorSet, BehaviorSetTransaction};
use crate::block::Block;
use crate::drawing::DrawingPlane;
use crate::math::{GridCoordinate, GridMatrix, GridPoint};
use crate::space::{GridAab, Space};
use crate::transaction::{
    CommitError, Merge, NoOutput, PreconditionFailed, Transaction, TransactionConflict,
    Transactional,
};

impl Transactional for Space {
    type Transaction = SpaceTransaction;
}
/// A [`Transaction`] that modifies a [`Space`].
#[derive(Clone, Default, Eq, PartialEq)]
#[must_use]
pub struct SpaceTransaction {
    cubes: BTreeMap<[GridCoordinate; 3], CubeTransaction>,
    behaviors: BehaviorSetTransaction<Space>,
}
impl SpaceTransaction {
    /// Construct a [`SpaceTransaction`] for a single cube.
    ///
    /// If `old` is not [`None`], requires that the existing block is that block or the
    /// transaction will fail.
    /// If `new` is not [`None`], replaces the existing block with `new`.
    ///
    /// TODO: This name is a poor name now that [`Self::set`] exists.
    pub(crate) fn set_cube(
        cube: impl Into<GridPoint>,
        old: Option<Block>,
        new: Option<Block>,
    ) -> Self {
        loop {}
    }
    /// Expand this transaction to include modifying the given cube, or return an error if
    /// that would conflict (by the same definition as transaction merging).
    ///
    /// If `old` is not [`None`], requires that the existing block is that block or the
    /// transaction will fail.
    /// If `new` is not [`None`], replaces the existing block with `new`.
    pub(crate) fn set(
        &mut self,
        cube: impl Into<GridPoint>,
        old: Option<Block>,
        new: Option<Block>,
    ) -> Result<(), TransactionConflict> {
        loop {}
    }
    /// Expand this transaction to include modifying the given cube, replacing any
    /// existing modification instruction (but not an existing `old` block precondition).
    /// This is thus comparable to a direct [`Space::set()`] after the rest of the
    /// transaction.
    pub(crate) fn set_overwrite(&mut self, cube: impl Into<GridPoint>, block: Block) {
        loop {}
    }
    /// Provides an [`DrawTarget`](embedded_graphics::prelude::DrawTarget)
    /// adapter for 2.5D drawing.
    ///
    /// For more information on how to use this, see
    /// [`all_is_cubes::drawing`](crate::drawing).
    pub(crate) fn draw_target<C>(
        &mut self,
        transform: GridMatrix,
    ) -> DrawingPlane<'_, Self, C> {
        loop {}
    }
    /// Marks all cube modifications in this transaction as [non-conservative].
    ///
    /// This means that two transactions which both place the same block in a given cube
    /// may be merged, whereas the default state is that they will conflict (on the
    /// principle that such a merge could cause there to be fewer total occurrences of
    /// that block than intended).
    ///
    /// Also, the transaction will not fail if some of its cubes are outside the bounds of
    /// the [`Space`].
    ///
    /// [non-conservative]: https://en.wikipedia.org/wiki/Conserved_quantity
    pub(crate) fn nonconserved(mut self) -> Self {
        loop {}
    }
    fn single(cube: impl Into<GridPoint>, transaction: CubeTransaction) -> Self {
        loop {}
    }
    /// Modify the space's [`BehaviorSet`].
    pub(crate) fn behaviors(t: behavior::BehaviorSetTransaction<Space>) -> Self {
        loop {}
    }
    /// Add a behavior to the [`Space`].
    /// This is a shortcut for creating a [`BehaviorSetTransaction`].
    pub(crate) fn add_behavior<B>(bounds: GridAab, behavior: B) -> Self
    where
        B: behavior::Behavior<Space> + 'static,
    {
        loop {}
    }
    pub(crate) fn activate_block(cube: GridPoint) -> Self {
        loop {}
    }
    /// Computes the region of cubes directly affected by this transaction.
    /// Ignores behaviors.
    ///
    /// Returns [`None`] if no cubes are affected.
    ///
    /// TODO: Handle the case where the total volume is too large.
    /// (Maybe `GridAab` should lose that restriction.)
    pub(crate) fn bounds_only_cubes(&self) -> Option<GridAab> {
        loop {}
    }
    /// Computes the region affected by this transaction.
    ///
    /// Returns [`None`] if no specific regions of the space are affected.
    pub(crate) fn bounds(&self) -> Option<GridAab> {
        loop {}
    }
}
impl Transaction<Space> for SpaceTransaction {
    type CommitCheck = <BehaviorSetTransaction<
        Space,
    > as Transaction<BehaviorSet<Space>>>::CommitCheck;
    type Output = NoOutput;
    fn check(&self, space: &Space) -> Result<Self::CommitCheck, PreconditionFailed> {
        loop {}
    }
    fn commit(
        &self,
        space: &mut Space,
        check: Self::CommitCheck,
        _outputs: &mut dyn FnMut(Self::Output),
    ) -> Result<(), CommitError> {
        loop {}
    }
}
impl Merge for SpaceTransaction {
    type MergeCheck = <BehaviorSetTransaction<Space> as Merge>::MergeCheck;
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(mut self, mut other: Self, check: Self::MergeCheck) -> Self {
        loop {}
    }
}
impl fmt::Debug for SpaceTransaction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// Data for a single cube in a [`SpaceTransaction`]. This does not function as a
/// transaction on its own, though it does implement [`Merge`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct CubeTransaction {
    /// Previous block which must occupy this cube.
    /// If `None`, no precondition.
    old: Option<Block>,
    /// Block to be put in this cube.
    /// If `None`, this is only a precondition for modifying another block.
    new: Option<Block>,
    /// If true, two transactions with the same `new` block may not be merged.
    conserved: bool,
    /// The cube was “activated” (clicked on, more or less) and should
    /// respond to that.
    activate: bool,
}
impl CubeTransaction {
    const ACTIVATE: Self = Self {
        old: None,
        new: None,
        conserved: false,
        activate: true,
    };
}
impl Merge for CubeTransaction {
    type MergeCheck = CubeMergeCheck;
    fn check_merge(
        &self,
        other: &Self,
    ) -> Result<Self::MergeCheck, TransactionConflict> {
        loop {}
    }
    fn commit_merge(self, other: Self, CubeMergeCheck {}: Self::MergeCheck) -> Self {
        loop {}
    }
}
struct CubeMergeCheck {}
