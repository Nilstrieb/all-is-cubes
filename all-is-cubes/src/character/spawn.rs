use cgmath::{Point3, Vector3};
use crate::inv::Slot;
use crate::math::{FreeCoordinate, GridAab, NotNan};
use crate::universe::{RefVisitor, VisitRefs};
/// Defines the initial state of a [`Character`] that is being created or moved into a [`Space`].
///
/// TODO: This is lacking a full set of accessor methods to be viewable+editable.
///
/// [`Character`]: super::Character
/// [`Space`]: crate::space::Space
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Spawn {}
impl Spawn {
    /// Sets the position at which the character will appear, in terms of its viewpoint.
    pub(crate) fn set_eye_position(
        &mut self,
        position: impl Into<Point3<FreeCoordinate>>,
    ) {
        loop {}
    }
    /// Sets the bounds within which the character may be placed is allowed.
    pub(crate) fn set_bounds(&mut self, bounds: GridAab) {
        loop {}
    }
    /// Sets the direction the character should be facing, or looking at.
    ///
    /// The results are unspecified but harmless if the direction is zero or NaN.
    pub(crate) fn set_look_direction(
        &mut self,
        direction: impl Into<Vector3<FreeCoordinate>>,
    ) {
        loop {}
    }
    /// Sets the starting inventory items.
    pub(crate) fn set_inventory(&mut self, inventory: Vec<Slot>) {
        loop {}
    }
}
impl VisitRefs for Spawn {
    fn visit_refs(&self, visitor: &mut dyn RefVisitor) {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
#[mutants::skip]
impl<'a> arbitrary::Arbitrary<'a> for Spawn {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}
