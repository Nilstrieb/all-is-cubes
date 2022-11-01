use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser as _;
use futures::future::BoxFuture;
use image::RgbaImage;
use tokio::sync::OnceCell;

use all_is_cubes::apps::StandardCameras;
use all_is_cubes::camera::{Flaws, HeadlessRenderer, RenderError, Viewport};
use all_is_cubes::character::Cursor;
use all_is_cubes::listen::{DirtyFlag, ListenableSource};
use all_is_cubes_gpu::in_wgpu::{init, EverythingRenderer};
use all_is_cubes_gpu::{FrameBudget, GraphicsResourceError};
use test_renderers::{RendererFactory, RendererId};

#[tokio::main]
pub async fn main() -> test_renderers::HarnessResult {
    test_renderers::initialize_logging();

    let (_instance, adapter) = init::create_instance_and_adapter_for_test().await;
    if let Some(adapter) = adapter {
        WGPU_ADAPTER.set(Arc::new(adapter)).unwrap();
    } else {
        eprintln!("Skipping rendering tests due to lack of wgpu::Adapter.");
        return ExitCode::SUCCESS;
    };

    test_renderers::harness_main(
        test_renderers::HarnessArgs::parse(),
        RendererId::Wgpu,
        test_renderers::test_cases::all_tests,
        get_factory,
    )
    .await
}

/// We don't share the [`wgpu::Device`] because it can enter failure states,
/// but we can use just one [`wgpu::Adapter`] to create all of them.
/// TODO: Should we bother not making this global, but threading it through
/// the test harness? Probably, in the form of some `impl TestRenderer`.
static WGPU_ADAPTER: OnceCell<Arc<wgpu::Adapter>> = OnceCell::const_new();

async fn get_factory() -> WgpuFactory {
    let adapter: &Arc<wgpu::Adapter> = WGPU_ADAPTER
        .get()
        .expect("Called get_device() without initializing WGPU_ADAPTER");
    let (device, queue) = adapter
        .request_device(&EverythingRenderer::device_descriptor(), None)
        .await
        .expect("Adapter::request_device() failed");
    WgpuFactory {
        device: Arc::new(device),
        queue: Arc::new(queue),
        adapter: Arc::clone(adapter),
    }
}

#[derive(Clone, Debug)]
struct WgpuFactory {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    adapter: Arc<wgpu::Adapter>,
}

impl RendererFactory for WgpuFactory {
    fn renderer_from_cameras(&self, cameras: StandardCameras) -> Box<dyn HeadlessRenderer + Send> {
        let viewport_source = cameras.viewport_source();
        let everything = EverythingRenderer::new(
            self.device.clone(),
            cameras,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &self.adapter,
        );

        let viewport_dirty = DirtyFlag::listening(false, |l| viewport_source.listen(l));
        let viewport = viewport_source.snapshot();
        let color_texture = create_color_texture(&self.device, viewport);

        Box::new(WgpuHeadlessRenderer {
            factory: self.clone(),
            color_texture,
            everything,
            viewport_source,
            viewport_dirty,
        })
    }

    fn id(&self) -> RendererId {
        RendererId::Wgpu
    }
}

struct WgpuHeadlessRenderer {
    // factory provides Device and Queue
    factory: WgpuFactory,
    color_texture: wgpu::Texture,
    everything: EverythingRenderer,
    viewport_source: ListenableSource<Viewport>,
    viewport_dirty: DirtyFlag,
}

impl HeadlessRenderer for WgpuHeadlessRenderer {
    fn update<'a>(
        &'a mut self,
        cursor: Option<&'a Cursor>,
    ) -> BoxFuture<'a, Result<(), RenderError>> {
        Box::pin(async move {
            self.everything
                .update(
                    &self.factory.queue,
                    cursor,
                    &FrameBudget::PRACTICALLY_INFINITE,
                )
                .map_err(GraphicsResourceError::into_render_error_or_panic)?;
            Ok(())
        })
    }

    fn draw<'a>(
        &'a mut self,
        info_text: &'a str,
    ) -> BoxFuture<'a, Result<(RgbaImage, Flaws), RenderError>> {
        let viewport = self.viewport_source.snapshot();
        if self.viewport_dirty.get_and_clear() {
            self.color_texture = create_color_texture(&self.factory.device, viewport);
        }

        Box::pin(async move {
            let _dinfo = self
                .everything
                .draw_frame_linear(&self.factory.queue)
                .unwrap();
            self.everything.add_info_text_and_postprocess(
                &self.factory.queue,
                &self.color_texture,
                info_text,
            );
            let image = init::get_pixels_from_gpu(
                &self.factory.device,
                &self.factory.queue,
                &self.color_texture,
                viewport,
            )
            .await;
            Ok((image, Flaws::default()))
        })
    }
}

fn create_color_texture(device: &wgpu::Device, viewport: Viewport) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("WgpuHeadlessRenderer::color_texture"),
        size: wgpu::Extent3d {
            width: viewport.framebuffer_size.x.max(1),
            height: viewport.framebuffer_size.y.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
    })
}
