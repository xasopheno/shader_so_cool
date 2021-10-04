[[block]]
struct Uniforms {
    resolution: vec2<f32>;
    frame: f32;
    time: f32;
};


var<push_constant> u: Uniforms;

[[group(0), binding(0)]]
var<uniform> u: Uniforms;

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    var x = f32(i32((vertex_index << 1u32) & 2u32));
    var y = f32(i32(vertex_index & 2u32));
    var uv = vec2<f32>(x, y);
    var out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out.x, out.y, 0.0, 1.0);

}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    var uv = frag_coord.xy / u.resolution;
    return vec4<f32>(uv.x, uv.x, uv.x, 1.0);
}
