pub mod color;
pub mod geometry;
pub mod pipeline;

use std::sync::Arc;

use wgpu::{
    Color, Device, IndexFormat, Instance, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    wgt::{CommandEncoderDescriptor, DeviceDescriptor, TextureViewDescriptor},
};
use winit::window::Window;

use crate::renderer::pipeline::RendererPipeline;

pub struct GameRenderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RendererPipeline,
}

impl GameRenderer {
    pub async fn new(window: Window) -> anyhow::Result<Self> {
        let window = Arc::new(window);

        let instance = Instance::default();
        let surface = instance.create_surface(Arc::clone(&window))?;

        let adapter_options = RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        };
        let adapter = instance.request_adapter(&adapter_options).await?;
        log::debug!("Running on adapter: {:#?}", adapter.get_info());

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await?;
        let size = window.inner_size();
        let config = match surface.get_default_config(&adapter, size.width, size.height) {
            Some(config) => config,
            None => anyhow::bail!("failed to get default configuration for the window"),
        };

        surface.configure(&device, &config);
        log::debug!("Configured window's surface for the target device");

        let pipeline = RendererPipeline::new(&device, &config.format);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            pipeline,
        })
    }

    pub fn render(&self) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(value) => value,
            wgpu::CurrentSurfaceTexture::Suboptimal(value) => value,
            _ => {
                log::error!("Received an invalid texture from the surface");
                return;
            }
        };

        // We don't send the raw texture directly to the surface. Rather, we
        // create new texture view, and use it to render the frame.
        let texture_view = frame.texture.create_view(&TextureViewDescriptor::default());

        // Since GPU does not execute each command individually, we need a
        // command encoder (like a textbook).
        let descriptor = CommandEncoderDescriptor {
            label: Some("Renderer Command Encoder"),
        };
        let mut command_encoder = self.device.create_command_encoder(&descriptor);

        // Here we are beginning a new render pass. It's a set of commands for
        // the command encoder to produce a specific image on the surface using
        // available frame and a texture view.
        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Renderer Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,  // Texture view to draw the render pass on.
                    depth_slice: None,    // The depth slice for the 3D view.
                    resolve_target: None, // Anti-aliasing/multisampling setting.
                    ops: Operations {
                        // Tells wgpu to clear the frame with the given color.
                        // If frame isn't cleared, the previous texture / frame
                        // will be still present.
                        load: LoadOp::Clear(Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
                            a: 1.0,
                        }),
                        // Keep drawn pixels on the screen.
                        store: StoreOp::Store,
                    },
                })],
                // Allows GPU to select which triangle is closer.
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None, // Required for anti-aliasing/multisampling.
            });
            render_pass.set_pipeline(&self.pipeline.pipeline);
            // Add renderer pipeline's vertex buffer to the render pass.
            render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
            // Add renderer pipeline's indices buffer to the render pass.
            render_pass
                .set_index_buffer(self.pipeline.indices_buffer.slice(..), IndexFormat::Uint16);
            // Tell the render pass to draw indices from the vertex buffer from
            // 0 to the final one. Use offset = zero, and draw a single instance.
            render_pass.draw_indexed(0..self.pipeline.indices_num as u32, 0, 0..1);
        }

        // Submit commands, produced by render pass, to the driver's queue.
        self.queue.submit(std::iter::once(command_encoder.finish()));
        frame.present();
    }
}
