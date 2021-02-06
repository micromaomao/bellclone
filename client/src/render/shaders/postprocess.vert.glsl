#version 330 core
attribute vec2 aVertexPosition;
out vec2 oTexCord;

////

void main() {
  gl_Position = vec4(aVertexPosition, 0.0, 1.0);
  oTexCord = aVertexPosition * 0.5 + 0.5;
}
