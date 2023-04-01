use cgmath::{Point3};
use crate::block::{self, MinEval, Resolution::{self}};
use crate::math::{GridPoint};
use crate::universe;
/// Data for [`Modifier::Zoom`], describing a portion of the original block that is scaled
/// up to become the whole block.
///
/// Design note: This is a struct separate from [`Modifier`] so that it can have a
/// constructor accepting only valid bounds.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Zoom {
    /// Scale factor to zoom in by.
    scale: Resolution,
    /// Which portion of the block/space will be used, specified in terms of an offset
    /// in the grid of zoomed blocks (that is, this should have coordinates between `0`
    /// and `scale - 1`).
    offset: Point3<u8>,
}
impl Zoom {
    /// Construct a [`Zoom`] which enlarges the original block's voxels by `scale` and
    /// selects the region of them whose lower corner is `offset * scale`.
    ///
    /// Panics if any of `offset`'s components are out of bounds, i.e. less than 0 or
    /// greater than `scale - 1`.
    #[track_caller]
    pub fn new(scale: Resolution, offset: GridPoint) -> Self {
        loop {}
    }
    /// Decompose into parts, for serialization.
    pub(crate) fn to_serial_schema(&self) -> crate::save::schema::ModifierSer {
        loop {}
    }
    pub(super) fn evaluate(
        &self,
        input: MinEval,
    ) -> Result<MinEval, block::EvalBlockError> {
        loop {}
    }
}
impl From<Zoom> for block::Modifier {
    fn from(value: Zoom) -> Self {
        loop {}
    }
}
impl universe::VisitRefs for Zoom {
    fn visit_refs(&self, _visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Zoom {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {}
    }
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        loop {}
    }
}