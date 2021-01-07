
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNorm;

uniform mat4 scale;

out vec3 pos;
out vec3 norm;

void main()
{
    pos = (scale * vec4(aPos, 1.0) ).xyz;
    norm = aNorm;
    gl_Position = vec4(pos, 1.0);
}