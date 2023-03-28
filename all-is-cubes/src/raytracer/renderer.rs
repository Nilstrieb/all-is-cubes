use std::fmt;
use cgmath::{ElementWise, Point2, Vector2};
use futures_core::future::BoxFuture;
use image::RgbaImage;
use crate::camera::{
    Camera, Flaws, HeadlessRenderer, Layers, RenderError, StandardCameras, Viewport,
};
use crate::character::Cursor;
use crate::listen::ListenableSource;
use crate::raytracer::{
    PixelBuf, RaytraceInfo, RtBlockData, RtOptionsRef, SpaceRaytracer,
    UpdatingSpaceRaytracer,
};
fn trace_patch_in_one_space<P: PixelBuf>(
    space: &SpaceRaytracer<<P as PixelBuf>::BlockData>,
    camera: &Camera,
    patch: NdcRect,
    include_sky: bool,
) -> (P, RaytraceInfo) {
    loop {}
}
