var<push_constant> viewport: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) instance_pos: vec2<f32>,
    @location(1) instance_size: vec2<f32>,
    @location(2) instance_tex_coords: vec4<f32>, // x, y, width, height
    @location(3) instance_color: vec4<f32>,
) -> VertexOutput {
    var vertex_pos: vec2<f32>;
    var tex_coord: vec2<f32>;
    
    switch vertex_index % 6u {
        case 0u: { 
            vertex_pos = vec2<f32>(0.0, 1.0); 
            tex_coord = vec2<f32>(0.0, 1.0);
        }
        case 1u: { 
            vertex_pos = vec2<f32>(0.0, 0.0); 
            tex_coord = vec2<f32>(0.0, 0.0);
        }
        case 2u: { 
            vertex_pos = vec2<f32>(1.0, 0.0); 
            tex_coord = vec2<f32>(1.0, 0.0);
        }
        case 3u: { 
            vertex_pos = vec2<f32>(0.0, 1.0); 
            tex_coord = vec2<f32>(0.0, 1.0);
        }
        case 4u: { 
            vertex_pos = vec2<f32>(1.0, 0.0); 
            tex_coord = vec2<f32>(1.0, 0.0);
        }
        case 5u, default: { 
            vertex_pos = vec2<f32>(1.0, 1.0); 
            tex_coord = vec2<f32>(1.0, 1.0);
        }
    }

    let pos = instance_pos + vertex_pos * instance_size;
    let ndc_x = (pos.x / viewport.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pos.y / viewport.y) * 2.0;

    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    
    // Calculate texture coordinates
    let tex_start = instance_tex_coords.xy;
    let tex_size = instance_tex_coords.zw;
    output.tex_coords = tex_start + tex_coord * tex_size;
    
    output.color = instance_color;

    return output;
}

@group(0) @binding(0)
var text_texture: texture_2d<f32>;

@group(0) @binding(1)
var text_sampler: sampler;

@fragment
fn fs_main(vs_output: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(text_texture, text_sampler, vs_output.tex_coords).r;
    return vec4<f32>(vs_output.color.rgb, vs_output.color.a * alpha);
}
