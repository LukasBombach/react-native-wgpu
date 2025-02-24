struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(vertex_index) vertex_index: u32
) -> VertexOutput {

    var color = array<vec4f, 3>(
        vec4f(1, 0, 0, 1), // red
        vec4f(0, 1, 0, 1), // green
        vec4f(0, 0, 1, 1), // blue
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(model.position, 1.0, 1.0);
    output.color = color[vertex_index];
    return output;
}

@fragment 
fn fs_main(fsInput: VertexOutput) -> @location(0) vec4f {
    return fsInput.color;
}