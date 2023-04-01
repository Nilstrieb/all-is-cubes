//! Algorithms for collision detection with [`Space`](crate::space::Space)s.
use std::fmt;
use cgmath::Vector3;
use crate::block::Evoxels;
use crate::block::{BlockCollision, Resolution};
use crate::math::{
    Aab, CubeFace, Face7, FreeCoordinate, Geometry, GridAab, GridCoordinate,
    GridPoint,
};
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
    Block(CubeFace),
    /// Contact with one voxel of a block with a potentially complex shape.
    Voxel {
        /// The “outer” cube in the [`Space`].
        cube: GridPoint,
        /// The voxel resolution of the block; that is, the factor by which voxel
        /// coordinates are smaller than `cube` coordinates.
        resolution: Resolution,
        /// The voxel position in the block (each coordinate lies between `0` and
        /// `resolution - 1`) and the face of that voxel collided with, which is also
        /// the contact normal.
        voxel: CubeFace,
    },
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
pub(crate) struct CollisionRayEnd {
    /// Non-colliding length of the provided ray.
    pub(crate) t_distance: FreeCoordinate,
    pub(crate) contact: Contact,
}
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
/// Move `aab`'s origin along the line segment from `ray.origin` to `ray.origin + ray.direction`,
/// and find the first point at which it collides with `space`'s collidable blocks.
///
/// The return value specifies the distance achieved and the normal (face) of the surface collided
///  with; if [`None`], then no obstacles were met along the full length of the line segment.
///
/// `collision_callback` is called once for each colliding cube — any one of them would have been
/// sufficient to stop the ray, but all are reported.
pub(crate) fn collide_along_ray<Sp, CC>(
    space: &Sp,
    ray: Ray,
    aab: Aab,
    mut collision_callback: CC,
    stop_at: StopAt,
) -> Option<CollisionRayEnd>
where
    Sp: CollisionSpace,
    CC: FnMut(Contact),
{
    loop {}
}
/// Returns an iterator over all blocks in `space` which intersect `aab`, accounting for
/// collision options.
pub(crate) fn find_colliding_cubes<Sp>(
    space: &Sp,
    aab: Aab,
) -> impl Iterator<Item = GridPoint> + '_
where
    Sp: CollisionSpace,
{
    let mut points = Vec::new();
    collide_along_ray(
        space,
        Ray::new([0., 0., 0.], [0., 0., 0.]),
        aab,
        |contact| {
            points.push(contact.cube());
        },
        StopAt::Anything,
    );
    points.into_iter()
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
/// Given a ray describing movement of the origin of an AAB, perform a raycast to find
/// the positions where the AAB moves into new cubes. The returned ray steps' `t_distance`
/// values report how far to move the AAB to meet the edge.
///
/// If `reversed` is true, find positions where it leaves cubes.
///
/// Note that due to the nature of floating-point arithmetic, it is uncertain whether the
/// resulting new AAB position will have the AAB's forward face land before, after, or
/// exactly on the boundary. The caller must compute an appropriate nudge (using TODO:
/// provide a function for this) to serve its needs.
pub(crate) fn aab_raycast(aab: Aab, origin_ray: Ray, reversed: bool) -> Raycaster {
    loop {}
}
/// Given an [`Aab`] and a [`Ray`] such that the given face of
/// `aab.translate(segment.unit_endpoint().to_vec())`
/// lands almost but not quite on a unit cell boundary along the `plane` axis,
/// nudge the endpoint of the ray
/// infinitesimally so that it lands definitely beyond (or before if `backward`)
/// the cell boundary.
///
/// Note that the required `face` is the opposite of the face produced by a raycast.
/// (This may be confusing but we feel that it would be more confusing to use a face
/// other than the relevant face of the [`Aab`]).
pub(crate) fn nudge_on_ray(
    aab: Aab,
    segment: Ray,
    face: Face7,
    subdivision: Resolution,
    backward: bool,
) -> Ray {
    loop {}
}
