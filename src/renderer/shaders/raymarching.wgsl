struct VertexOutput {
    @builtin(position) out_vertex_pos: vec4<f32>
}

struct FragmentOutput {
    @location(0) out_frag_color: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> camera_position: vec3<f32>;
@group(0)
@binding(1)
var<uniform> camera_direction: vec3<f32>;

@vertex
fn vs_main(

    @location(0) in_vertex_position: vec2<f32>

) -> VertexOutput {
    var result: VertexOutput;

    result.out_vertex_pos = vec4<f32> (in_vertex_position.x, in_vertex_position.y, 0.0, 1.0);

    return result;
}

fn map (p: vec3<f32>) -> f32 {
    return length(p) - 1.0;
}

@fragment
fn fs_main(

    @builtin(position) in_frag_position: vec4<f32>,

) -> FragmentOutput {
    var result: FragmentOutput;

    let uv: vec2<f32> =  vec2<f32> ((in_frag_position.x / 720.0) * 2.0 - 1280.0/720.0, (in_frag_position.y / 720.0) * 2.0 - 1.0);

    let ro = vec3<f32> (0, 0, -3) - camera_position;

    let forward = normalize (vec3<f32> (-camera_direction.z, -camera_direction.x, 1.0));

    let side = normalize (cross (vec3<f32> (0, 1, 0), forward));
    let up = cross (forward, side);

    let rd = normalize (forward + uv.x * side + uv.y * up);

    var t: f32 = 0.0;

    for (var i: i32 = 0; i < 80; i = i + 1) {
        let p: vec3<f32> = ro + rd * t;
        let d: f32 = map(p);

        t = t + d;
    }

    let col = vec3<f32> (t * 0.001);

    result.out_frag_color = vec4<f32> (col, 1.0);

    return result;
}