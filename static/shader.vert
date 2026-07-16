#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texCoord;
layout(location = 2) in float aTexIndex;

out vec2 vTexCoord;
out float vTexIndex;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vTexCoord = texCoord;
    vTexIndex = aTexIndex;
}
