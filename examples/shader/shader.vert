#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 inPos;
layout(location = 1) in vec4 inColor;
layout(location = 0) out vec4 fragmentColor;

void main() {
    gl_Position = vec4(inPos);
    fragmentColor = inColor;
}