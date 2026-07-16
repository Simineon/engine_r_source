#version 330 core

in vec2 vTexCoord;
in float vTexIndex;

out vec4 FragColor;

uniform sampler2D u_Textures[16];

void main() {
    int index = int(vTexIndex);

    if (index >= 0 && index < 16) {
        FragColor = texture(u_Textures[index], vTexCoord);
    } else {
        FragColor = vec4(1.0, 0.0, 1.0, 1.0);
    }
}
