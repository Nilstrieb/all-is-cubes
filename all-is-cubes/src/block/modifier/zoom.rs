use cgmath::{Point3};
use crate::block::{self, Resolution::{self}};

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