attribute vec2 aVertexPosition;

uniform mat4 uViewMat;
uniform mat4 uObjectTransform;
uniform vec2 uSize;

////

void main() {
  gl_Position = uViewMat * uObjectTransform * vec4(vec2(aVertexPosition.x * uSize.x, aVertexPosition.y * uSize.y), 0.0, 1.0);
}
