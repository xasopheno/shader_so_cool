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
    var color = vec4<f32>(sin(u.time * 0.003) * (uv.x + uv.y) * uv.y, sin(u.time * 0.001) * uv.y - uv.x, sin(u.time * 0.01) * uv.x - uv.y, 1.0);
    return color * vec4<f32>(sin(u.time * 30.0 * (uv.x - uv.x / uv.y)) * 0.01, tan(u.time * 10.0 * (uv.y - uv.y / uv.x))
    * 0.0001, tan(u.time * 3.0 * (1.0 - uv.x - uv.y / uv.x)) * 0.00001, 1.0);
}


