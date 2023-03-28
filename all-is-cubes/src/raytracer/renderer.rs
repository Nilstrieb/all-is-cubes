use std::fmt;
use cgmath::{ElementWise, Point2, Vector2};
use futures_core::future::BoxFuture;
use image::RgbaImage;
use ordered_float::NotNan;
use crate::camera::{
    AntialiasingOption, Camera, Flaws, FogOption, GraphicsOptions, HeadlessRenderer,
    Layers, RenderError, StandardCameras, Viewport,
};
use crate::character::Cursor;
use crate::content::palette;
use crate::listen::ListenableSource;
use crate::math::Rgba;
use crate::raytracer::{
    ColorBuf, PixelBuf, RaytraceInfo, RtBlockData, RtOptionsRef, SpaceRaytracer,
    UpdatingSpaceRaytracer,
};
use crate::space::Space;
use crate::universe::URef;
/// Builds upon [`UpdatingSpaceRaytracer`] to make a complete [`HeadlessRenderer`],
/// following the scene and camera information in a [`StandardCameras`].
pub struct RtRenderer<D: RtBlockData = ()> {
    rts: Layers<Option<UpdatingSpaceRaytracer<D>>>,
    cameras: StandardCameras,
    /// Adjusts the `cameras` viewport to control how many pixels are actually traced.
    /// The output images will alway
    size_policy: Box<dyn Fn(Viewport) -> Viewport + Send + Sync>,
    custom_options: ListenableSource<D::Options>,
    /// Whether there was a [`Cursor`] to be drawn.
    /// Raytracing doesn't yet support cursors but we need to report that.
    had_cursor: bool,
}
impl<D: RtBlockData> RtRenderer<D>
where
    D::Options: Clone + Sync + 'static,
{
    /// * `cameras`: Scene to draw.
    /// * `size_policy`: Modifier to the `cameras`' provided viewport to control how many
    ///    pixels are actually traced.
    /// * `custom_options`: The custom options for the `D` block data type; see
    ///   [`RtBlockData`].
    pub fn new(
        cameras: StandardCameras,
        size_policy: Box<dyn Fn(Viewport) -> Viewport + Send + Sync>,
        custom_options: ListenableSource<D::Options>,
    ) -> Self {
        loop {}
    }
    /// Update the renderer's internal copy of the scene from the data sources
    /// (`URef<Character>` etc.) it is tracking.
    ///
    /// Returns [`RenderError::Read`] if said sources are in use.
    /// In that case, the renderer is still functional but will have stale data.
    ///
    /// This method is equivalent to [`HeadlessRenderer::update()`] except for
    /// fitting the raytracer's needs and capabilities (works with all types;
    /// not `async`).
    pub fn update(&mut self, cursor: Option<&Cursor>) -> Result<(), RenderError> {
        loop {}
    }
    /// Produce an image of the current state of the scene this renderer was created to
    /// track, as of the last call to [`Self::update()`], with the given overlaid text.
    ///
    /// The image's dimensions are determined by the previously supplied
    /// [`StandardCameras`]’ viewport value as of the last call to [`Self::update()`],
    /// as affected by the `size_policy`. The provided `output` buffer must have exactly
    /// that length.
    ///
    /// This operation does not attempt to access the scene objects and therefore may be
    /// called while the [`Universe`] is being stepped, etc.
    ///
    /// This method is equivalent to [`HeadlessRenderer::draw()`] except that it works
    /// with any [`PixelBuf`] instead of requiring [`ColorBuf`] and [`Rgba`] output,
    /// is not async, and does not require `&mut self`.
    ///
    /// [`Universe`]: crate::universe::Universe
    pub fn draw<P, E, O, IF>(
        &self,
        info_text_fn: IF,
        encoder: E,
        output: &mut [O],
    ) -> RaytraceInfo
    where
        P: PixelBuf<BlockData = D>,
        E: Fn(P) -> O + Send + Sync,
        O: Clone + Send + Sync,
        IF: FnOnce(&RaytraceInfo) -> String,
    {
        loop {}
    }
    /// Returns the [`StandardCameras`] this renderer contains.
    ///
    /// TODO: Should this be a standard part of [`HeadlessRenderer`] and/or other traits?
    /// It's likely to be useful for dealing with cursors and such matters, I think.
    pub fn cameras(&self) -> &StandardCameras {
        loop {}
    }
    /// Returns the [`Viewport`] as of the last [`Self::update()`] as modified by the
    /// `size_policy`. That is, this reports the size of images that will be actually
    /// drawn.
    pub fn modified_viewport(&self) -> Viewport {
        loop {}
    }
}
impl RtRenderer<()> {
    /// As [`Self::draw()`], but the output is an [`RgbaImage`], and
    /// [`Camera::post_process_color()`] is applied to the pixels.
    ///
    ///  [`Camera::post_process_color()`]: crate::camera::Camera::post_process_color
    pub fn draw_rgba(
        &self,
        info_text_fn: impl FnOnce(&RaytraceInfo) -> String,
    ) -> (RgbaImage, RaytraceInfo, Flaws) {
        loop {}
    }
}
impl<D: RtBlockData> fmt::Debug for RtRenderer<D>
where
    D::Options: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl HeadlessRenderer for RtRenderer<()> {
    fn update<'a>(
        &'a mut self,
        cursor: Option<&'a Cursor>,
    ) -> BoxFuture<'a, Result<(), RenderError>> {
        loop {}
    }
    fn draw<'a>(
        &'a mut self,
        info_text: &'a str,
    ) -> BoxFuture<'a, Result<(RgbaImage, Flaws), RenderError>> {
        loop {}
    }
}
/// Bundle of references to the current scene data in a [`RtRenderer`],
/// used to implement tracing individual rays independent of how they
/// are assembled into an image. Differs from [`SpaceRaytracer::trace_ray`]
/// in that it includes the cameras (thus accepting screen-space coordinates
/// rather than a lay) and [`Layers`] rather than one space.
struct RtScene<'a, P: PixelBuf> {
    rts: Layers<Option<&'a SpaceRaytracer<P::BlockData>>>,
    /// Cameras *with* size_policy applied.
    cameras: &'a Layers<Camera>,
    options: RtOptionsRef<'a, <P::BlockData as RtBlockData>::Options>,
}
impl<'a, P: PixelBuf> Clone for RtScene<'a, P> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<P: PixelBuf> Copy for RtScene<'_, P> {}
impl<P: PixelBuf> RtScene<'_, P> {
    /// Called from threaded or non-threaded `trace_scene_to_image()` implementations
    /// to produce a single image pixel.
    #[inline]
    fn trace_patch(&self, patch: NdcRect) -> (P, RaytraceInfo) {
        loop {}
    }
}
fn trace_patch_in_one_space<P: PixelBuf>(
    space: &SpaceRaytracer<<P as PixelBuf>::BlockData>,
    camera: &Camera,
    patch: NdcRect,
    include_sky: bool,
) -> (P, RaytraceInfo) {
    loop {}
}
/// A rectangle in normalized device coordinates (-1 to 1 is the viewport).
#[derive(Clone, Copy, Debug, PartialEq)]
struct NdcRect {
    low: Point2<f64>,
    high: Point2<f64>,
}
impl NdcRect {
    fn center(self) -> Point2<f64> {
        loop {}
    }
    fn point_within(self, uv: Vector2<f64>) -> Point2<f64> {
        loop {}
    }
}
/// Threaded and non-threaded implementations of generating a full image.
/// TODO: The design of this code (and its documentation) are slightly residual from
/// when `trace_scene_to_image()` was a public interface. Revisit them.
mod trace_image {
    use super::*;
    use crate::raytracer::{PixelBuf, RaytraceInfo};
    use cgmath::Point2;
    /// Compute a full image, writing it into `output`.
    ///
    /// The produced data is in the usual left-right then top-bottom raster order;
    /// its dimensions are `camera.framebuffer_size`.
    ///
    /// `encoder` may be used to transform the output of the `PixelBuf` into the stored
    /// representation.
    ///
    /// Panics if `output`'s length does not match the area of `camera.framebuffer_size`.
    ///
    /// TODO: Add a mechanism for incrementally rendering (not 100% of pixels) for
    /// interactive use.
    #[cfg(feature = "threads")]
    pub(super) fn trace_scene_to_image_impl<P, E, O>(
        scene: super::RtScene<'_, P>,
        encoder: E,
        output: &mut [O],
    ) -> RaytraceInfo
    where
        P: PixelBuf,
        E: Fn(P) -> O + Send + Sync,
        O: Send + Sync,
    {
        loop {}
    }
    /// Compute a full image, writing it into `output`.
    ///
    /// The produced data is in the usual left-right then top-bottom raster order;
    /// its dimensions are `camera.framebuffer_size`.
    ///
    /// `encoder` may be used to transform the output of the `PixelBuf` into the stored
    /// representation.
    ///
    /// Panics if `output`'s length does not match the area of `camera.framebuffer_size`.
    ///
    /// TODO: Add a mechanism for incrementally rendering (not 100% of pixels) for
    /// interactive use.
    #[cfg(not(feature = "threads"))]
    pub(super) fn trace_scene_to_image_impl<P, E, O>(
        scene: super::RtScene<'_, P>,
        encoder: E,
        output: &mut [O],
    ) -> RaytraceInfo
    where
        P: PixelBuf,
        E: Fn(P) -> O + Send + Sync,
        O: Send + Sync,
    {
        loop {}
    }
}
mod eg {
    use super::*;
    use crate::camera::info_text_drawable;
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::draw_target::DrawTargetExt;
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics::prelude::{OriginDimensions, Point, Size};
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics::Drawable;
    use embedded_graphics::Pixel;
    pub fn draw_info_text<T: Clone>(
        output: &mut [T],
        viewport: Viewport,
        paint: [T; 2],
        info_text: &str,
    ) {
        loop {}
    }
    /// Just enough [`DrawTarget`] to implement info text drawing.
    pub(crate) struct EgImageTarget<'a, T> {
        data: &'a mut [T],
        paint: [T; 2],
        size: Size,
    }
    impl<T: Clone> DrawTarget for EgImageTarget<'_, T> {
        type Color = BinaryColor;
        type Error = std::convert::Infallible;
        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            loop {}
        }
    }
    impl<T> OriginDimensions for EgImageTarget<'_, T> {
        fn size(&self) -> Size {
            loop {}
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::util::assert_send_sync;
    use super::*;
    #[test]
    fn renderer_is_send_sync() {
        loop {}
    }
}
