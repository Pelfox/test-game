//! This module implements renderer pipeline, enabling support for shaders, and
//! implementing the logic behind drawing objects in the world.

use std::collections::HashMap;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState,
    BufferBindingType, Color, ColorTargetState, ColorWrites, CommandEncoder, CompareFunction,
    DepthBiasState, DepthStencilState, Device, FragmentState, FrontFace, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderStages, StencilState, StoreOp,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, VertexState,
    wgt::TextureViewDescriptor,
};

use crate::{
    math::transform::Transform,
    renderer::{
        camera::GpuCamera,
        mesh::{INDICES_FORMAT, Mesh, MeshData, MeshId, Object, Vertex},
    },
    scene::material::Material,
};

/// Represents the return type of the user-level game object to be registered.
pub type ObjectParts = (MeshId, Transform, Material);

/// Initializes all available mesh types (from [MeshId]) with their respective
/// CPU-sided [MeshData] for each mesh type.
fn initialize_mesh_types(device: &Device) -> HashMap<MeshId, Mesh> {
    let mut meshes = HashMap::new();
    meshes.insert(MeshId::Cube, Mesh::from_data(device, MeshData::cube())); // Cube.
    meshes.insert(MeshId::Plane, Mesh::from_data(device, MeshData::plane())); // Plane.
    meshes
}

/// Represents a renderer pipeline.
pub struct RendererPipeline {
    depth_texture_view: TextureView,
    pipeline: RenderPipeline,
    objects: Vec<Object>,
    objects_bind_group_layout: BindGroupLayout,
    meshes: HashMap<MeshId, Mesh>,

    /// Representation of a GPU-sided player's camera.
    pub(crate) gpu_camera: GpuCamera,
}

impl RendererPipeline {
    /// Creates a new renderer pipeline for the given target device, camera and
    /// output (screen) parameters.
    pub fn new(
        device: &Device,
        camera: &crate::scene::camera::Camera,
        output_width: u32,
        output_height: u32,
        output_texture_format: TextureFormat,
    ) -> Self {
        // Loading shader at the given path into the wgpu.
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main Renderer Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        // Creating GPU-sided camera representation.
        let gpu_camera = GpuCamera::new(device, camera);

        // Creating bind group layout for drawable objects.
        let objects_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Objects Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Creating render pipeline layout, telling wgpu which external
        // resources are needed.
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Renderer Pipeline Layout"),
            bind_group_layouts: &[
                Some(&gpu_camera.bind_group_layout),
                Some(&objects_bind_group_layout),
            ],
            immediate_size: 0,
        });

        // Creating depth texture for renderer. This allows GPU to select which
        // triangle to draw in front and on the back.
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Renderer Depth Texture"),
            size: wgpu::Extent3d {
                width: output_width,
                height: output_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth24Plus, // TODO: Configure this, if needed.
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&TextureViewDescriptor::default());

        // Creating the pipeline itself.
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Renderer Main Pipeline"),
            layout: Some(&pipeline_layout),
            // Vertex shader state.
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"), // The name of the vertex shader entrypoint.
                compilation_options: Default::default(),
                buffers: &[Vertex::to_buffer_layout()], // The vertex descriptor.
            },
            // Fragment shader state.
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: output_texture_format,
                    blend: Some(BlendState::REPLACE), // Replace old pixel color with the new one.
                    write_mask: ColorWrites::ALL,     // Allow to write all channels.
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            // TODO: Read more about parameters.
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth24Plus,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::Less),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            depth_texture_view,
            pipeline,
            objects: Vec::new(),
            objects_bind_group_layout,
            meshes: initialize_mesh_types(device),
            gpu_camera,
        }
    }

    /// Registers a new object to be drawn.
    ///
    /// This method does not mutate current frame's objects, but it will queue
    /// the object to be drawn on the next frame.
    pub fn register_object(&mut self, device: &Device, parts: ObjectParts) {
        let object = Object::new(
            parts.0,
            device,
            parts.1,
            parts.2,
            &self.objects_bind_group_layout,
        );
        self.objects.push(object);
    }

    /// Creates a new render pass, processing all objects that are present on
    /// the scene, as well as camera changes.
    pub fn create_render_pass(
        &self,
        command_encoder: &mut CommandEncoder,
        texture_view: &TextureView,
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Renderer Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: texture_view,   // Texture view to draw the render pass on.
                depth_slice: None,    // The depth slice for the 3D view.
                resolve_target: None, // Anti-aliasing/multisampling setting.
                ops: Operations {
                    // Tells wgpu to clear the frame with the given color.
                    // If frame isn't cleared, the previous texture / frame
                    // will be still present.
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    // Keep drawn pixels on the screen.
                    store: StoreOp::Store,
                },
            })],
            // Allows GPU to determine which triangle is closer.
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None, // Required for anti-aliasing/multisampling.
        });
        render_pass.set_pipeline(&self.pipeline);

        // Adding camera's group to the render pass.
        render_pass.set_bind_group(0, &self.gpu_camera.bind_group, &[]);

        // Processing all scene's objects.
        for object in &self.objects {
            if let Some(mesh) = &self.meshes.get(&object.mesh_id) {
                // Adding object bind group, different per object.
                render_pass.set_bind_group(1, &object.bind_group, &[]);

                // Rendering this object.
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), INDICES_FORMAT);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);

                continue;
            }

            log::error!("Received an invalid mesh ID: {:?}", object.mesh_id);
        }
    }
}
