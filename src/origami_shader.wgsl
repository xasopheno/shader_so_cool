// Vertex shader

struct Uniforms {
    view_proj: mat4x4<f32>;
    //width: f32;
    //height: f32;
    //frame: f32;
    //time: f32;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
  [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.color = model.color;
  out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
  return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}
