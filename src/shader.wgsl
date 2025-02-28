@group(0) @binding(0) 
var<uniform> view_size: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    @location(0) vert_pos: vec2<f32>,
    @location(1) inst_pos: vec2<f32>,
    @location(2) inst_size: vec2<f32>,
    @builtin(vertex_index) vert_index: u32
) -> VertexOutput {

    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1), // red
        vec4f(0, 1, 0, 1), // green
        vec4f(0, 0, 1, 1), // blue
    );

    let ndc_pos = (
        (
            // Scale the rect to the instance size
            vert_pos * (inst_size / view_size) * 2.0
            // Move the rect from the center to the top left of the screen
            + vec2<f32>(-1.0, 1.0)
            // Move by the height of the instance (ndc = *2) because
            // the rect ist anchored to the top left of its instance
            + (inst_size / view_size) * vec2<f32>(0.0, -2.0)
        )

        + (
            // Convert instance pixel position to NDC
            (inst_pos / view_size)
            
            // Flip Y axis (NDC Y axis is inverted) and  Scale to NDC (NDC 0-1 is only half of the screen)
            * vec2<f32>(2.0, -2.0)

        )
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_pos, 1.0, 1.0);
    output.color = color[vert_index];
    return output;
}

@fragment 
fn fs_main(fsInput: VertexOutput) -> @location(0) vec4f {
    return fsInput.color;
}