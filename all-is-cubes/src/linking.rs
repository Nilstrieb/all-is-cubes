//! Storing and accessing definitions of standard blocks in a [`Universe`].
//!
//! An enum implementing [`BlockModule`] defines a set of names, and
//! [`BlockProvider`] assists in ensuring that all of those names are defined
//! and storing or retrieving their block values in a specific [`Universe`].
//!
//! In the future this mechanism may grow to become a dynamic linker/dependency injector
//! by becoming aware of dependencies between “modules”. For now, it's just enough to
//! solve bootstrapping needs.
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::ops::Index;
use exhaust::Exhaust;
use crate::block::{Block, BlockDef, Primitive};
use crate::space::SetCubeError;
use crate::transaction::ExecuteError;
use crate::universe::{InsertError, Name, URef, Universe, UniverseIndex};
use crate::util::YieldProgress;
fn name_in_module<E: BlockModule>(key: &E) -> Name {
    loop {}
}
/// Allows the use of [`BlockProvider::default`] to construct a [`BlockProvider`]
/// using this type as its set of keys. [`Self::default`] will be called once for
/// each value of [`Self`].
///
/// See [`BlockModule`] for related expectations.
pub trait DefaultProvision {
    /// Returns the default block value to use for the given key. This will typically
    /// have to be a [`Primitive::Atom`] block.
    fn default(self) -> Block;
}
/// Types whose values identify blocks in a set of related blocks, which may be
/// stored in a [`BlockProvider`] or under specific names in a [`Universe`].
///
/// The names of the [`Universe`]'s corresponding [`BlockDef`]s are formed by
/// combining the [`namespace()`](Self::namespace) and `self.to_string()` (the
/// [`Display`](fmt::Display) trait implementation).
///
/// Implement this trait for an enum, then use the functions of
/// [`BlockProvider`] to work with the described set of blocks.
///
/// TODO: consider replacing Display with a separate method so as not to presume its meaning
pub trait BlockModule: Exhaust + fmt::Debug + fmt::Display + Eq + Hash + Clone {
    /// A namespace for the members of this module; currently, this should be a
    /// `/`-separated path with no trailing slash, but (TODO:) we should have a
    /// more rigorous namespace scheme for [`Name`]s in future versions.
    fn namespace() -> &'static str;
}
/// TODO: document
#[derive(Clone, Debug)]
pub struct BlockProvider<E> {
    /// Guaranteed to contain an entry for every variant of `E` if `E`'s
    /// [`Exhaust`] implementation is accurate.
    map: HashMap<E, Block>,
}
impl<E> Default for BlockProvider<E>
where
    E: DefaultProvision + Exhaust + Eq + Hash + Clone,
{
    fn default() -> Self {
        loop {}
    }
}
impl<E> BlockProvider<E>
where
    E: BlockModule,
{
    /// Constructs a `BlockProvider` with block definitions computed by the given function.
    ///
    /// This is an async function for the sake of cancellation and optional cooperative
    /// multitasking. It may be blocked on from a synchronous context.
    pub async fn new<F, B>(
        progress: YieldProgress,
        mut definer: F,
    ) -> Result<Self, GenError>
    where
        F: FnMut(E) -> Result<B, InGenError>,
        B: Into<Block>,
    {
        loop {}
    }
    /// Add the block definitions stored in this [`BlockProvider`] into `universe` as
    /// [`BlockDef`]s, returning a new [`BlockProvider`] whose blocks refer to those
    /// definitions (via [`Primitive::Indirect`]).
    ///
    /// TODO: Migrate this to operate via `UniverseTransaction` instead.
    pub fn install(
        &self,
        universe: &mut Universe,
    ) -> Result<BlockProvider<E>, InsertError> {
        loop {}
    }
    /// Obtain the definitions of `E`'s blocks from `universe`, returning a new
    /// [`BlockProvider`] whose blocks refer to those definitions (via
    /// [`Primitive::Indirect`]).
    ///
    /// Returns an error if any of the blocks are not defined in that universe.
    pub fn using(universe: &Universe) -> Result<BlockProvider<E>, ProviderError>
    where
        E: Eq + Hash + fmt::Display,
    {
        loop {}
    }
    /// Iterate over the entire contents of this.
    pub fn iter(&self) -> impl Iterator<Item = (E, &Block)> + Send
    where
        E: Sync,
        <E as Exhaust>::Iter: Send,
    {
        E::exhaust()
            .map(|key| {
                let block: &Block = &self.map[&key];
                (key, block)
            })
    }
}
/// These methods do not require `E` to be a [`BlockModule`].
impl<E: Exhaust + fmt::Debug + Clone + Eq + Hash> BlockProvider<E> {
    /// Alternative to [`Self::new()`] which is neither async nor fallible.
    fn new_sync<F>(mut definer: F) -> Self
    where
        F: FnMut(E) -> Block,
    {
        loop {}
    }
    /// Create another [`BlockProvider`] with different keys that map into a subset of
    /// this provider's keys.
    ///
    /// TODO: add a test
    #[must_use]
    pub fn subset<K: Exhaust + fmt::Debug + Clone + Eq + Hash>(
        &self,
        function: impl Fn(K) -> E,
    ) -> BlockProvider<K> {
        loop {}
    }
    /// Create another [`BlockProvider`] with a modification to each block.
    #[must_use]
    pub fn map(&self, mut function: impl FnMut(&E, &Block) -> Block) -> Self {
        loop {}
    }
    #[cfg(test)]
    fn consistency_check(&self) {
        loop {}
    }
}
impl<E: Eq + Hash> Index<E> for BlockProvider<E> {
    type Output = Block;
    fn index(&self, index: E) -> &Self::Output {
        loop {}
    }
}
/// Error when a [`BlockProvider`] could not be created because the definitions of some
/// of its blocks are missing.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("missing block definitions: {missing:?}")]
pub struct ProviderError {
    missing: Box<[Name]>,
}
/// An error resulting from “world generation”: failure to calculate/create/place objects
/// (due to bad parameters or unforeseen edge cases), failure to successfully store them
/// in or retrieve them from a [`Universe`], et cetera.
#[derive(Debug, thiserror::Error)]
pub struct GenError {
    #[source]
    detail: InGenError,
    for_object: Option<Name>,
}
impl GenError {
    /// Wrap an error, that occurred while creating an object, as a [`GenError`] which also
    /// names the object.
    pub fn failure(error: impl Into<InGenError>, object: Name) -> Self {
        loop {}
    }
}
impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl From<InsertError> for GenError {
    fn from(error: InsertError) -> Self {
        loop {}
    }
}
/// Aggregation of types of errors that might occur in “world generation”.
///
/// This is distinct from [`GenError`] in that this type is returned from functions
/// _responsible for generation,_ and that type is returned from functions that
/// _manage_ generation — that invoke the first kind and (usually) store its result
/// in the [`Universe`]. This separation is intended to encourage more precise
/// attribution of the source of the error despite implicit conversions, because a
/// “nested” [`GenError`] will be obligated to be wrapped in `InGenError` rather than
/// mistakenly taken as the same level.
///
/// TODO: Work this into a coherent set of error cases rather than purely
/// "I saw one of these once, so add it".
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InGenError {
    /// Generic error container for unusual situations.
    #[error(transparent)]
    Other(Box<dyn Error + Send + Sync>),
    /// Something else needed to be generated and that failed.
    #[error(transparent)]
    Gen(Box<GenError>),
    /// Failed to insert the generated items in the [`Universe`].
    #[error(transparent)]
    Insert(#[from] InsertError),
    /// Failed to find a needed dependency.
    #[error(transparent)]
    Provider(#[from] ProviderError),
    /// Failed during [`Space`](crate::space::Space) manipulation.
    #[error(transparent)]
    SetCube(#[from] SetCubeError),
    /// Failed during a transaction.
    #[error(transparent)]
    Transaction(#[from] ExecuteError),
}
impl InGenError {
    /// Convert an arbitrary error to `InGenError`.
    pub fn other<E: Error + Send + Sync + 'static>(error: E) -> Self {
        loop {}
    }
}
impl From<GenError> for InGenError {
    fn from(error: GenError) -> Self {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Quote, Resolution::*};
    use crate::content::make_some_blocks;
    use crate::math::GridAab;
    use crate::util::assert_send_sync;
    #[derive(Exhaust, Clone, Debug, Eq, Hash, PartialEq)]
    enum Key {
        A,
        B,
        C,
    }
    fn test_provider() -> ([Block; 3], BlockProvider<Key>) {
        loop {}
    }
    #[test]
    fn provider_subset() {
        loop {}
    }
    #[test]
    fn provider_map() {
        loop {}
    }
    #[test]
    fn errors_are_send_sync() {
        loop {}
    }
    #[test]
    fn gen_error_message() {
        loop {}
    }
    #[test]
    #[allow(clippy::try_err)]
    fn gen_error_composition() {
        loop {}
    }
}
