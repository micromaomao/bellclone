#version 330 core

in vec2 aVertexPosition;
in vec2 aTexCord;
out vec2 oTexCord;

uniform mat4 uTransform;
////

void main() {
  gl_Position = uTransform * vec4(aVertexPosition, 0.0, 1.0);
  oTexCord = aTexCord;
}
