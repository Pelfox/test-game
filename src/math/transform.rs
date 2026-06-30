use crate::math::{matrix::Matrix4x4, vector::Vec3d};

/// Describes mesh transformation.
pub struct Transform {
    /// Position component of the transformation.
    pub position: Vec3d,
    /// Rotation component of the transformation.
    pub rotation: Vec3d,
    /// Scale component of the transformation.
    pub scale: Vec3d,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3d::default(),
            rotation: Vec3d::default(),
            scale: Vec3d::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    /// Calculates the rotation matrix for the transformation.
    fn calculate_rotation_matrix(&self) -> Matrix4x4 {
        let rot = self.rotation;
        let rotation_x_matrix = Matrix4x4::from_rows([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, rot.x.cos(), -rot.x.sin(), 0.0],
            [0.0, rot.x.sin(), rot.x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let rotation_y_matrix = Matrix4x4::from_rows([
            [rot.y.cos(), 0.0, rot.y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-rot.y.sin(), 0.0, rot.y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let rotation_z_matrix = Matrix4x4::from_rows([
            [rot.z.cos(), -rot.z.sin(), 0.0, 0.0],
            [rot.z.sin(), rot.z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        rotation_z_matrix.mul(&rotation_y_matrix.mul(&rotation_x_matrix))
    }

    /// Creates a model matrix for the transformation, combining all specified
    /// components into a resulting matrix - model matrix.
    pub fn model_matrix(&self) -> Matrix4x4 {
        let scale_matrix = Matrix4x4::from_rows([
            [self.scale.x, 0.0, 0.0, 0.0],
            [0.0, self.scale.y, 0.0, 0.0],
            [0.0, 0.0, self.scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let rotation_matrix = self.calculate_rotation_matrix();
        let position_matrix = Matrix4x4::from_rows([
            [1.0, 0.0, 0.0, self.position.x],
            [0.0, 1.0, 0.0, self.position.y],
            [0.0, 0.0, 1.0, self.position.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        position_matrix.mul(&rotation_matrix.mul(&scale_matrix))
    }
}
