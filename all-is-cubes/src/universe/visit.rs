use crate::universe::URefErased;
#[cfg(doc)]
use crate::universe::{URef, Universe};
/// Allows finding all of the [`URef`]s inside a data structure.
///
/// Correct implementations of this trait are necessary for many functions of a
/// [`Universe`] to work correctly; failing to report a reference may result in it
/// breaking.
pub(crate) trait VisitRefs {
    /// For each [`URef`] contained within `self` that is reachable without traversing
    /// another [`URef`], call `visitor` with a reference to it.
    fn visit_refs(&self, visitor: &mut dyn RefVisitor);
}
/// Callback used by [`VisitRefs::visit_refs`].
///
/// Note that this is automatically implemented for functions.
pub(crate) trait RefVisitor {
    /// Called by a value which is responding to a [`VisitRefs::visit_refs()`] operation
    /// to report one of the refs it contains.
    fn visit(&mut self, r: &dyn URefErased);
}
#[cfg(test)]
pub(crate) fn list_refs<T: VisitRefs + 'static>(target: &T) -> Vec<super::Name> {
    loop {}
}
