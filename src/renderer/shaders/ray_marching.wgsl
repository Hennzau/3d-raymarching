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
var<uniform> inverted_projection_matrix: mat4x4<f32>;

@group(0)
@binding(2)
var<uniform> inverted_view_matrix: mat4x4<f32>;

@group(0)
@binding(3)
var<uniform> surface_configuration: vec2<f32>;

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

    let x = 2.0 * in_frag_position.x / surface_configuration.x - 1.0;
    let y = 1.0 - (2.0 * in_frag_position.y) / surface_configuration.y;
    let z = 1.0;

    let ray_nds = vec3<f32> (x, y, z);
    let ray_clip = vec4<f32> (ray_nds.x, ray_nds.y, -1.0, 1.0);

    var ray_eye = inverted_projection_matrix * ray_clip;
    ray_eye = vec4<f32> (ray_eye.xy, -1.0, 0.0);

    let ray_world_space = inverted_view_matrix * ray_eye;
    var ray_world = vec3<f32> (ray_world_space.x, ray_world_space.y, ray_world_space.z);
    ray_world = normalize (ray_world);

    var t: f32 = 0.0;

    for (var i: i32 = 0; i < 80; i = i + 1) {
        let p: vec3<f32> = camera_position + ray_world * t;
        let d: f32 = map(p);

        t = t + d;

        if (d < 0.001 || t > 100.0) {
            break;
        }
    }

    let col = vec3<f32> (t * 0.1);

    result.out_frag_color = vec4<f32> (col, 1.0);

    return result;
}