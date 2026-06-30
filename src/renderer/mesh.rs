//! This module implements meshing for objects and their respective vertices
//! and indices.

use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, Buffer, BufferAddress, BufferUsages, Device,
    IndexFormat, VertexAttribute,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::math::{transform::Transform, vector::Vec3d};

/// Describes the format for Vertex indices.
pub const INDICES_FORMAT: IndexFormat = IndexFormat::Uint16;

/// Represents a single vertex of an object.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Position of the vertex, relative to mesh local coordinate system.
    pub position: Vec3d,
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 1] = [VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: Vec3d::VERTEX_FORMAT,
    }];

    /// Converts [Vertex] to the buffer layout, regardless of its contents.
    pub fn to_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        let vertex_size = size_of::<Self>() as BufferAddress;
        wgpu::VertexBufferLayout {
            array_stride: vertex_size,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    /// Creates a new vertex with the given position (in local coordinate space).
    pub fn new(position: Vec3d) -> Self {
        Self { position }
    }
}

/// CPU-side geometry data for a mesh shape, such as a cube, plane, or sphere.
pub struct MeshData {
    /// Vertices of the current mesh.
    pub vertices: Vec<Vertex>,
    /// Indices for the vertices of the current mesh.
    pub indices: Vec<u16>,
    /// The label of the current container, used for debugging.
    pub label: &'static str,
}

impl MeshData {
    /// Creates a new mesh for the cube with the given color.
    ///
    /// This function creates an 8-vertex cube.
    pub fn cube() -> Self {
        let vertices = vec![
            // front face corners, z = 1
            Vertex::new(Vec3d::new(-1.0, -1.0, 1.0)),
            Vertex::new(Vec3d::new(1.0, -1.0, 1.0)),
            Vertex::new(Vec3d::new(1.0, 1.0, 1.0)),
            Vertex::new(Vec3d::new(-1.0, 1.0, 1.0)),
            // back face corners, z = -1
            Vertex::new(Vec3d::new(-1.0, -1.0, -1.0)),
            Vertex::new(Vec3d::new(1.0, -1.0, -1.0)),
            Vertex::new(Vec3d::new(1.0, 1.0, -1.0)),
            Vertex::new(Vec3d::new(-1.0, 1.0, -1.0)),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, // front
            1, 5, 6, 1, 6, 2, // right
            5, 4, 7, 5, 7, 6, // back
            4, 0, 3, 4, 3, 7, // left
            3, 2, 6, 3, 6, 7, // top
            4, 5, 1, 4, 1, 0, // bottom
        ];
        Self {
            vertices,
            indices,
            label: "Cube",
        }
    }
}

/// Describes the type for the unique mesh ID.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum MeshId {
    /// Representation of a cube.
    Cube,
}

/// GPU-sided mesh, with ready to write buffers.
pub struct Mesh {
    /// Buffer for all [MeshData]'s vertices.
    pub vertex_buffer: Buffer,
    /// Buffer for all [MeshData]'s indices.
    pub index_buffer: Buffer,
    /// Total amount of indices from the [MeshData].
    pub index_count: u32,
}

impl Mesh {
    /// Creates a new GPU-sided mesh for the given device and mesh data from
    /// the CPU, and prepares all buffers for a write.
    pub fn from_data(device: &wgpu::Device, data: MeshData) -> Self {
        let vertex_label = format!("{}'s Vertex Buffer", data.label);
        let index_label = format!("{}'s Index Buffer", data.label);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(vertex_label.as_str()),
            usage: BufferUsages::VERTEX,
            contents: cast_slice(&data.vertices),
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(index_label.as_str()),
            usage: BufferUsages::INDEX,
            contents: cast_slice(&data.indices),
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: data.indices.len() as u32,
        }
    }
}

/// Describes a single object that must be drawn.
pub struct Object {
    /// The ID for the mesh that this object is connected to.
    pub mesh_id: MeshId,
    /// Transformation that must be applied to the object.
    pub transform: Transform,
    /// Object's uniform to be sent to GPU.
    pub uniform: ObjectUniform,
    /// GPU buffer for the object.
    pub buffer: Buffer,
    /// Object's binding group.
    pub bind_group: BindGroup,
}

impl Object {
    /// Creates a new object with the given properties.
    pub fn new(
        mesh_id: MeshId,
        device: &Device,
        transform: Transform,
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let uniform = ObjectUniform {
            model_matrix: transform.model_matrix().to_wgsl_matrix(),
        };

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Object Buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Object Bind Group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            mesh_id,
            transform,
            uniform,
            buffer,
            bind_group,
        }
    }
}

/// Represents a single object's uniform for the shader.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectUniform {
    /// Model matrix of the object's mesh.
    pub model_matrix: [[f32; 4]; 4],
}
