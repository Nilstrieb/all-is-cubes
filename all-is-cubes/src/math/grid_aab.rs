//! Axis-aligned integer-coordinate box volumes ([`GridAab`]), arrays bounded by them
//! ([`GridArray`]), and related.






/// An axis-aligned box with integer coordinates, whose volume is no larger than [`usize::MAX`].
/// [`GridAab`]s are used to specify the coordinate extent of [`Space`](crate::space::Space)s, and
/// regions within them.
///
/// When we refer to “a cube” in a [`GridAab`], that is a unit cube which is identified by the
/// integer coordinates of its most negative corner. Hence, coordinate bounds are always
/// half-open intervals: lower inclusive and upper exclusive. There are functions to help with
/// this such as [`cube_to_midpoint`](super::cube_to_midpoint).
///
/// A [`GridAab`] may have a zero-size range in any direction, thus making its total volume zero.
/// The different possibilities are not considered equal; thus, points, lines, and planes may be
/// represented, which may be useful for procedural-generation purposes.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct GridAab {}
