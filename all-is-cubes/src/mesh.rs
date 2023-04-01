//! Data structures for triangle meshes and algorithms for converting blocks/voxels to
//! meshes for rendering.
//!
//! All of the algorithms here are independent of graphics API, but they require providing
//! vertex and texture data types suitable for the API or data format you wish to use.
#[doc(hidden)]
pub(crate) mod chunked_mesh;
