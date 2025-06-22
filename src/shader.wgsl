var<push_constant> viewport: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) background_color: vec4<f32>,
    @location(1) border_radius: f32,
    @location(2) rect_pos: vec2<f32>,
    @location(3) rect_size: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_pos: vec2<f32>,
    @location(1) instance_size: vec2<f32>,
    @location(2) background_color: vec4<f32>,
    @location(3) border_radius: f32,
) -> VertexOutput {

    var vertex_pos: vec2<f32>;
    switch vertex_index % 6u {
        case 0u: { vertex_pos = vec2<f32>(0.0, 1.0); }           // left top
        case 1u: { vertex_pos = vec2<f32>(0.0, 0.0); }           // left bottom
        case 2u: { vertex_pos = vec2<f32>(1.0, 0.0); }           // right bottom
        case 3u: { vertex_pos = vec2<f32>(0.0, 1.0); }           // left top
        case 4u: { vertex_pos = vec2<f32>(1.0, 0.0); }           // right bottom
        case 5u, default: { vertex_pos = vec2<f32>(1.0, 1.0); }  // right top
    }

    let pos = instance_pos + vertex_pos * instance_size;
    let ndc_x = (pos.x / viewport.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / viewport.y) * 2.0;

    var output: VertexOutput;

    output.clip_position = vec4<f32>(ndc_x, ndc_y, 1.0, 1.0);
    output.background_color = background_color;
    output.border_radius = border_radius;
    output.rect_pos = vertex_pos * instance_size;
    output.rect_size = instance_size;

    return output;
}

@fragment 
fn fs_main(vs_output: VertexOutput) -> @location(0) vec4f {
    let radius = vs_output.border_radius;
    let rect_size = vs_output.rect_size;
    let rect_pos = vs_output.rect_pos;
    
    // Calculate distance from corners for rounded rectangle
    let corner_radius = min(radius, min(rect_size.x, rect_size.y) * 0.5);

    var alpha: f32;

    if (corner_radius == 0.0) {
        alpha = 1.0;
    } else {
        let corner_distance = length(max(abs(rect_pos - rect_size * 0.5) - (rect_size * 0.5 - corner_radius), vec2<f32>(0.0, 0.0)));
    // Anti-aliased edge
        let edge_softness = 1.0;
        alpha = 1.0 - smoothstep(corner_radius - edge_softness, corner_radius, corner_distance);
    }

    return vec4<f32>(vs_output.background_color.rgb, vs_output.background_color.a * alpha);
}
