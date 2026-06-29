/// Represents a three-dimensional vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3d {
    /// The vector component along the X axis.
    pub x: f64,
    /// The vector component along the Y axis.
    pub y: f64,
    /// The vector component along the Z axis.
    pub z: f64,
}

impl Vec3d {
    /// Creates a new three-dimensional vector from its X, Y and Z components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Represents a two-dimensional vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2d {
    /// The vector component along the X axis.
    pub x: f64,
    /// The vector component along the Y axis.
    pub y: f64,
}

/// Represents a triangle in 3D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    /// The three vertices of the triangle.
    pub vertices: [Vec3d; 3],
}

impl Triangle {
    /// Projects this triangle into two-dimensional vector, additionally
    /// returning average triangle depth.
    ///
    /// This function expects that FOV is given in the degrees, and width and
    /// height is output's (screen's) width and height respectively.
    ///
    /// For now, triangle's depth is averaged over all verticies.
    pub fn into_vec2d(&self, fov: f64, width: u32, height: u32) -> ([Vec2d; 3], f64) {
        let mut projected_vertices: [Vec2d; 3] = [Vec2d::default(); 3];
        let mut total_depth = 0.0;
        for (i, vertex) in self.vertices.iter().enumerate() {
            let (projection, depth) = project_vec3_to_vec2(vertex, fov, width, height);
            projected_vertices[i] = projection;
            total_depth += depth;
        }
        (projected_vertices, (total_depth) / 3.0)
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

/// Distance from the camera to the near clipping plane.
const Z_NEAR: f64 = 0.1;

/// Distance from the camera to the far clipping plane.
const Z_FAR: f64 = 1000.0;

/// Projects given vector in 3D space onto the 2D space.
///
/// This function expects that FOV is given in the degrees, and width and
/// height is output's (screen's) width and height respectively.
pub fn project_vec3_to_vec2(vec: &Vec3d, fov: f64, width: u32, height: u32) -> (Vec2d, f64) {
    let fov_scale = 1.0 / (fov.to_radians() * 0.5).tan();
    // Converts camera-space Z into depth-buffer Z (how far each is element on the scale 0..1).
    let depth_scale = Z_FAR / (Z_FAR - Z_NEAR);

    let aspect_ratio = (width as f64) / (height as f64);

    let x = vec.x * (fov_scale / aspect_ratio);
    let y = vec.y * fov_scale;
    let z = depth_scale * (vec.z - Z_NEAR);
    let w = vec.z;

    // Projecting 3D-spaced coordinates onto the 2D space of the output vector.
    let x_projected = x / w;
    let y_projected = y / w;
    let z_projected = z / w;

    // Translating coordinates from -1..1 into desired output's width and height.
    let x_output = ((x_projected + 1.0) / 2.0) * (width as f64);
    let y_output = ((1.0 - y_projected) / 2.0) * (height as f64);

    let new_vec = Vec2d {
        x: x_output,
        y: y_output,
    };
    (new_vec, z_projected)
}
