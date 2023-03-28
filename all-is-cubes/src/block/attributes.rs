//! [`BlockAttributes`] and closely related types.
use std::borrow::Cow;
use std::fmt;
use crate::drawing::VoxelBrush;
use crate::math::{Face6, Rgb};
#[cfg(doc)]
use crate::{
    block::{Block, BlockDef},
    space::Space,
};
/// Collection of miscellaneous attribute data for blocks that doesn't come in variants.
///
/// `BlockAttributes::default()` will produce a reasonable set of defaults for “ordinary”
/// blocks.
#[derive(Clone, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct BlockAttributes {
    /// The name that should be displayed to players.
    ///
    /// The default value is the empty string. The empty string should be considered a
    /// reasonable choice for solid-color blocks with no special features.
    pub display_name: Cow<'static, str>,
    /// Whether players' [cursors](crate::character::Cursor) target it or pass through it.
    ///
    /// The default value is `true`.
    pub selectable: bool,
    /// The effect on a [`Body`](crate::physics::Body) of colliding with this block.
    ///
    /// The default value is [`BlockCollision::Hard`].
    pub collision: BlockCollision,
    /// Rule about how this block should be rotated, or not, when placed in a [`Space`] by
    /// some agent not otherwise specifying rotation.
    ///
    /// The default value is [`RotationPlacementRule::Never`].
    pub rotation_rule: RotationPlacementRule,
    /// Light emitted by the block.
    ///
    /// The default value is [`Rgb::ZERO`].
    pub light_emission: Rgb,
    /// Something this block does when time passes.
    ///
    /// Currently the only possibility is “turn into another block”.
    ///
    /// TODO: Very placeholder. This needs more possible effects and also time/probability options.
    pub tick_action: Option<VoxelBrush<'static>>,
    /// Advice to the renderer about how to expect this block to change, and hence
    /// what rendering strategy to use.
    pub animation_hint: AnimationHint,
}
impl fmt::Debug for BlockAttributes {
    /// Only attributes which differ from the default are shown.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl BlockAttributes {
    /// Block attributes suitable as default values for in-game use.
    ///
    /// This function differs from the [`Default::default`] trait implementation only
    /// in that it is a `const fn`.
    pub const fn default() -> BlockAttributes {
        BlockAttributes {
            display_name: Cow::Borrowed(""),
            selectable: true,
            collision: BlockCollision::Hard,
            rotation_rule: RotationPlacementRule::Never,
            light_emission: Rgb::ZERO,
            tick_action: None,
            animation_hint: AnimationHint::UNCHANGING,
        }
    }
}
impl Default for BlockAttributes {
    /// Block attributes suitable as default values for in-game use.
    fn default() -> BlockAttributes {
        BlockAttributes::default()
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for BlockAttributes {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
/// Specifies the effect on a [`Body`](crate::physics::Body) of colliding with the
/// [`Block`] this applies to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub enum BlockCollision {
    /// The block can be passed through; it is not an obstacle (though intersecting it
    /// might cause other effects not directly part of collision response).
    None,
    /// The block is a perfectly solid obstacle occupying its entire bounding cube.
    ///
    /// This is the default value used for most blocks. (Caveat: The default might be
    /// changed to `Recur` if that proves more ergonomic.)
    Hard,
    /// Collide with the block's component voxels individually.
    ///
    /// If the block does not have voxels then this is equivalent to [`Hard`](Self::Hard).
    Recur,
}
/// Rule about how this block should be rotated, or not, when placed in a [`Space`] by
/// some agent not otherwise specifying rotation.
///
/// TODO: We may want to replace this with a struct that also carries declared symmetries
/// ("this is a vertical pillar so never make it upside down") and/or prohibited rotations
/// rather than requiring each individual rule variant to be sufficiently expressive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub enum RotationPlacementRule {
    /// Never rotate the block.
    Never,
    /// Rotate the block so that the specified face meets the face it was placed against.
    Attach {
        /// This face of the placed block will meet the face it was placed against.
        ///
        /// If the block was somehow placed without such an adjacent block, it will not be
        /// rotated.
        by: Face6,
    },
}
/// Specifies how a [`Block`] might change in the very near future, for the benefit
/// of rendering algorithms. Does not currently describe non-visual aspects of a block.
///
/// This should be configured for blocks which either are continuously animated in some
/// fashion, or for which it is especially important that the specified changes are handled
/// efficiently, at the possible cost of spending more resources on those blocks. Blocks
/// which merely might change in response to user action should not set this hint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct AnimationHint {
    /// Ways in which the block's definition might change (via modification to a
    /// [`BlockDef`] or recursive [`Space`]) such that many instances of this block
    /// will become another simultaneously.
    pub redefinition: AnimationChange,
    /// If this block is likely to be replaced in a [`Space`] by another, this field
    /// specifies the replacement's relation to this.
    pub replacement: AnimationChange,
}
impl AnimationHint {
    /// There are no expectations that the block is soon going to change.
    ///
    /// This is the default value of this type and within [`BlockAttributes`].
    pub const UNCHANGING: Self = Self {
        redefinition: AnimationChange::None,
        replacement: AnimationChange::None,
    };
    /// The block is not going to exist in its current form for long.
    ///
    /// This suggests using a rendering technique which is comparatively expensive
    /// per-block but allows it (and any successors that are also `TEMPORARY`) to be added
    /// and removed cheaply.
    pub const TEMPORARY: Self = Self {
        redefinition: AnimationChange::None,
        replacement: AnimationChange::Shape,
    };
    /// The block's appearance is expected to change very frequently, but not by replacing
    /// the block in its [`Space`].
    ///
    /// This suggests using a rendering technique which optimizes for not needing to e.g.
    /// rebuild chunk meshes.
    pub const CONTINUOUS: Self = Self {
        redefinition: AnimationChange::Shape,
        replacement: AnimationChange::None,
        ..Self::UNCHANGING
    };
    /// Returns whether this block's value for [`EvaluatedBlock::visible`] is likely to
    /// change from `false` to `true` for animation reasons.
    pub(crate) fn might_become_visible(&self) -> bool {
        loop {}
    }
}
impl Default for AnimationHint {
    fn default() -> Self {
        loop {}
    }
}
/// Component of [`AnimationHint`], describing the type of change predicted.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub enum AnimationChange {
    /// Expect no changes.
    None,
    /// Expect that the block’s voxels’ colors will change, while remaining within the
    /// same [`OpacityCategory`](crate::math::OpacityCategory); that is, the alpha will
    /// remain 0, remain 1, or remain neither 0 nor 1.
    ///
    /// Suggestion to renderers: prepare to update texturing without recomputing an
    /// otherwise identical mesh.
    ColorSameCategory,
    /// Expect that the block’s colors and shape will change; that is, at least some
    /// voxels’ alpha will move from one [`OpacityCategory`](crate::math::OpacityCategory)
    /// to another.
    ///
    /// Suggestion to renderers: use a rendering strategy which is shape-independent, or
    /// prepare to efficiently recompute the mesh (don't merge with neighbors).
    Shape,
}
impl AnimationChange {
    /// Helper for [`AnimationHint::might_become_visible`].
    fn might_become_visible(&self) -> bool {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// [`BlockAttributes`] has an inherent `default()` function, which should be
    /// equivalent to the [`Default`] trait function.
    #[test]
    fn default_equivalent() {
        loop {}
    }
    #[test]
    fn debug() {
        loop {}
    }
}
