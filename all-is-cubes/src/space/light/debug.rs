//! Data structures and helper traits for getting detailed debug info
//! out of the lighting algorithm.
//!
//! Note that this entire module is `doc(hidden)`; pub items inside it
//! are for intra-project use only.
use cgmath::Vector3;
use crate::math::{FreeCoordinate, Geometry, GridPoint};
use crate::mesh::LineVertex;
use crate::raycast::Ray;
use crate::space::PackedLight;
