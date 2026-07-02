//! This module represents CPU-sided player's camera in the world.

use crate::math::{matrix::Matrix4x4, vector::Vec3d};

/// Describes the nearest visible distance for the object.
const Z_NEAR: f32 = 0.01;

/// Describes the farthest visible distance for the object.
const Z_FAR: f32 = 100.0;

/// Represents player's camera.
///
/// This structure is the CPU-side description of the camera.
pub struct Camera {
    /// Camera position relative to world coordinate system.
    eye: Vec3d,
    /// "Up" direction of the camera.
    up_direction: Vec3d,
    /// Aspect ratio of the screen.
    aspect_ratio: f32,

    /// Camera's field of view, in radians.
    pub fov: f32,
    /// Current rotational movement alongside vertical axis of the camera.
    pub yaw: f32,
    /// Current rotational movement alongside X axis of the camera.
    pub pitch: f32,
}

impl Camera {
    /// Creates a new camera from the given parameters.
    pub fn new(eye: Vec3d, aspect_ratio: f32, fov: f32) -> Self {
        Self {
            eye,
            up_direction: Vec3d::new(0.0, 1.0, 0.0),
            aspect_ratio,
            fov,
            yaw: 0.0, // With current `up_direction`, rotates alongside Y axis.
            pitch: 0.0,
        }
    }

    /// Calculates the three-dimensional vector of the direction in which
    /// camera is looking.
    pub fn direction(&self) -> Vec3d {
        println!("yaw={}, pitch={}", self.yaw, self.pitch);
        Vec3d::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        )
        .normalize()
    }

    /// Creates a view matrix for the current camera position.
    pub fn view_matrix(&self) -> Matrix4x4 {
        let forward = self.direction(); // Direction is already normalized.
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
        let near = Z_NEAR;
        let far = Z_FAR;

        Matrix4x4::from_rows([
            [scale_factor / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, scale_factor, 0.0, 0.0],
            [0.0, 0.0, far / (near - far), (near * far) / (near - far)],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}
