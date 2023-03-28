//! Lesser-used helpers for [`BlockBuilder`].
use cgmath::EuclideanSpace as _;
use std::borrow::Cow;
use std::sync::Arc;
use crate::block::{
    AnimationHint, Block, BlockAttributes, BlockCollision, BlockDef, BlockParts,
    BlockPtr, Modifier, Primitive, Resolution, RotationPlacementRule, AIR,
};
use crate::drawing::VoxelBrush;
use crate::math::{GridPoint, Rgb, Rgba};
use crate::space::{SetCubeError, Space};
use crate::universe::{Name, URef, Universe, UniverseIndex};
/// Tool for constructing [`Block`] values conveniently.
///
/// To create one, call [`Block::builder()`].
/// ([`BlockBuilder::default()`] is also available.)
///
/// ```
/// use all_is_cubes::block::Block;
/// use all_is_cubes::math::Rgba;
///
/// let block = Block::builder()
///    .display_name("BROWN")
///    .color(Rgba::new(0.5, 0.5, 0., 1.))
///    .build();
///
/// assert_eq!(block.evaluate().unwrap().color, Rgba::new(0.5, 0.5, 0., 1.));
/// assert_eq!(
///     block.evaluate().unwrap().attributes.display_name.as_ref(),
///     "BROWN",
/// );
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct BlockBuilder<P> {
    attributes: BlockAttributes,
    primitive_builder: P,
    modifiers: Vec<Modifier>,
}
impl Default for BlockBuilder<NeedsPrimitive> {
    fn default() -> Self {
        loop {}
    }
}
impl BlockBuilder<NeedsPrimitive> {
    /// Common implementation of [`Block::builder`] and [`Default::default`]; use one of those to call this.
    pub(super) const fn new() -> BlockBuilder<NeedsPrimitive> {
        loop {}
    }
}
impl<C> BlockBuilder<C> {
    /// Sets the [`BlockAttributes`] the block will have.
    /// This replaces individual attribute values set using other builder methods.
    pub fn attributes(mut self, value: BlockAttributes) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::display_name`].
    pub fn display_name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::selectable`].
    pub const fn selectable(mut self, value: bool) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::collision`].
    pub const fn collision(mut self, value: BlockCollision) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::rotation_rule`].
    pub const fn rotation_rule(mut self, value: RotationPlacementRule) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::light_emission`].
    pub fn light_emission(mut self, value: impl Into<Rgb>) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::tick_action`].
    pub fn tick_action(mut self, value: Option<VoxelBrush<'static>>) -> Self {
        loop {}
    }
    /// Sets the value for [`BlockAttributes::animation_hint`].
    pub fn animation_hint(mut self, value: AnimationHint) -> Self {
        loop {}
    }
    /// Adds a modifier to the end of the list of modifiers for the block.
    /// It will be applied after all previously specified modifiers.
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        loop {}
    }
    /// Sets the color value for building a [`Primitive::Atom`].
    ///
    /// This will replace any previous color **or voxels.**
    pub fn color(self, color: impl Into<Rgba>) -> BlockBuilder<Rgba> {
        loop {}
    }
    /// Sets the space for building a [`Primitive::Recur`].
    ///
    /// This will replace any previous voxels **or color.**
    pub fn voxels_ref(
        self,
        resolution: Resolution,
        space: URef<Space>,
    ) -> BlockBuilder<BlockBuilderVoxels> {
        loop {}
    }
    /// Constructs a `Space` for building a [`Primitive::Recur`], and calls
    /// the given function to fill it with blocks, in the manner of [`Space::fill`].
    ///
    /// Note that the resulting builder is cloned, all clones will share the same
    /// space.
    pub fn voxels_fn<F, B>(
        self,
        universe: &mut Universe,
        resolution: Resolution,
        mut function: F,
    ) -> Result<BlockBuilder<BlockBuilderVoxels>, SetCubeError>
    where
        F: FnMut(GridPoint) -> B,
        B: std::borrow::Borrow<Block>,
    {
        loop {}
    }
    /// Converts this builder into a block value.
    pub fn build(self) -> Block
    where
        C: BuildPrimitiveIndependent,
    {
        loop {}
    }
    /// Converts this builder into a block value and stores it as a [`BlockDef`] in
    /// the given [`Universe`] with the given name, then returns a [`Primitive::Indirect`]
    /// block referring to it.
    pub fn into_named_definition(
        self,
        universe: &mut Universe,
        name: impl Into<Name>,
    ) -> Result<Block, crate::universe::InsertError>
    where
        C: BuildPrimitiveInUniverse,
    {
        loop {}
    }
}
/// Voxel-specific builder methods.
impl BlockBuilder<BlockBuilderVoxels> {
    /// Sets the coordinate offset for building a [`Primitive::Recur`]:
    /// the lower-bound corner of the region of the [`Space`]
    /// which will be used for block voxels. The default is zero.
    pub fn offset(mut self, offset: GridPoint) -> Self {
        loop {}
    }
}
/// Allows implicitly converting `BlockBuilder` to the block it would build.
impl<C: BuildPrimitiveIndependent> From<BlockBuilder<C>> for Block {
    fn from(builder: BlockBuilder<C>) -> Self {
        loop {}
    }
}
/// Equivalent to `Block::builder().color(color)`.
impl From<Rgba> for BlockBuilder<Rgba> {
    fn from(color: Rgba) -> Self {
        loop {}
    }
}
/// Equivalent to `Block::builder().color(color.with_alpha_one())`.
impl From<Rgb> for BlockBuilder<Rgba> {
    fn from(color: Rgb) -> Self {
        loop {}
    }
}
/// Placeholder type for an incomplete [`BlockBuilder`]'s content. The builder
/// cannot create an actual block until this is replaced.
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct NeedsPrimitive;
/// Primitive-builder of a [`BlockBuilder`] that can build a block without a [`Universe`].
#[doc(hidden)]
pub trait BuildPrimitiveIndependent {
    fn build_i(self, attributes: BlockAttributes) -> Primitive;
}
/// Primitive-builder of a [`BlockBuilder`] that can only build a block with a [`Universe`].
#[doc(hidden)]
pub trait BuildPrimitiveInUniverse {
    fn build_u(self, attributes: BlockAttributes, universe: &mut Universe) -> Primitive;
}
/// Every [`BuildPrimitiveIndependent`] can act as [`BuildPrimitiveInUniverse`].
impl<T: BuildPrimitiveIndependent> BuildPrimitiveInUniverse for T {
    fn build_u(self, attributes: BlockAttributes, _: &mut Universe) -> Primitive {
        loop {}
    }
}
/// Used by [`BlockBuilder::color`].
impl BuildPrimitiveIndependent for Rgba {
    fn build_i(self, attributes: BlockAttributes) -> Primitive {
        loop {}
    }
}
/// Concrete type for a [`BlockBuilder`] that is building a voxel block.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockBuilderVoxels {
    space: URef<Space>,
    resolution: Resolution,
    offset: GridPoint,
}
impl BuildPrimitiveIndependent for BlockBuilderVoxels {
    fn build_i(self, attributes: BlockAttributes) -> Primitive {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use crate::block::{Resolution::*, AIR};
    use crate::math::{Face6, GridAab};
    use crate::space::SpacePhysics;
    use super::*;
    #[test]
    fn defaults() {
        loop {}
    }
    #[test]
    fn default_equivalent() {
        loop {}
    }
    #[test]
    fn every_field_nondefault() {
        loop {}
    }
    #[test]
    fn voxels_from_space() {
        loop {}
    }
    #[test]
    fn voxels_from_fn() {
        loop {}
    }
}
