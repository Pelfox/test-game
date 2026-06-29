use wgpu::{
    BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState,
    FrontFace, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    TextureFormat, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};

// TODO: REMOVE!!!
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

// Simple triangle
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2];

pub struct RendererPipeline {
    shader_module: ShaderModule,
    pub vertex_buffer: Buffer,
    pub indices_buffer: Buffer,
    pipeline_layout: PipelineLayout,
    pub pipeline: RenderPipeline,
    pub indices_num: usize,
}

impl RendererPipeline {
    pub fn new(device: &Device, surface_format: &TextureFormat) -> Self {
        // Loads shader at the given path into the wgpu.
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main Renderer Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        // Creates a vertex buffer that holds all vertices.
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Renderer Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        // Creates indices buffer that holds all indices to the vertices.
        let indices_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Renderer Indices Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        // Creates render pipeline layout, telling wgpu which external resources are needed.
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Renderer Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        // And finally, creating rendering pipeline itself.
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Renderer Pipeline"),
            layout: Some(&pipeline_layout),
            // Vertex shader state.
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"), // The name of the vertex shader entrypoint.
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()], // The vertex descriptor.
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
            depth_stencil: None,
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
            vertex_buffer,
            indices_buffer,
            pipeline_layout,
            pipeline,
            indices_num: INDICES.len(),
        }
    }
}
