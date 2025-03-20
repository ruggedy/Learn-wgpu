struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
}

@group(0) @binding(0) var<uniform> use_position_color: u32;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (use_position_color == 1u) {
        return vec4<f32>(in.vert_pos + vec3<f32>(0.5, 0.5, 0.5), 1.0);
    } else {
        return vec4<f32>(0.3, 0.2, 0.1, 1.0);
    }
}
