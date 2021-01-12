#version 330 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D ourTexture;

void main()
{
    FragColor = texture(ourTexture, texCoord);
    // FragColor = vec4(1.0, 0.0, 0.0, 1.0);
} 