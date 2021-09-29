//[[block]]
//struct Uniforms {
    //resolution: vec2<f32>;
//    frame: f32;
 //   time: f32;
//};

//[[group(0), binding(0)]]
//var<uniform> u: Uniforms;

struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    // var out: vec2<f32>;
    var x = f32(i32((vertex_index << 1u32) & 2u32));
    var y = f32(i32(vertex_index & 2u32));
    var uv = vec2<f32>(x, y);
    var out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out, 0.0, 1.0);

}


[[stage(fragment)]]
fn main(
    [[builtin(position)]] in: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);

    var uv = in.xy / vec2<f32>(1800.0, 1800.0);
    // const half = vec3<f32>(0.5, 0.5, 0.5);
    // const time = vec3<f32>(u.time / 4.0, u.time / 3.0, u.time / 2.0);
    // const col: vec3<f32> = half + half * cos(time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0)); */
    return vec4<f32>(uv.x, uv.x, uv.x, 1.0);
}
