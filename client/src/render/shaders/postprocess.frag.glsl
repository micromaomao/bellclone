#version 330 core

in vec2 oTexCord;
uniform sampler2D tex;
uniform float mb_dist;

////

vec3 get_sample(vec2 texCord) {
  if (texCord.x < 0.0) {
    texCord.x = 0.0;
  }
  if (texCord.x > 1.0) {
    texCord.x = 1.0;
  }
  if (texCord.y < 0.0) {
    texCord.y = 0.0;
  }
  if (texCord.y > 1.0) {
    texCord.y = 1.0;
  }
  return texture(tex, texCord).rgb;
}

void main() {
  vec3 samp = vec3(0.0);
  float samp_sum = 0.0;
  const float mb_max_dist = 0.1;
  if (mb_dist > 0.001) {
    for (float yoff = -mb_max_dist; yoff < mb_max_dist; yoff += mb_max_dist / 10.0) {
      if (abs(yoff) <= mb_dist) {
        float weight = 1.0 - abs(yoff) / mb_max_dist;
        samp += weight * get_sample(oTexCord + vec2(0.0, yoff));
        samp_sum += weight;
      }
    }
  } else {
    samp = get_sample(oTexCord);
    samp_sum = 1.0;
  }
  samp /= samp_sum;
  const float gamma = 2.2;
  gl_FragColor += vec4(pow(samp, vec3(1.0/gamma)), 1.0);
}
