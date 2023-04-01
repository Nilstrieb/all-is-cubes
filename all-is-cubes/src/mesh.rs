//! Data structures for triangle meshes and algorithms for converting blocks/voxels to
//! meshes for rendering.
//!
//! All of the algorithms here are independent of graphics API, but they require providing
//! vertex and texture data types suitable for the API or data format you wish to use.
mod block_vertex;
pub(crate) use block_vertex::*;
mod block_mesh;
pub(crate) use block_mesh::*;
#[doc(hidden)]
pub(crate) mod chunked_mesh;
mod space_mesh;

mod texalloc;

