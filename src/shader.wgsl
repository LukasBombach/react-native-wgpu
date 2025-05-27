var<push_constant> viewport: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) vert_pos: vec2<f32>,
    @location(1) inst_pos: vec2<f32>,
    @location(2) inst_size: vec2<f32>,
    @location(3) bg_color: vec4<f32>,
    @builtin(vertex_index) vert_index: u32
) -> VertexOutput {

    // var color = array<vec4f, 3>(
    //     vec4f(1, 0, 0, 1), // red
    //     vec4f(0, 1, 0, 1), // green
    //     vec4f(0, 0, 1, 1), // blue
    // );

    let pos = inst_pos + vert_pos * inst_size;
    let ndc_x = (pos.x / viewport.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / viewport.y) * 2.0;

    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_x, ndc_y, 1.0, 1.0);
    output.color = bg_color;
    return output;
}

@fragment 
fn fs_main(fsInput: VertexOutput) -> @location(0) vec4f {
    return fsInput.color;
}