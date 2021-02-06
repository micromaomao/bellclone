#version 330 core

in vec2 oTexCord;

uniform sampler2D tex;
uniform float alpha;

////

void main() {
  vec4 samp = texture(tex, oTexCord);
  gl_FragColor = vec4(samp.rgb, samp.a * alpha);
}
