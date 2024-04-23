struct VertexOutput {
    @builtin(position) out_vertex_pos: vec4<f32>
}

struct FragmentOutput {
    @location(0) out_frag_color: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> projection_view_model_matrix: mat4x4<f32>;

@vertex
fn vs_main(

    @location(0) in_vertex_position: vec3<f32>,
    @location(1) in_vertex_color: vec4<f32>,

) -> VertexOutput {
    var result: VertexOutput;

    result.out_vertex_pos = projection_view_model_matrix * vec4<f32> (in_vertex_position.x, in_vertex_position.y, in_vertex_position.z, 1.0);

    return result;
}

@fragment
fn fs_main(

    @builtin(position) in_frag_position: vec4<f32>,

) -> FragmentOutput {
    var result: FragmentOutput;

    result.out_frag_color = vec4<f32> (1.0, 0.0, 0.0, 1.0);

    return result;
}