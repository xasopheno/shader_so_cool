// Vertex shader

[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
  [[location(0)]] color: vec4<f32>;
};

struct InstanceInput {
    [[location(5)]] model_matrix_0: vec4<f32>;
    [[location(6)]] model_matrix_1: vec4<f32>;
    [[location(7)]] model_matrix_2: vec4<f32>;
    [[location(8)]] model_matrix_3: vec4<f32>;
    [[location(9)]] life: f32;
    [[location(10)]] size: f32;
    [[location(11)]] length: f32;
};

[[stage(vertex)]]
fn main(
  model: VertexInput,
  instance: InstanceInput,
) -> VertexOutput {
  let model_matrix = mat4x4<f32>(
      instance.model_matrix_0,
      instance.model_matrix_1,
      instance.model_matrix_2,
      instance.model_matrix_3,
  );
  var out: VertexOutput;
  let color_matrix = vec3<f32>(
      pow(model.color[0] * instance.life, 3.0),
      pow(model.color[1] * instance.life, 3.0),
      pow(model.color[2] * instance.life, 3.0),
  );

  let scale = mat4x4<f32>(
      vec4<f32>(instance.size * 1.2, 0.0, 0.0, 0.0),
      vec4<f32>(0.0, instance.size * 1.2, 0.0, 0.0),
      vec4<f32>(0.0, 0.0, instance.size * 1.2, 0.0),
      vec4<f32>(0.0, 0.0, 0.0, 1.0)
  );

  out.color = vec4<f32>(color_matrix, instance.life);
  out.clip_position = 
    uniforms.view_proj 
    * model_matrix 
    * scale
    * vec4<f32>(
        model.position.x + 2.0 + instance.life * 3.0, 
        model.position.y * 1.4,
        model.position.z + (instance.life * 80.0) - 160.0, 
        1.0
    );
  return out;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(in.color);
}

