//! [`EvaluatedBlock`] and [`Evoxel`].
use std::fmt;
use cgmath::Zero as _;
use crate::block::{self, BlockAttributes, Resolution};
use crate::math::{FaceMap, GridAab, GridArray, GridPoint, OpacityCategory, Rgb, Rgba};
use crate::universe::RefError;
#[cfg(doc)]
use super::{Block, Primitive, URef, AIR};
/// A snapshotted form of [`Block`] which contains all information needed for rendering
/// and physics, and does not require dereferencing [`URef`]s or unbounded computation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) struct EvaluatedBlock {
    /// The block's attributes.
    pub(crate) attributes: BlockAttributes,
    /// The voxels making up the block, and the [`resolution`](Resolution) (scale factor)
    /// of those voxels.
    pub(crate) voxels: Evoxels,
    /// The block's color; if made of multiple voxels, then an average or representative
    /// color.
    pub(crate) color: Rgba,
    /// Whether the block is known to be completely opaque to light passing in or out of
    /// each face.
    ///
    /// Currently, this is calculated as whether each of the surfaces of the block are
    /// fully opaque, but in the future it might be refined to permit concave surfaces.
    pub(crate) opaque: FaceMap<bool>,
    /// Whether the block has any voxels/color at all that make it visible; that is, this
    /// is false if the block is completely transparent.
    pub(crate) visible: bool,
    /// The opacity of all voxels. This is redundant with the data  [`Self::voxels`],
    /// and is provided as a pre-computed convenience that can be cheaply compared with
    /// other values of the same type.
    ///
    /// May be [`None`] if the block is fully invisible. (TODO: This is a kludge to avoid
    /// obligating [`AIR_EVALUATED`] to allocate at compile time, which is impossible.
    /// It doesn't harm normal operation because the point of having this is to compare
    /// block shapes, which is trivial if the block is invisible.)
    pub(crate) voxel_opacity_mask: Option<GridArray<OpacityCategory>>,
}
impl fmt::Debug for EvaluatedBlock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl EvaluatedBlock {
    /// Computes the derived values of a voxel block.
    ///
    /// This is also available as `impl From<MinEval> for EvaluatedBlock`.
    pub(crate) fn from_voxels(
        attributes: BlockAttributes,
        voxels: Evoxels,
    ) -> EvaluatedBlock {
        loop {}
    }
    /// Returns the resolution (scale factor) of this block's voxels.
    /// See [`Resolution`] for more information.
    #[inline]
    pub(crate) fn resolution(&self) -> Resolution {
        loop {}
    }
    /// Returns whether [`Self::visible`] is true (the block has some visible color/voxels)
    /// or [`BlockAttributes::animation_hint`] indicates that the block might _become_
    /// visible (by change of evaluation result rather than by being replaced).
    #[inline]
    pub(crate) fn visible_or_animated(&self) -> bool {
        loop {}
    }
    /// Returns the bounding box of the voxels, or the full cube if no voxels,
    /// scaled up by `resolution`.
    ///
    /// TODO: This isn't a great operation to be exposing because it “leaks” the implementation
    /// detail of whether the bounds are tightly fitting or not, particularly for the purpose
    /// it is being used for (cursor drawing). Figure out what we want to do instead.
    #[doc(hidden)]
    pub(crate) fn voxels_bounds(&self) -> GridAab {
        loop {}
    }
    #[doc(hidden)]
    #[track_caller]
    pub(crate) fn consistency_check(&self) {
        loop {}
    }
}
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
impl EvalBlockError {
    /// Convert this error into an [`EvaluatedBlock`] which represents that an error has
    /// occurred.
    ///
    /// This block is fully opaque and as inert to game mechanics as currently possible.
    pub(crate) fn to_placeholder(&self) -> EvaluatedBlock {
        loop {}
    }
}
/// Properties of an individual voxel within [`EvaluatedBlock`].
///
/// This is essentially a subset of the information in a full [`EvaluatedBlock`] and
/// its [`BlockAttributes`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) struct Evoxel {
    /// Diffuse reflection color.
    pub(crate) color: Rgba,
    /// Whether players' [cursors](crate::character::Cursor) target this voxel's containing
    /// block or pass through it.
    pub(crate) selectable: bool,
    /// The effect on a [`Body`](crate::physics::Body) of colliding with this voxel.
    pub(crate) collision: block::BlockCollision,
}
impl Evoxel {
    /// The `Evoxel` value that would have resulted from using [`AIR`] in a recursive block.
    ///
    /// TODO: Write a test for that.
    pub(crate) const AIR: Self = Self {
        color: Rgba::TRANSPARENT,
        selectable: false,
        collision: block::BlockCollision::None,
    };
    /// Construct an [`Evoxel`] which represents the given evaluated block.
    ///
    /// This is the same operation as is used for each block/voxel in a [`Primitive::Recur`].
    pub(crate) fn from_block(block: &EvaluatedBlock) -> Self {
        loop {}
    }
    /// Construct the [`Evoxel`] that would have resulted from evaluating a voxel block
    /// with the given color and default attributes.
    pub(crate) const fn from_color(color: Rgba) -> Self {
        const DA: &BlockAttributes = &BlockAttributes::default();
        Self {
            color,
            selectable: DA.selectable,
            collision: DA.collision,
        }
    }
}
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
    Many(Resolution, GridArray<Evoxel>),
}
impl Evoxels {
    /// Returns the resolution (scale factor) of this set of voxels.
    /// See [`Resolution`] for more information.
    #[inline]
    pub(crate) fn resolution(&self) -> Resolution {
        loop {}
    }
    /// If this has a resolution of 1, then return that single voxel.
    #[inline]
    pub(crate) fn single_voxel(&self) -> Option<Evoxel> {
        loop {}
    }
    /// Get the single voxel at the specified position, or [`None`] if the position is
    /// out of bounds of the data (which is not necessarily out of bounds of the block;
    /// missing data should be taken as [`Evoxel::AIR`]).
    ///
    /// Generally behaves like [`GridArray::get()`].
    ///
    /// TODO: Should we inherently return AIR instead of None?
    #[inline]
    pub(crate) fn get(&self, position: GridPoint) -> Option<Evoxel> {
        loop {}
    }
    /// Returns the bounds of the voxel data.
    #[inline]
    pub(crate) fn bounds(&self) -> GridAab {
        loop {}
    }
}
impl std::ops::Index<GridPoint> for Evoxels {
    type Output = Evoxel;
    #[inline]
    #[track_caller]
    fn index(&self, position: GridPoint) -> &Self::Output {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for Evoxels {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// The result of <code>[AIR].[evaluate()](Block::evaluate)</code>, as a constant.
/// This may be used when an [`EvaluatedBlock`] value is needed but there is no block
/// value.
///
/// ```
/// use all_is_cubes::block::{AIR, AIR_EVALUATED};
///
/// assert_eq!(Ok(AIR_EVALUATED), AIR.evaluate());
/// ```
pub(crate) const AIR_EVALUATED: EvaluatedBlock = EvaluatedBlock {
    attributes: AIR_ATTRIBUTES,
    color: Rgba::TRANSPARENT,
    voxels: Evoxels::One(AIR_INNER_EVOXEL),
    opaque: FaceMap::repeat_copy(false),
    visible: false,
    voxel_opacity_mask: None,
};
/// Note that this voxel is *not* no-collision and unselectable; the block attributes
/// override it. For now, all atom blocks work this way. TODO: Perhaps we should change that.
const AIR_INNER_EVOXEL: Evoxel = Evoxel::from_color(Rgba::TRANSPARENT);
/// Used only by [`AIR_EVALUATED`].
const AIR_ATTRIBUTES: BlockAttributes = BlockAttributes {
    display_name: std::borrow::Cow::Borrowed("<air>"),
    selectable: false,
    collision: block::BlockCollision::None,
    rotation_rule: block::RotationPlacementRule::Never,
    light_emission: Rgb::ZERO,
    tick_action: None,
    animation_hint: block::AnimationHint::UNCHANGING,
};
/// A minimal version of [`EvaluatedBlock`] which contains all the fundamental data, but
/// none of the computed data.
///
/// This type is used as the intermediate type inside block modifier evaluation, so as to
/// avoid computing any derived data that will be discarded anyway, or possibly
/// mis-computing some of the derived data as an attempted optimization.
/// This type is never exposed as part of the public API; only [`EvaluatedBlock`] is.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct MinEval {
    pub(crate) attributes: BlockAttributes,
    pub(crate) voxels: Evoxels,
}
impl From<MinEval> for EvaluatedBlock {
    fn from(value: MinEval) -> Self {
        loop {}
    }
}
impl MinEval {
    pub(crate) fn resolution(&self) -> Resolution {
        loop {}
    }
}
