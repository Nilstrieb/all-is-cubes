//! Definition of blocks, which are the game objects which occupy the grid of a
//! [`Space`]. See [`Block`] for details.
//!
//! The types of most interest in this module are [`Block`], [`Primitive`],
//! [`BlockAttributes`], and [`Modifier`].
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;
use cgmath::EuclideanSpace as _;
use crate::listen::{Listen, Listener};
use crate::math::{GridPoint, GridRotation, Rgb, Rgba};
use crate::raycast::Ray;
use crate::space::{SetCubeError, Space};
use crate::universe::URef;
mod attributes;
pub use attributes::*;
mod block_def;
pub use block_def::*;
pub(crate) mod builder;
#[doc(inline)]
pub use builder::BlockBuilder;
mod evaluated;
pub use evaluated::*;
mod modifier;
pub use modifier::*;
mod resolution;
pub use resolution::*;
/// A [`Block`] is something that can exist in the grid of a [`Space`]; it occupies one
/// unit cube of simulated physical space, and has a specified appearance and behavior.
///
/// A [`Block`] is made up of a [`Primitive`] and zero or more [`Modifier`]s.
///
/// In general, when a block appears multiple times from an in-game perspective, that may
/// or may not be the the same copy; `Block`s are "by value" and any block [`Eq`] to
/// another will behave identically and should be treated identically. However, some
/// blocks are defined by reference to shared mutable data, and [`Block`] containers such
/// as [`Space`] must follow those changes.
///
/// To obtain the concrete appearance and behavior of a block, use [`Block::evaluate()`]
/// to obtain an [`EvaluatedBlock`] value, preferably with caching.
/// Use [`Block::listen()`] to be informed of possible changes to the result of
/// evaluation.
#[derive(Clone)]
pub struct Block(BlockPtr);
/// Pointer to data of a [`Block`] value.
///
/// This is a separate type so that the enum variants are not exposed.
/// It does not implement Eq and Hash, but Block does through it.
#[derive(Clone, Debug)]
enum BlockPtr {
    Static(&'static Primitive),
    Owned(Arc<BlockParts>),
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct BlockParts {
    primitive: Primitive,
    /// Modifiers are stored in innermost-first order.
    modifiers: Vec<Modifier>,
}
/// The possible fundamental representations of a [`Block`]'s shape.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Primitive {
    /// A block whose definition is stored elsewhere in a
    /// [`Universe`](crate::universe::Universe).
    ///
    /// Note that this is a reference to a [`Block`], not a [`Primitive`]; the referenced
    /// [`BlockDef`] may have its own [`Modifier`]s, and thus the result of
    /// [evaluating](Block::evaluate) a primitive with no modifiers is not necessarily
    /// free of the effects of modifiers.
    Indirect(URef<BlockDef>),
    /// A block that is a single-colored unit cube. (It may still be be transparent or
    /// non-solid to physics; in fact, [`AIR`] is such an atom.)
    Atom(BlockAttributes, Rgba),
    /// A block that is composed of smaller blocks, defined by the referenced [`Space`].
    Recur {
        #[allow(missing_docs)]
        attributes: BlockAttributes,
        /// The space from which voxels are taken.
        space: URef<Space>,
        /// Which portion of the space will be used, specified by the most negative
        /// corner.
        offset: GridPoint,
        /// The side length of the cubical volume of sub-blocks (voxels) used for this
        /// block.
        resolution: Resolution,
    },
    /// An invisible, unselectable, inert block used as “no block”; the primitive of [`AIR`].
    ///
    /// This is essentially a specific [`Primitive::Atom`]. There are a number of
    /// algorithms which treat this block specially or which return it (e.g. outside the
    /// bounds of a `Space`), so it exists here to make it an explicit element of the
    /// data model — so that if it is, say, serialized and loaded in a future version,
    /// it is still recognized as [`AIR`]. Additionally, it's cheaper to compare this way.
    Air,
}
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Block {
    /// Returns a new [`BlockBuilder`] which may be used to construct a [`Block`] value
    /// from various inputs with convenient syntax.
    pub(crate) const fn builder() -> BlockBuilder<builder::NeedsPrimitive> {
        loop {}
    }
    /// Construct a [`Block`] from a [`Primitive`] value.
    pub(crate) fn from_primitive(p: Primitive) -> Self {
        loop {}
    }
    /// Construct a [`Block`] from a [`Primitive`] constant.
    #[cfg(test)]
    pub(crate) const fn from_static_primitive(r: &'static Primitive) -> Self {
        loop {}
    }
    /// Returns the [`Primitive`] which defines this block before any
    /// [`Modifier`]s are applied.
    pub(crate) fn primitive(&self) -> &Primitive {
        loop {}
    }
    /// Returns a mutable reference to the [`Primitive`] which defines this block before
    /// any [`Modifier`]s are applied.
    ///
    /// This may cause part or all of the block's data to stop sharing storage with other
    /// blocks.
    pub(crate) fn primitive_mut(&mut self) -> &mut Primitive {
        loop {}
    }
    /// Returns all the modifiers of this block.
    ///
    /// Modifiers are arranged in order of their application to the primitive,
    /// or “innermost” to “outermost”.
    ///
    /// Note that this does not necessarily return all modifiers involved in its
    /// definition; modifiers on the far end of a [`Primitive::Indirect`] are
    /// not reported here, even though they take effect when evaluated.
    pub(crate) fn modifiers(&self) -> &[Modifier] {
        loop {}
    }
    /// Returns a mutable reference to the vector of [`Modifier`]s on this block.
    ///
    /// This may cause part or all of the block's data to stop sharing storage with other
    /// blocks.
    pub(crate) fn modifiers_mut(&mut self) -> &mut Vec<Modifier> {
        loop {}
    }
    fn make_parts_mut(&mut self) -> &mut BlockParts {
        loop {}
    }
    /// Add the given modifier to this block.
    ///
    /// This is a convenience operation which is exactly equivalent to
    /// doing `block.modifiers_mut().push(modifier.into())`. It does not do any of the
    /// special case logic that, for example, [`Block::rotate()`] does.
    #[must_use]
    pub(crate) fn with_modifier(mut self, modifier: impl Into<Modifier>) -> Self {
        loop {}
    }
    /// Rotates this block by the specified rotation.
    ///
    /// Compared to direct use of [`Modifier::Rotate`], this will:
    ///
    /// * Avoid constructing chains of redundant modifiers.
    /// * Not rotate blocks that should never appear rotated (including atom blocks).
    ///
    /// (TODO: This should be replaced with `with_modifier()` or similar having a general
    /// rule set for combining modifiers.)
    ///
    /// ```
    /// use all_is_cubes::block::{AIR, Block, Modifier};
    /// use all_is_cubes::content::make_some_voxel_blocks;
    /// use all_is_cubes::math::{GridRotation, Rgba};
    /// use all_is_cubes::universe::Universe;
    ///
    /// let mut universe = Universe::new();
    /// let [block] = make_some_voxel_blocks(&mut universe);
    /// let clockwise = GridRotation::CLOCKWISE;
    ///
    /// // Basic rotation
    /// let rotated = block.clone().rotate(clockwise);
    /// assert_eq!(rotated.modifiers(), &[Modifier::Rotate(clockwise)]);
    ///
    /// // Multiple rotations are combined
    /// let double = rotated.clone().rotate(clockwise);
    /// assert_eq!(double.modifiers(), &[Modifier::Rotate(clockwise * clockwise)]);
    ///
    /// // Atoms and AIR are never rotated
    /// let atom = Block::from(Rgba::WHITE);
    /// assert_eq!(atom.clone().rotate(clockwise), atom);
    /// assert_eq!(AIR.rotate(clockwise), AIR);
    /// ```
    #[must_use]
    pub(crate) fn rotate(mut self, rotation: GridRotation) -> Self {
        loop {}
    }
    /// Standardizes any characteristics of this block which may be presumed to be
    /// specific to its usage in its current location, so that it can be used elsewhere
    /// or compared with others. Specifically, it has the following effects:
    ///
    /// * Removes [`Modifier::Rotate`].
    ///
    /// In future versions there may be additional changes or ones customizable per block.
    ///
    /// # Examples
    ///
    /// Removing rotation:
    /// ```
    /// use all_is_cubes::block::Block;
    /// # use all_is_cubes::content::make_some_voxel_blocks;
    /// use all_is_cubes::math::GridRotation;
    /// use all_is_cubes::universe::Universe;
    ///
    /// let mut universe = Universe::new();
    /// let [block] = make_some_voxel_blocks(&mut universe);
    /// let rotated = block.clone().rotate(GridRotation::CLOCKWISE);
    ///
    /// assert_ne!(&block, &rotated);
    /// assert_eq!(vec![block], rotated.clone().unspecialize());
    /// ```
    #[must_use]
    pub(crate) fn unspecialize(&self) -> Vec<Block> {
        loop {}
    }
    /// Converts this `Block` into a “flattened” and snapshotted form which contains all
    /// information needed for rendering and physics, and does not require [`URef`] access
    /// to other objects.
    pub(crate) fn evaluate(&self) -> Result<EvaluatedBlock, EvalBlockError> {
        loop {}
    }
    #[inline]
    fn evaluate_impl(&self, depth: u8) -> Result<MinEval, EvalBlockError> {
        loop {}
    }
    /// Registers a listener for mutations of any data sources which may affect this
    /// block's [`Block::evaluate`] result.
    ///
    /// Note that this does not listen for mutations of the [`Block`] value itself, in the
    /// sense that none of the methods on [`Block`] will cause this listener to fire.
    /// Rather, it listens for changes in by-reference-to-interior-mutable-data sources
    /// such as the [`Space`] referred to by a [`Primitive::Recur`] or the [`BlockDef`]
    /// referred to by a [`Primitive::Indirect`].
    ///
    /// This may fail under the same conditions as [`Block::evaluate()`]; it returns the
    /// same error type so that callers which both evaluate and listen don't need to
    /// handle this separately.
    ///
    /// This is not an implementation of [`Listen`] because it can fail.
    pub(crate) fn listen(
        &self,
        listener: impl Listener<BlockChange> + Clone + Send + Sync + 'static,
    ) -> Result<(), EvalBlockError> {
        loop {}
    }
    fn listen_impl(
        &self,
        listener: impl Listener<BlockChange> + Clone + Send + Sync + 'static,
        depth: u8,
    ) -> Result<(), EvalBlockError> {
        loop {}
    }
    /// Returns the single [`Rgba`] color of this block's [`Primitive::Atom`] or
    /// [`Primitive::Air`], or panics if it has a different kind of primitive.
    /// **Intended for use in tests only.**
    pub(crate) fn color(&self) -> Rgba {
        loop {}
    }
}
/// Recursion limiter helper for evaluate.
fn next_depth(depth: u8) -> Result<u8, EvalBlockError> {
    loop {}
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        loop {}
    }
}
impl Eq for Block {}
impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        loop {}
    }
}
impl From<&'static Primitive> for Block {
    fn from(r: &'static Primitive) -> Self {
        loop {}
    }
}
impl From<Primitive> for Block {
    fn from(primitive: Primitive) -> Self {
        loop {}
    }
}
/// Convert a color to a block with default attributes.
impl From<Rgb> for Block {
    fn from(color: Rgb) -> Self {
        loop {}
    }
}
/// Convert a color to a block with default attributes.
impl From<Rgba> for Block {
    fn from(color: Rgba) -> Self {
        loop {}
    }
}
/// An invisible, unselectable, inert block used as “no block”.
///
/// It is used by [`Space`] to respond to out-of-bounds requests,
/// as well as other algorithms treating it as replaceable or discardable.
///
/// When evaluated, will always produce [`AIR_EVALUATED`].
pub const AIR: Block = Block(BlockPtr::Static(&Primitive::Air));
/// Given the `resolution` of some recursive block occupying `cube`, transform `ray`
/// into an equivalent ray intersecting the recursive grid.
///
#[inline]
pub(crate) fn recursive_ray(ray: Ray, cube: GridPoint, resolution: Resolution) -> Ray {
    loop {}
}
/// Notification when an [`EvaluatedBlock`] result changes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct BlockChange {
    /// I expect there _might_ be future uses for a set of flags of what changed;
    /// this helps preserve the option of adding them.
    _not_public: (),
}
impl BlockChange {
    #[allow(clippy::new_without_default)]
    #[allow(missing_docs)]
    pub(crate) fn new() -> BlockChange {
        loop {}
    }
}
/// Construct a set of [`Primitive::Recur`] blocks that form a miniature of the given `space`.
/// The returned [`Space`] contains each of the blocks; its coordinates will correspond to
/// those of the input, scaled down by `resolution`.
///
/// Returns [`SetCubeError::EvalBlock`] if the `Space` cannot be accessed, and
/// [`SetCubeError::TooManyBlocks`] if the dimensions would result in too many blocks.
///
/// TODO: add doc test for this
pub(crate) fn space_to_blocks(
    resolution: Resolution,
    attributes: BlockAttributes,
    space_ref: URef<Space>,
) -> Result<Space, SetCubeError> {
    loop {}
}
