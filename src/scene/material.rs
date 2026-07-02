//! This module represents materials that can be applied to the game objects.

use crate::renderer::color::Color;

/// Represents a material that can be applied to the game object.
pub enum Material {
    /// Apply the given color to the object.
    Color(Color),
}
