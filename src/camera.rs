use crate::math::{matrix::Matrix4x4, vector::Vec3d};

/// Represents the player's camera in the world.
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

/// Represents the GPU side of the camera.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// The view and the projection matrix of the camera view.
    pub view_projection_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    /// Creates a new camera uniform from the given camera, extracting its
    /// view and projection matrices and multiplying it.
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            view_projection_matrix: camera
                .projection_matrix()
                .mul(&camera.view_matrix())
                .to_wgsl_matrix(),
        }
    }
}
