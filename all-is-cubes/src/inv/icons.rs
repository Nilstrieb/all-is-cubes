use std::fmt;
use cgmath::{ElementWise, EuclideanSpace as _, InnerSpace, Vector3};
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::{Drawable, Primitive};
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyleBuilder};
use exhaust::Exhaust;
use crate::block::{Block, BlockCollision, Resolution::*, AIR, AIR_EVALUATED};
use crate::content::load_image::{default_srgb, include_image, space_from_image};
use crate::drawing::VoxelBrush;
use crate::linking::{BlockModule, BlockProvider};
use crate::math::{
    cube_to_midpoint, Face7, FreeCoordinate, GridCoordinate, GridMatrix, GridPoint,
    GridRotation, GridVector, Rgb, Rgba,
};
use crate::space::Space;
use crate::universe::Universe;
#[cfg(doc)]
use crate::inv::Tool;
use crate::util::YieldProgress;
/// Blocks that are icons for [`Tool`]s.
///
/// TODO: Should this be considered strictly part of the UI/content and not fundamentals,
/// since it is making lots of aesthetic decisions?
/// If so, then [`Tool::icon()`] needs to go away, and the UI will need to either contain
/// these icons or accept them as configuration.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Exhaust)]
#[non_exhaustive]
pub enum Icons {
    /// Icon for an empty toolbar slot.
    EmptySlot,
    /// Icon for [`Tool::Activate`],
    Activate,
    /// Icon for [`Tool::RemoveBlock`].
    Delete,
    /// Icon for [`Tool::CopyFromSpace`].
    CopyFromSpace,
    /// Icon for [`Tool::EditBlock`].
    EditBlock,
    /// Icon for [`Tool::PushPull`].
    PushPull,
    /// Icon for [`Tool::Jetpack`].
    Jetpack {
        /// Actually flying?
        active: bool,
    },
}
impl BlockModule for Icons {
    fn namespace() -> &'static str {
        loop {}
    }
}
impl fmt::Display for Icons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Icons {
    /// Construct the standard icons, inserting block definitions into the given [`Universe`].
    ///
    /// TODO: Replace `&mut Universe` parameter with a transaction return value.
    pub async fn new(universe: &mut Universe, p: YieldProgress) -> BlockProvider<Icons> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn icons_smoke_test() {
        loop {}
    }
}
