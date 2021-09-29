[[block]]
struct Uniforms {
    resolution: vec2<f32>;
    frame: f32;
    time: f32;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    // var out: vec2<f32>;
    const x = f32(i32((vertex_index << 1u32) & 2u32));
    const y = f32(i32(vertex_index & 2u32));
    const uv = vec2<f32>(x, y);
    const out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out.x, out.y, 0.0, 1.0);

}

var<push_constant> u: Uniforms;

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);

    const uv = frag_coord.xy / u.resolution;
    // const half = vec3<f32>(0.5, 0.5, 0.5);
    // const time = vec3<f32>(u.time / 4.0, u.time / 3.0, u.time / 2.0);
    // const col: vec3<f32> = half + half * cos(time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0)); */
    return vec4<f32>(0.1 * sin(0.1 * u.time), 0.1, 0.3, 1.0);
}
