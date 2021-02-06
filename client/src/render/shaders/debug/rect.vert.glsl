#version 330 core

attribute vec2 aVertexPosition;

uniform mat4 uTransform;
uniform vec2 uSize;

////

void main() {
  gl_Position = uTransform * vec4(vec2(aVertexPosition.x * uSize.x * 0.5, aVertexPosition.y * uSize.y * 0.5), 0.0, 1.0);
}
