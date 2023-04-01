//! Lighting algorithms for `Space`. This module is closely tied to `Space`
//! and separated out for readability, not modularity.
use std::fmt;
use crate::block::EvaluatedBlock;
use crate::math::FaceMap;
use crate::util::{CustomFormat, StatusText};
