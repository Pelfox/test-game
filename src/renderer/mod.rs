pub mod camera;
pub mod color;
pub mod mesh;
pub mod pipeline;

use std::sync::Arc;

use wgpu::{
    Device, Instance, Queue, RequestAdapterOptions, Surface,
    wgt::{CommandEncoderDescriptor, DeviceDescriptor, TextureViewDescriptor},
};
use winit::window::Window;

use crate::{
    math::{transform::Transform, vector::Vec3d},
    renderer::{color::Color, pipeline::RendererPipeline},
    scene::{
        camera::Camera,
        material::Material,
        object::{CubeObject, PlaneObject},
    },
};

pub struct GameRenderer {
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

        // TODO: This probably should be in the constructor arguments.
        let camera = Camera::new(
            Vec3d::new(0.0, 0.0, 0.0),
            Vec3d::new(0.0, 0.0, -1.0),
            Vec3d::new(0.0, 1.0, 0.0),
            config.width as f32 / config.height as f32,
            45.0f32.to_radians(),
        );

        let mut pipeline =
            RendererPipeline::new(&device, &camera, config.width, config.height, config.format);

        let mut cube_object = CubeObject::new();
        cube_object.with_transform(Transform {
            position: Vec3d::new(0.0, 0.5, -5.0),
            rotation: Vec3d::default(),
            scale: Vec3d::new(1.0, 1.0, 1.0),
        });
        cube_object.with_material(Material::Color(Color::hex(0xFFF22FFF)));
        pipeline.register_object(&device, cube_object.into_object_parts());

        let mut plane_object = PlaneObject::new();
        plane_object.with_transform(Transform {
            position: Vec3d::new(0.0, -1.0, 0.0),
            rotation: Vec3d::default(),
            scale: Vec3d::new(10.0, 1.0, 10.0),
        });
        plane_object.with_material(Material::Color(Color::hex(0xFFFFFFFF)));
        pipeline.register_object(&device, plane_object.into_object_parts());

        Ok(Self {
            surface,
            device,
            queue,
            pipeline,
        })
    }

    // TODO: public API later on.
    // pub fn register_object(&mut self, values: (MeshId, Transform)) {
    //     self.pipeline.register_object(&self.device, values);
    // }

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
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Renderer Command Encoder"),
            });

        // Here we are beginning a new render pass. It's a set of commands for
        // the command encoder to produce a specific image on the surface using
        // available frame and a texture view.
        self.pipeline
            .create_render_pass(&mut command_encoder, &texture_view);

        // Submit commands, produced by render pass, to the driver's queue.
        self.queue.submit(std::iter::once(command_encoder.finish()));
        frame.present();
    }
}
