use wgpu::{BufferAddress, VertexAttribute};

/// Represents a three-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3d {
    /// The vector component along the X axis.
    pub x: f32,
    /// The vector component along the Y axis.
    pub y: f32,
    /// The vector component along the Z axis.
    pub z: f32,
}

impl Vec3d {
    /// Describes the format of the structure as a vertex, allowing to create
    /// vertex attributes with this structure.
    pub const VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;

    /// Creates a new three-dimensional vector with the given coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Represents a single vertex of an object.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Position of the vertex, relative to mesh local coordinate system.
    pub position: Vec3d,
    /// Color of the vertex.
    pub color: super::color::Color,
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
            format: super::color::Color::VERTEX_FORMAT,
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

    /// Creates a new vertex with the given position and color.
    pub fn new(position: Vec3d, color: super::color::Color) -> Self {
        Self { position, color }
    }
}
