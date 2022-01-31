struct Uniforms {
    width: f32;
    height: f32;
    frame: f32;
    time: f32;
    //kind: u32; 
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
    var uv = frag_coord.xy / resolution - vec2<f32>(0.5, 0.5);
    // var uv = frag_coord.xy / resolution - vec2<f32>(1.15, 1.15);
   //var thing = uv.x < 0.35 && uv.y > 0.85;
   //if (thing) {
//  return vec4<f32>(
//      sin((103.0/uv.x * 45.0/uv.y) / 1.0) * 100.45, 
//      sin((80.0/uv.x * 45.0/uv.y) / 1.0) * 0.45, 
//      sin((80.0/uv.x * 85.0/uv.y) / 1.0) * 100.05, 
//      1.0
//  );
  var time = 1.0;

  return vec4<f32>(
      sin(23.0/uv.x * 15.0/uv.y * sin(time/500.0 + 30.0)) * 0.35, 
      cos(14.0/uv.x * 14.0/uv.y * sin(time/1000.0 + 100.0)) * 0.15, 
      cos(15.0/uv.x * 14.0/uv.y * sin(time/800.0 + 20.0)) * 0.45, 
      1.0
  );


//   return vec4<f32>(
//       sin(4.0/uv.x * 5.0/uv.y) * 0.15, 
//       sin(4.0/uv.x * 5.0/uv.y) * 0.15, 
//       sin(4.0/uv.x * 5.0/uv.y) * 0.15, 
//       1.0
//   );

//   return vec4<f32>(
//       sin(uv.y * uv.x) * 0.04, 
//       sin(uv.y * -uv.x) * 0.08, 
//       sin(uv.y * uv.x) * 0.30, 
//       1.0
//   );
    // else {
    //   return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    //
}


