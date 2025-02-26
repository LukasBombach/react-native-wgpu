struct SurfaceUniform {
    size: vec2<f32>,
};

@group(0) @binding(0) 
var<uniform> surface: SurfaceUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4f,
};

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
};


@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
    @builtin(vertex_index) vertex_index: u32
) -> VertexOutput {

    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1), // red
        vec4f(0, 1, 0, 1), // green
        vec4f(0, 0, 1, 1), // blue
    );

    let ndc_pos = (
        (
            // Scale the rect to the instance size
            vertex.position * (instance.size / surface.size) * 2.0
            // Move the rect from the center to the top left of the screen
            + vec2<f32>(-1.0, 1.0)
            // Move by the height of the instance (ndc = *2) because
            // the rect ist anchored to the top left of its instance
            + (instance.size / surface.size) * vec2<f32>(0.0, -2.0)
        )

        + (
            // Convert instance pixel position to NDC
            (instance.position / surface.size)
            
            // Flip Y axis (NDC Y axis is inverted) and  Scale to NDC (NDC 0-1 is only half of the screen)
            * vec2<f32>(2.0, -2.0)

        )
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_pos, 1.0, 1.0);
    output.color = color[vertex_index];
    return output;
}

@fragment 
fn fs_main(fsInput: VertexOutput) -> @location(0) vec4f {
    return fsInput.color;
}