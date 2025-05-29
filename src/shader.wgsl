var<push_constant> viewport: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) inst_pos: vec2<f32>,
    @location(1) inst_size: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
) -> VertexOutput {

    // Generate quad vertices based on vertex index
    // 0: (0.0, 1.0) - left top
    // 1: (0.0, 0.0) - left bottom  
    // 2: (1.0, 0.0) - right bottom
    // 3: (1.0, 1.0) - right top
    var vert_pos: vec2<f32>;
    switch vertex_index % 4u {
        case 0u: { vert_pos = vec2<f32>(0.0, 1.0); }
        case 1u: { vert_pos = vec2<f32>(0.0, 0.0); }
        case 2u: { vert_pos = vec2<f32>(1.0, 0.0); }
        case 3u, default: { vert_pos = vec2<f32>(1.0, 1.0); }
    }

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