//! This module represents and implements mathematical vectors.

/// Represents a three-dimensional vector.
///
/// All methods of this structure must be immutable to the current instance.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
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

    /// Reverses the direction of the current three-dinmensional vector.
    pub fn reverse(&self) -> Self {
        Self::new(self.x * -1.0, self.y * -1.0, self.z * -1.0)
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

    /// Concatenates two given three-dimensional vectors.
    pub fn add(&self, another: &Self) -> Self {
        Self::new(self.x + another.x, self.y + another.y, self.z + another.z)
    }

    /// Multiplies current vector with the given number.
    pub fn mul_num(&self, num: f32) -> Self {
        Self::new(self.x * num, self.y * num, self.z * num)
    }
}
