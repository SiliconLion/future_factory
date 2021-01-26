
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 scale;
uniform vec2 translation;

out vec2 texCoord;

void main()
{
    // vec3 pos = (scale * vec4(aPos, 1.0) ).xyz + vec3(translation, 1.0);
    vec3 pos = (scale * vec4( aPos + vec3(translation, 0.0), 1.0)).xyz;
    texCoord = aTexCoord;
    gl_Position = vec4(pos, 1.0);
}