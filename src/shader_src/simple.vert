#version 330 core
layout (location = 0) in vec3 aPos;

uniform vec2 translation;

out vec3 pos;

void main()
{
    pos = vec3(translation + aPos.xy, aPos.z);
    gl_Position = vec4(pos, 1.0);
}