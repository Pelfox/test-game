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
    // Normal axis for the vertex.
    @location(1) normal: vec3<f32>,
};

// Output of the vertex shader to be processed by the fragment shader.
struct VertexOutput {
    // Scaled position of the vertex.
    @builtin(position) position: vec4<f32>,
    // Normal axis for the vertex.
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Scale and position the vertex using both camera view projection and
    // object's model matrix.
    output.position = camera.view_projection_matrix *
        object.model_matrix *
        vec4<f32>(input.position, 1.0);
    // Normalizing the normal axis for the vertex.
    output.normal = normalize(
        (object.model_matrix * vec4<f32>(input.normal, 0.0)).xyz
    );

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Create a light point on the given coordinates for a directional light.
    let light_direction = normalize(vec3<f32>(-0.5, -1.0, -0.5));
    // We need to normalize vertex' normal to ensure correct lighting
    // calculation.
    let normal = normalize(input.normal);
    // Determining "how similar" the light direction and vertex normal are.
    // Clamping to 0.0, since negative lighting doesn't make any sense.
    let light = max(dot(input.normal, -light_direction), 0.0);

    // Without ambient light, vertex faces pointing away from the light source
    // will become completely black, and this looks harsh, so we're adding a
    // small ambient light for them.
    let ambient = 0.2;

    // Getting the brightness of the current vertex. We add at least 20% from
    // the ambient light, and up to 80% from the light source itself. In the
    // end, down vertices will end up with only 0.2 units of brightness, while
    // up vertices will get 1 unit of brightness.
    let brightness = ambient + light * 0.8;

    // And finally, we're applying vertex brightness to its color. We touch
    // only R, G and B channels, since no matter the lighting, alpha channel
    // should stay the same.
    let lighted_color_material = vec3<f32>(
        object.color_material.x * brightness,
        object.color_material.y * brightness,
        object.color_material.z * brightness
    );

    return vec4<f32>(lighted_color_material, object.color_material.w);
}
