//! This module describes and implements matrices for different sizes.

/// Represents a 4x4 matrix for the f32 type.
#[repr(C)]
#[derive(Clone, Copy, std::fmt::Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4x4 {
    /// Values of the matrix, represented in rows.
    pub rows: [[f32; 4]; 4],
}

impl Matrix4x4 {
    /// Describes the format of the structure as a vertex, allowing to create
    /// vertex attributes with this structure.
    pub const VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;

    /// Creates a new 4x4 matrix from the given rows.
    pub fn from_rows(rows: [[f32; 4]; 4]) -> Self {
        Self { rows }
    }

    /// Multiplies the current matrix with the given one.
    pub fn mul(&self, another: &Self) -> Self {
        let mut result_matrix = Self::from_rows([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum = sum + self.rows[i][k] * another.rows[k][j];
                }
                result_matrix.rows[i][j] = sum;
            }
        }
        result_matrix
    }

    /// Multiplies the current matrix with the given four-dimensional vector.
    pub fn mul_vec(&self, vec: &super::vector::Vec4d) -> [f32; 4] {
        let vec_matrix = Self::from_rows([
            [vec.x, 0.0, 0.0, 0.0],
            [vec.y, 0.0, 0.0, 0.0],
            [vec.z, 0.0, 0.0, 0.0],
            [vec.w, 0.0, 0.0, 0.0],
        ]);
        let result = self.mul(&vec_matrix);
        [
            result.rows[0][0],
            result.rows[1][0],
            result.rows[2][0],
            result.rows[3][0],
        ]
    }

    /// Converts this matrix into WGSL-compatible one, passing colums first, then rows.
    pub fn to_wgsl_matrix(&self) -> [[f32; 4]; 4] {
        [
            [
                self.rows[0][0],
                self.rows[1][0],
                self.rows[2][0],
                self.rows[3][0],
            ],
            [
                self.rows[0][1],
                self.rows[1][1],
                self.rows[2][1],
                self.rows[3][1],
            ],
            [
                self.rows[0][2],
                self.rows[1][2],
                self.rows[2][2],
                self.rows[3][2],
            ],
            [
                self.rows[0][3],
                self.rows[1][3],
                self.rows[2][3],
                self.rows[3][3],
            ],
        ]
    }
}
