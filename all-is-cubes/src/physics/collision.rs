//! Algorithms for collision detection with [`Space`](crate::space::Space)s.
use std::fmt;
use cgmath::Vector3;
use crate::block::Evoxels;
use crate::block::{BlockCollision, Resolution};
use crate::math::{Aab, Face7, Geometry, GridAab, GridCoordinate, GridPoint};
use crate::mesh::LineVertex;
use crate::raycast::{Ray, Raycaster};
/// An individual collision contact; something in a [`Space`] that a moving [`Aab`]
/// collided with.
///
/// This type is designed to be comparable/hashable to deduplicate contacts.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum Contact {
    /// Contact with a fully solid block; the [`CubeFace`] specifies the block position
    /// and the side of it that was collided with (hence also the contact normal).
    Block(),
    /// Contact with one voxel of a block with a potentially complex shape.
    Voxel {},
}
impl Contact {
    /// Returns the cube that was collided with or within.
    pub(crate) fn cube(&self) -> GridPoint {
        loop {}
    }
    /// Returns the contact normal: the direction in which the colliding box should be
    /// pushed back.
    ///
    /// Note that this may be equal to [`Face7::Within`] in case the box was already
    /// intersecting before any movement.
    pub(crate) fn normal(&self) -> Face7 {
        loop {}
    }
    /// Returns the scale of the voxel collided with.
    pub(crate) fn resolution(&self) -> Resolution {
        loop {}
    }
    /// Return a copy where the contact normal is replaced with [`Face7::Within`].
    fn without_normal(&self) -> Self {
        loop {}
    }
}
impl fmt::Debug for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Geometry for Contact {
    type Coord = GridCoordinate;
    fn translate(mut self, offset: Vector3<Self::Coord>) -> Self {
        loop {}
    }
    fn wireframe_points<E>(&self, output: &mut E)
    where
        E: Extend<LineVertex>,
    {
        loop {}
    }
}
/// Result of [`collide_along_ray`] which specifies a collision point possibly inside the cube.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CollisionRayEnd {}
/// Specifies the ending condition for [`collide_along_ray()`]: what type of situation
/// it should stop prior to the end of the ray for.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum StopAt {
    /// Stop on any collisions, including those at the starting position (already
    /// colliding).
    Anything,
    /// Stop when something new is collided with (excluding the starting position).
    NotAlreadyColliding,
    /// Stop at the first position where nothing is being collided with.
    #[allow(dead_code)]
    EmptySpace,
}
impl StopAt {
    fn reversed(self) -> bool {
        loop {}
    }
}
/// Abstraction over voxel arrays that the collision detection algorithm can use,
/// i.e. [`Space`] and `GridArray<Evoxel>`.
pub(crate) trait CollisionSpace {
    type Cell;
    /// Bounds outside of which there is definitely nothing that collides.
    fn bounds(&self) -> GridAab;
    /// Retrieve a cell value from the grid.
    /// Should return a non-colliding value if the point is out of bounds.
    fn get_cell(&self, cube: GridPoint) -> &Self::Cell;
    /// Retrieve a cell's collision behavior option.
    fn collision(cell: &Self::Cell) -> BlockCollision;
    /// TODO: document
    fn get_voxels(cell: &Self::Cell) -> Option<&Evoxels>;
    /// TODO: document
    ///
    /// * `entry_end`: the endpoint we would return if the recursion stopped here — entering the given cell.
    fn recurse(
        entry_end: CollisionRayEnd,
        aab: Aab,
        ray: Ray,
        cell: &Self::Cell,
        stop_at: StopAt,
    ) -> Option<CollisionRayEnd>;
}
