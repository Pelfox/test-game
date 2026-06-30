use wgpu::{
    BindGroup, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer,
    BufferBindingType, BufferUsages, ColorTargetState, ColorWrites, CompareFunction,
    DepthBiasState, DepthStencilState, Device, FragmentState, FrontFace, MultisampleState,
    PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderStages, StencilState, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::TextureViewDescriptor,
};

use crate::{
    camera::{Camera, CameraUniform},
    math::vector::Vec3d,
    renderer::{
        color::Color,
        mesh::{Mesh, MeshData, Object, Transform, Vertex},
    },
};

pub struct RendererPipeline {
    shader_module: ShaderModule,
    pub camera_buffer: Buffer,
    pub camera: Camera,
    pub camera_bind_group: BindGroup,
    pub depth_texture: Texture,
    pub depth_texture_view: TextureView,
    pipeline_layout: PipelineLayout,
    pub pipeline: RenderPipeline,

    pub objects: Vec<Object>,
    pub meshes: Vec<Mesh>,
}

impl RendererPipeline {
    pub fn new(device: &Device, surface_format: &TextureFormat, width: u32, height: u32) -> Self {
        // Loads shader at the given path into the wgpu.
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main Renderer Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        let objects_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Renderer Objects Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let mut meshes = Vec::new();

        let cube_mesh_data = MeshData::cube(Color::hex(0xFFFFFFFF));
        let (cube_mesh_id, cube_mesh) = Mesh::from_data(&device, &cube_mesh_data);
        meshes.insert(cube_mesh_id, cube_mesh);

        let objects = vec![
            // Object::new(
            //     cube_mesh_id,
            //     &device,
            //     Transform::default(),
            //     &objects_bind_group_layout,
            // ),
            Object::new(
                cube_mesh_id,
                &device,
                Transform {
                    position: Vec3d::new(3.0, 0.0, 0.0),
                    rotation: Vec3d::default(),
                    scale: Vec3d::new(0.75, 0.75, 0.75),
                },
                &objects_bind_group_layout,
            ),
            Object::new(
                cube_mesh_id,
                &device,
                Transform {
                    position: Vec3d::new(-1.0, 0.75, 0.0),
                    rotation: Vec3d::default(),
                    scale: Vec3d::new(0.5, 0.9, 1.0),
                },
                &objects_bind_group_layout,
            ),
        ];

        let camera = Camera {
            eye: Vec3d::new(2.0, 2.0, 5.0),
            direction: Vec3d::new(-2.0, -2.0, -5.0),
            up_direction: Vec3d::new(0.0, 1.0, 0.0),
            aspect_ratio: width as f32 / height as f32,
            fov: 45.0_f32.to_radians(),
            near: 0.1,
            far: 100.0,
        };

        let camera_uniform = CameraUniform::from_camera(&camera);

        // Creates GPU buffer for the camera.
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Renderer Camera Buffer"),
            contents: bytemuck::bytes_of(&camera_uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Renderer Camera Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Renderer Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Creates render pipeline layout, telling wgpu which external resources are needed.
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Renderer Pipeline Layout"),
            bind_group_layouts: &[
                Some(&camera_bind_group_layout),
                Some(&objects_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Rednerer Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth24Plus,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&TextureViewDescriptor::default());

        // And finally, creating rendering pipeline itself.
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Renderer Pipeline"),
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
                    format: *surface_format,
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
            shader_module,
            camera,
            camera_buffer,
            camera_bind_group,
            pipeline_layout,
            pipeline,
            depth_texture,
            depth_texture_view,
            objects,
            meshes,
        }
    }
}
