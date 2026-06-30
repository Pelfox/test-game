//! This module implements meshing for objects and their respective vertices
//! and indices.

use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, Buffer, BufferAddress, BufferUsages, Device,
    IndexFormat, VertexAttribute,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    math::{transform::Transform, vector::Vec3d},
    scene::material::Material,
};

/// Describes the format for Vertex indices.
pub const INDICES_FORMAT: IndexFormat = IndexFormat::Uint16;

/// Represents a single vertex of an object.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Position of the vertex, relative to mesh local coordinate system.
    pub position: Vec3d,
    /// The direction of the normal axis for the vertex.
    pub normal: Vec3d,
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 2] = [
        VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: Vec3d::VERTEX_FORMAT,
        },
        VertexAttribute {
            offset: size_of::<Vec3d>() as BufferAddress,
            shader_location: 1,
            format: Vec3d::VERTEX_FORMAT,
        },
    ];

    /// Converts [Vertex] to the buffer layout, regardless of its contents.
    pub fn to_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        let vertex_size = size_of::<Self>() as BufferAddress;
        wgpu::VertexBufferLayout {
            array_stride: vertex_size,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    /// Creates a new vertex with the given position (in local coordinate
    /// space) and the normal.
    pub fn new(position: Vec3d, normal: Vec3d) -> Self {
        Self { position, normal }
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
    /// Creates a new mesh for the cube.
    ///
    /// This function creates an 24-vertex cube.
    pub fn cube() -> Self {
        let normal_front = Vec3d::new(0.0, 0.0, 1.0);
        let normal_back = Vec3d::new(0.0, 0.0, -1.0);
        let normal_left = Vec3d::new(-1.0, 0.0, 0.0);
        let normal_right = Vec3d::new(1.0, 0.0, 0.0);
        let normal_top = Vec3d::new(0.0, 1.0, 0.0);
        let normal_bottom = Vec3d::new(0.0, -1.0, 0.0);

        let vertices = vec![
            // front face, z = 1
            Vertex::new(Vec3d::new(-1.0, -1.0, 1.0), normal_front),
            Vertex::new(Vec3d::new(1.0, -1.0, 1.0), normal_front),
            Vertex::new(Vec3d::new(1.0, 1.0, 1.0), normal_front),
            Vertex::new(Vec3d::new(-1.0, 1.0, 1.0), normal_front),
            // back face, z = -1
            Vertex::new(Vec3d::new(1.0, -1.0, -1.0), normal_back),
            Vertex::new(Vec3d::new(-1.0, -1.0, -1.0), normal_back),
            Vertex::new(Vec3d::new(-1.0, 1.0, -1.0), normal_back),
            Vertex::new(Vec3d::new(1.0, 1.0, -1.0), normal_back),
            // right face, x = 1
            Vertex::new(Vec3d::new(1.0, -1.0, 1.0), normal_right),
            Vertex::new(Vec3d::new(1.0, -1.0, -1.0), normal_right),
            Vertex::new(Vec3d::new(1.0, 1.0, -1.0), normal_right),
            Vertex::new(Vec3d::new(1.0, 1.0, 1.0), normal_right),
            // left face, x = -1
            Vertex::new(Vec3d::new(-1.0, -1.0, -1.0), normal_left),
            Vertex::new(Vec3d::new(-1.0, -1.0, 1.0), normal_left),
            Vertex::new(Vec3d::new(-1.0, 1.0, 1.0), normal_left),
            Vertex::new(Vec3d::new(-1.0, 1.0, -1.0), normal_left),
            // top face, y = 1
            Vertex::new(Vec3d::new(-1.0, 1.0, 1.0), normal_top),
            Vertex::new(Vec3d::new(1.0, 1.0, 1.0), normal_top),
            Vertex::new(Vec3d::new(1.0, 1.0, -1.0), normal_top),
            Vertex::new(Vec3d::new(-1.0, 1.0, -1.0), normal_top),
            // bottom face, y = -1
            Vertex::new(Vec3d::new(-1.0, -1.0, -1.0), normal_bottom),
            Vertex::new(Vec3d::new(1.0, -1.0, -1.0), normal_bottom),
            Vertex::new(Vec3d::new(1.0, -1.0, 1.0), normal_bottom),
            Vertex::new(Vec3d::new(-1.0, -1.0, 1.0), normal_bottom),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, // front
            4, 5, 6, 4, 6, 7, // back
            8, 9, 10, 8, 10, 11, // right
            12, 13, 14, 12, 14, 15, // left
            16, 17, 18, 16, 18, 19, // top
            20, 21, 22, 20, 22, 23, // bottom
        ];
        Self {
            vertices,
            indices,
            label: "Cube",
        }
    }

    /// Creates a new mesh for the plane.
    pub fn plane() -> Self {
        // All vertices of the plane have the same normal since it is flat.
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let vertices = vec![
            Vertex::new(Vec3d::new(-1.0, 0.0, -1.0), normal),
            Vertex::new(Vec3d::new(1.0, 0.0, -1.0), normal),
            Vertex::new(Vec3d::new(1.0, 0.0, 1.0), normal),
            Vertex::new(Vec3d::new(-1.0, 0.0, 1.0), normal),
        ];
        let indices = vec![
            0, 1, 2, // first triangle
            0, 2, 3, // second triangle
        ];
        Self {
            vertices,
            indices,
            label: "Plane",
        }
    }
}

/// Describes the type for the unique mesh ID.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum MeshId {
    /// Representation of a cube.
    Cube,
    /// Representation of a plane.
    Plane,
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
    /// Object's texture material.
    pub material: Material,
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
        material: Material,
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let color_material = match material {
            Material::Color(color) => [color.r, color.g, color.b, color.a],
        };

        let uniform = ObjectUniform {
            model_matrix: transform.model_matrix().to_wgsl_matrix(),
            color_material,
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
            material,
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
    /// Value for the object's material color.
    pub color_material: [f32; 4],
}
