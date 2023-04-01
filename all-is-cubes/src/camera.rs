//! Projection and view matrices, viewport and aspect ratio, visibility,
//! raycasting into the scene, etc.

use crate::math::{FreeCoordinate, GridAab};

use cgmath::{Basis3, Decomposed, Matrix4, Point3, Vector3};

mod flaws;
pub(crate) use flaws::*;
mod graphics_options;
pub(crate) use graphics_options::*;
type M = Matrix4<FreeCoordinate>;
/// Representation of a camera viewpoint and orientation, using [`cgmath`] types.
///
/// Note that this is treated as a transform **from** the origin looking in the &minus;Z
/// direction **to** the camera position in the world. This is done so that the
/// [`Decomposed::disp`] vector is equal to the world position, rather than needing to
/// be rotated by the view direction.
pub(crate) type ViewTransform = Decomposed<Vector3<FreeCoordinate>, Basis3<FreeCoordinate>>;
/// Defines a viewpoint in/of the world: a viewport (aspect ratio), projection matrix,
/// and view matrix.
///
/// See also [`StandardCameras`], which adds self-updating from a character’s viewport,
/// among other features.
#[derive(Clone, Debug)]
pub struct Camera {}
/// Viewport dimensions for rendering and UI layout with the correct resolution and
/// aspect ratio.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Viewport {}
/// Calculate an “eye position” (camera position) to view the entire given `bounds`.
///
/// `direction` points in the direction the camera should be relative to the space.
///
/// TODO: This function does not yet consider the effects of field-of-view,
/// and it will need additional parameters to do so.
pub(crate) fn eye_for_look_at(
    bounds: GridAab,
    direction: Vector3<FreeCoordinate>,
) -> Point3<FreeCoordinate> {
    loop {}
}
/// A view frustum, represented by its corner points.
/// This is an underconstrained representation, but one that is useful to precompute.
#[derive(Clone, Copy, Debug, PartialEq)]
struct FrustumPoints {}
