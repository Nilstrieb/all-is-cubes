//! [`EvaluatedBlock`] and [`Evoxel`].

use crate::block::Resolution;

use crate::universe::RefError;
#[cfg(doc)]
use super::{Block, Primitive, URef, AIR};
/// Errors resulting from [`Block::evaluate`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum EvalBlockError {
    /// The block definition contained recursion that exceeded the evaluation limit.
    #[error("block definition contains too much recursion")]
    StackOverflow,
    /// Data referenced by the block definition was not available to read.
    ///
    /// This may be temporary or permanent; consult the [`RefError`] to determine that.
    #[error("block data inaccessible: {0}")]
    DataRefIs(#[from] RefError),
}
/// Properties of an individual voxel within [`EvaluatedBlock`].
///
/// This is essentially a subset of the information in a full [`EvaluatedBlock`] and
/// its [`BlockAttributes`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) struct Evoxel {}
/// Storage of an [`EvaluatedBlock`]'s shape — its _evaluated voxels._
///
/// This voxel data may be smaller than the dimensions implied by [`Self::resolution`],
/// in which case the out-of-bounds space should be treated as [`Evoxel::AIR`].
/// The logical bounds are always the cube computed by [`GridAab::for_block`].
///
/// This improves on a `GridArray<Evoxel>` by avoiding heap allocation and indirection
/// for the case of a single element, and by returning voxels by value rather than
/// reference.
///
/// TODO: Make this opaque instead of an enum; replace all matching on `One` vs. `Many`
/// with calls to [`Self::single_voxel()`] or similar. This will:
///
/// * allow ensuring consistent input (no out-of-bounds data, not using `Many` for one)
/// * allow more compact representations (e.g. when all voxels are solid+selectable)
/// * ensure there is no inappropriate dependence on the representation
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum Evoxels {
    /// Compact representation of exactly one voxel. The resolution is implicitly 1.
    One(Evoxel),
    /// The [`GridArray`] should not have any data outside of the expected bounds
    /// `GridAab::for_block(resolution)`.
    Many(Resolution),
}
/// A minimal version of [`EvaluatedBlock`] which contains all the fundamental data, but
/// none of the computed data.
///
/// This type is used as the intermediate type inside block modifier evaluation, so as to
/// avoid computing any derived data that will be discarded anyway, or possibly
/// mis-computing some of the derived data as an attempted optimization.
/// This type is never exposed as part of the public API; only [`EvaluatedBlock`] is.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct MinEval {}
