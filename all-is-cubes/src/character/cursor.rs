//! [`Cursor`] type and related items.
//!
//! TODO: It's unclear what the scope of this module should be.






/// Data collected by [`cursor_raycast`] about the blocks struck by the ray; intended to be
/// sufficient for various player interactions with blocks.
///
/// TODO: Should carry information about both the struck and preceding cubes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cursor {}
