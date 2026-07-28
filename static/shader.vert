#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texCoord;
layout (location = 2) in float aTexIndex;

out vec2 TexCoord;
out float TexIndex;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
    vec4 pos = vec4(position.x, position.y, 0.0, 1.0);
    gl_Position = projection * view * model * pos;
    TexCoord = texCoord;
    TexIndex = aTexIndex;
}
