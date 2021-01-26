#version 330 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D ourTexture;

void main()
{
    vec4 color = texture(ourTexture, texCoord);
    if (color == vec4(0.0, 0.0, 0.0, 0.0)) {
        discard;
    }
} 