/// Represents a single color, in sRGB color space as three f32 components.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    /// Red channel of the color.
    pub r: f32,
    /// Green channel of the color.
    pub g: f32,
    /// Blue channel of the color.
    pub b: f32,
    /// Alpha (transparency) channel of the color.
    pub a: f32,
}

impl Color {
    /// Describes the format of the structure as a vertex, allowing to create
    /// vertex attributes with this structure.
    pub const VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;

    /// Creates a new color from the given RGBA color components.
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new color from the given RGB color components.
    ///
    /// This function will set alpha channel to a fully opaque value.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color from the given HEX value.
    ///
    /// This function expects value in the RRGGBBAA format.
    pub fn hex(value: u32) -> Self {
        let r = ((value >> 24) & 0xFF) as f32 / 255.0;
        let g = ((value >> 16) & 0xFF) as f32 / 255.0;
        let b = ((value >> 8) & 0xFF) as f32 / 255.0;
        let a = (value & 0xFF) as f32 / 255.0;
        Self { r, g, b, a }
    }
}
