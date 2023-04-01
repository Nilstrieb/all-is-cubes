#![cfg_attr(
    not(feature = "arbitrary"),
    doc = "[`arbitrary::Arbitrary`]: https://docs.rs/arbitrary/1.0.2/arbitrary/trait.Arbitrary.html"
)]
#![cfg_attr(not(feature = "threads"), doc = "[`rayon`]: https://docs.rs/rayon/")]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_update)]
#![allow(clippy::single_match)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::exhaustive_structs)]
#![warn(clippy::modulo_arithmetic)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::uninlined_format_args)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::wrong_self_convention)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_lifetimes)]
#![cfg_attr(test, allow(clippy::float_cmp), allow(clippy::redundant_clone))]
#![warn(missing_docs)]
#[macro_use]
pub(crate) mod math;
pub(crate) mod behavior;
pub(crate) mod block;
pub(crate) mod camera;
pub(crate) mod character;
#[doc(hidden)]
pub(crate) mod chunking;
pub(crate) mod drawing;
pub(crate) mod inv;
pub(crate) mod linking;
pub(crate) mod listen;
pub(crate) mod mesh;
pub(crate) mod physics;
pub(crate) mod raycast;
pub(crate) mod space;
pub(crate) mod transaction;
pub(crate) mod universe;
pub(crate) mod util;
