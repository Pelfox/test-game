//! Representation and implementation for the player's camera.

use crate::math::{matrix::Matrix4x4, vector::Vec3d};

/// Represents the player's camera.
///
/// This structure is the CPU-side description of the camera.
pub struct Camera {
    /// Camera position relative to world coordinate system.
    pub eye: Vec3d,
    /// Where camera is looking, relative to world coordinate system.
    pub direction: Vec3d,
    /// "Up" direction of the camera.
    pub up_direction: Vec3d,
    /// Aspect ratio of the output (i.e. a screen).
    pub aspect_ratio: f32,
    /// Camera's field of view, in radians.
    pub fov: f32,
    /// Nearest visible distance.
    pub near: f32,
    /// Farthest visible distance.
    pub far: f32,
}

impl Camera {
    /// Creates a new camera from the given parameters.
    pub fn new(
        eye: Vec3d,
        direction: Vec3d,
        up_direction: Vec3d,
        aspect_ratio: f32,
        fov: f32,
    ) -> Self {
        Self {
            eye,
            direction,
            up_direction,
            aspect_ratio,
            fov,
            near: 0.1,
            far: 100.0,
        }
    }

    /// Creates a view matrix for the current camera position.
    pub fn view_matrix(&self) -> Matrix4x4 {
        let forward = self.direction.normalize();
        let right = forward.cross(&self.up_direction).normalize();
        let up = right.cross(&forward);

        Matrix4x4::from_rows([
            [right.x, right.y, right.z, -right.dot(&self.eye)],
            [up.x, up.y, up.z, -up.dot(&self.eye)],
            [-forward.x, -forward.y, -forward.z, forward.dot(&self.eye)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a projection matrix for the current camera position.
    pub fn projection_matrix(&self) -> Matrix4x4 {
        let scale_factor = 1.0 / (self.fov * 0.5).tan();
        let near = self.near;
        let far = self.far;

        Matrix4x4::from_rows([
            [scale_factor / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, scale_factor, 0.0, 0.0],
            [0.0, 0.0, far / (near - far), (near * far) / (near - far)],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}
