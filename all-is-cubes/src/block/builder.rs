//! Lesser-used helpers for [`BlockBuilder`].
use cgmath::EuclideanSpace as _;
use std::borrow::Cow;
use crate::block::{
    AnimationHint, Block, BlockAttributes, BlockCollision, Modifier, Primitive,
    Resolution, RotationPlacementRule,
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
pub(crate) struct BlockBuilder<P> {
    attributes: BlockAttributes,
    primitive_builder: P,
    modifiers: Vec<Modifier>,
}
/// Placeholder type for an incomplete [`BlockBuilder`]'s content. The builder
/// cannot create an actual block until this is replaced.
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct NeedsPrimitive;
