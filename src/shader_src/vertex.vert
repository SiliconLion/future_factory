
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNorm;

out vec3 pos;
out vec3 norm;

void main()
{
    pos = aPos;
    norm = aNorm;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}