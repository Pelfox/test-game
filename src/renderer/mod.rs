//! This module implements actual game renderer, allowing to display 3D
//! graphics and objects inside the world on player's screen.

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

/// Represents game renderer itself.
pub struct GameRenderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RendererPipeline,

    /// Instance of the player's camera.
    pub camera: Camera,
}

impl GameRenderer {
    /// Creates a new game renderer for the given window.
    ///
    /// This method initializes support for an appropriate graphics backend and
    /// configures the window, creating a new [RendererPipeline] and a [Camera].
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

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Renderer Device"),
                ..Default::default()
            })
            .await?;
        let size = window.inner_size();
        let config = match surface.get_default_config(&adapter, size.width, size.height) {
            Some(config) => config,
            None => anyhow::bail!("failed to get default configuration for the window"),
        };

        surface.configure(&device, &config);
        log::debug!("Configured window's surface for the target device");

        let camera = Camera::new(
            Vec3d::new(0.0, 0.0, 0.0),
            config.width as f32 / config.height as f32,
            45.0f32.to_radians(),
        );

        let mut pipeline =
            RendererPipeline::new(&device, &camera, config.width, config.height, config.format);

        let mut cube_object = CubeObject::new();
        cube_object.with_transform(Transform {
            position: Vec3d::new(0.0, 0.25, -5.0),
            rotation: Vec3d::new(0.5, 0.5, 0.0),
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
            camera,
        })
    }

    // TODO: public API later on.
    // pub fn register_object(&mut self, values: (MeshId, Transform)) {
    //     self.pipeline.register_object(&self.device, values);
    // }

    /// Updates camera direction (pitch and yaw) with the given deltas.
    ///
    /// It is important to scale down (i.e. to apply a sensitivity delta) them,
    /// otherwise it will resolve in invalid values, cancelling out changes.
    ///
    /// This method does not automatically re-render the scene, so it is up to
    /// the caller to update it.
    pub fn update_camera_direction(&mut self, delta_x: f32, delta_y: f32) {
        let yaw = self.camera.yaw + delta_x;
        self.camera.yaw = yaw;

        let mut pitch = self.camera.pitch - delta_y;
        pitch = pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
        self.camera.pitch = pitch;

        self.pipeline
            .gpu_camera
            .schedule_update(&self.camera, &self.queue);
    }

    /// Performs rendering for a single frame, using currently available data.
    ///
    /// This method is expensive, and should be called only on significant
    /// changes on the scene. It is recommended that multiple scene updates are
    /// grouped together before calling this method, allowing for a better
    /// performance.
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
