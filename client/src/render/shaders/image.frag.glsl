#version 330 core

in vec2 oTexCord;

uniform sampler2D tex;

////

void main() {
  gl_FragColor = texture(tex, oTexCord);
}
