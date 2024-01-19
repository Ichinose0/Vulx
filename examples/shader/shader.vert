#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 inPos;
layout(location = 1) in vec4 inColor;
layout(set = 0, binding = 0) uniform UBO {
    mat4 model;
    mat4 view;
    mat4 projection;
} ubo;
layout(location = 0) out vec4 fragmentColor;

void main() {
    gl_Position = ubo.projection * ubo.view * ubo.model * vec4(inPos);
    fragmentColor = inColor;
}