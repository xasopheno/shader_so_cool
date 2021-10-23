precision highp float;

uniform vec3 iResolution;
uniform sampler2D iChannel0;
uniform bool flip;
uniform vec2 direction;

vec4 blur(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
  vec4 color = vec4(0.0);
  vec2 off1 = vec2(1.3333333333333333) * direction;
  color += texture2D(image, uv) * 0.29411764705882354;
  color += texture2D(image, uv + (off1 / resolution)) * 0.35294117647058826;
  color += texture2D(image, uv - (off1 / resolution)) * 0.35294117647058826;
  return color; 
}

void main() {
  vec2 uv = vec2(gl_FragCoord.xy / iResolution.xy);
  if (flip) {
    uv.y = 1.0 - uv.y;
  }

  gl_FragColor = blur(iChannel0, uv, iResolution.xy, direction);
}

