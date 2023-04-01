//! Tools that we could imagine being in the Rust standard library, but aren't.
use std::fmt::{self, Debug};
/// Generic extension to [`std::fmt`'s set of formatting traits](std::fmt#formatting-traits).
///
/// This can be thought of as a mechanism to easily create a new special-purpose
/// formatting trait, analogous to [`std::fmt::LowerHex`] or [`std::fmt::Pointer`].
/// Instead of implementing the entire necessary wrapper and setup code, implementations
/// need only be types representing the choice of formatting (e.g. [`ConciseDebug`]),
/// and [`CustomFormat::custom_format`] provides a wrapper which may be used to cause
/// a value implementing `CustomFormat<T>` to be formatted using
/// [`CustomFormat<T>::fmt`](Self::fmt).
pub(crate) trait CustomFormat<F: Copy> {
    /// Wrap this value so that when formatted with [`Debug`] or [`Display`] it uses
    /// the given custom format instead.
    fn custom_format(&self, format_type: F) -> CustomFormatWrapper<'_, F, Self> {
        loop {}
    }
    /// Implement this to provide custom formatting for this type.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>, format_type: F) -> fmt::Result;
}
/// You can use [`CustomFormat::custom_format`] to construct this.
/// See its documentation.
///
/// To enable using the wrapper inside [`assert_eq`], it implements [`PartialEq`]
/// (comparing both value and format).
#[derive(Eq, PartialEq)]
pub(crate) struct CustomFormatWrapper<'a, F: Copy, T: CustomFormat<F> + ?Sized>(
    F,
    &'a T,
);
/// Format type for [`CustomFormat`] which provides an highly condensed, ideally
/// constant-size, user-facing format for live-updating textual status messages.
/// This format does not follow Rust [`Debug`](fmt::Debug) syntax, and when implemented
/// for standard Rust types may have quirks. Values may have multiple lines.
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct StatusText;
/// Aggregation of the time taken by a set of events.
///
/// TODO: Consider including an identifier for the longest.
/// TODO: Consider generalizing this to quantities other than time? Probably not.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub(crate) struct TimeStats {}
