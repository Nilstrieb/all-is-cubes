use crate::block;
use crate::universe;
/// Data for [`Modifier::Quote`](block::Modifier::Quote).
/// Suppresses all behaviors of the [`Block`](block::Block) that might affect the space
/// around it, (or itself).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct Quote {
    /// If true, also suppress light and sound effects.
    pub suppress_ambient: bool,
}
impl Quote {
    /// Construct an instance of [`Quote`], the same as [`Quote::default()`].
    pub fn new() -> Self {
        loop {}
    }
}
impl universe::VisitRefs for Quote {
    fn visit_refs(&self, _visitor: &mut dyn universe::RefVisitor) {
        loop {}
    }
}
