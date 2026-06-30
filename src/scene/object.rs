use crate::{
    math::transform::Transform,
    renderer::{color::Color, mesh::MeshId},
    scene::material::Material,
};

/// Represents a single cube object.
pub struct CubeObject {
    transform: Transform,
    material: Material,
}

impl CubeObject {
    /// Creates a new cube object, setting its properties to default values.
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            material: Material::Color(Color::hex(0xFFFFFFFF)),
        }
    }

    /// Updates this cube's transform.
    pub fn with_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    /// Update this cube's material.
    pub fn with_material(&mut self, material: Material) {
        self.material = material;
    }

    /// Converts this object into parts that will create the final renderer
    /// object.
    pub fn into_object_parts(self) -> (MeshId, Transform) {
        (MeshId::Cube, self.transform)
    }
}
