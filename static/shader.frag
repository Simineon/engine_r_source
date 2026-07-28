#version 330 core

in vec2 TexCoord;
in float TexIndex;

out vec4 FragColor;

uniform sampler2D u_Textures[8];

void main() {
    int index = int(TexIndex);
    vec4 texColor;

    if (index == 0)
        texColor = texture(u_Textures[0], TexCoord);
    else if (index == 1)
        texColor = texture(u_Textures[1], TexCoord);
    else if (index == 2)
        texColor = texture(u_Textures[2], TexCoord);
    else if (index == 3)
        texColor = texture(u_Textures[3], TexCoord);
    else if (index == 4)
        texColor = texture(u_Textures[4], TexCoord);
    else if (index == 5)
        texColor = texture(u_Textures[5], TexCoord);
    else if (index == 6)
        texColor = texture(u_Textures[6], TexCoord);
    else if (index == 7)
        texColor = texture(u_Textures[7], TexCoord);
    else
        texColor = vec4(1.0, 0.0, 1.0, 1.0);

    FragColor = texColor;
}
