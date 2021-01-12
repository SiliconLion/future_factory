
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 transform;

out vec2 texCoord;

void main()
{
    vec3 pos = (transform * vec4(aPos, 1.0) ).xyz;
    texCoord = aTexCoord;
    gl_Position = vec4(pos, 1.0);
}