// Describes camera's uniform.
struct CameraUniform {
    // This is a view and a projection matrix, multiplied. It should be used to
    // scale the object, using its own model matrix.
    view_projection_matrix: mat4x4<f32>,
};

// Uniform of the game's camera.
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Describes a uniform for a single game object that should be drawn.
struct ObjectUniform {
    // Object's model matrix.
    model_matrix: mat4x4<f32>,
    // Object's color material.
    color_material: vec4<f32>,
};

// Uniform of current game object.
@group(1) @binding(0)
var<uniform> object: ObjectUniform;

// Input of the vertex from the CPU to be rendered.
struct VertexInput {
    // Local position of the vertex.
    @location(0) position: vec3<f32>,
};

// Output of the vertex shader to be processed by the fragment shader.
struct VertexOutput {
    // Scaled position of the vertex.
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Scale and position the vertex using both camera view projection and
    // object's model matrix.
    output.position = camera.view_projection_matrix *
        object.model_matrix *
        vec4<f32>(input.position, 1.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // For now, just return original color material of the object.
    return object.color_material;
}
