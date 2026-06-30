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

    /// Normalizes the current three-dimensional vector.
    pub fn normalize(&self) -> Self {
        let length = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Self::new(self.x / length, self.y / length, self.z / length)
    }

    /// Crosses two three-dimensional vectors, creating a perpendicular one as
    /// a cross product.
    pub fn cross(&self, another: &Self) -> Self {
        let x = (self.y * another.z) - (self.z * another.y);
        let y = (self.z * another.x) - (self.x * another.z);
        let z = (self.x * another.y) - (self.y * another.x);
        Self::new(x, y, z)
    }

    /// Produces a dot product of the current three-dimenstional vector and a
    /// supplied one.
    pub fn dot(&self, another: &Self) -> f32 {
        (self.x * another.x) + (self.y * another.y) + (self.z * another.z)
    }
}

/// Represents a four-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec4d {
    /// The vector component along the X axis.
    pub x: f32,
    /// The vector component along the Y axis.
    pub y: f32,
    /// The vector component along the Z axis.
    pub z: f32,
    /// The vector component along the W axis.
    pub w: f32,
}

impl Vec4d {
    /// Describes the format of the structure as a vertex, allowing to create
    /// vertex attributes with this structure.
    pub const VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;

    /// Creates a new four-dimensional vector from the given components.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4d {
        Self { x, y, z, w }
    }
}
