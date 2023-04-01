//! Algorithms for collision detection with [`Space`](crate::space::Space)s.
use std::fmt;
use cgmath::Vector3;
use crate::block::Evoxels;
use crate::block::{BlockCollision, Resolution};
use crate::math::{Aab, Face7, Geometry, GridAab, GridCoordinate, GridPoint};
use crate::mesh::LineVertex;
use crate::raycast::{Ray, Raycaster};
