[[block]]
struct Uniforms {
    width: f32;
    height: f32;
    frame: f32;
    time: f32;
};

[[group(0), binding(0)]]
var<uniform> u: Uniforms;


[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    var x = f32(i32((vertex_index << u32(1)) & u32(2)));
    var y = f32(i32(vertex_index & u32(2)));
    var uv = vec2<f32>(x, y);
    var out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out.x, out.y, 0.0, 1.0);

}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(u.width, u.height);
    var uv = frag_coord.xy / resolution;
    if (
        sin((100.0 + u.time) * 0.3 * uv.x) > 0.499 
        && sin(uv.x * 10.0) < 0.801
        && sin(u.time * uv.y * uv.x * 5000.0) < 0.1
    ) {
        return vec4<f32>(sin(u.time * 0.3) * 0.01, 0.01, 0.02, 1.0);
    }
    return vec4<f32>(sin(u.time * 0.333) * uv.x * 0.01, sin(u.time) * uv.x * 0.01, sin(u.time * 0.1) * uv.x * 0.01, 1.0);
}


