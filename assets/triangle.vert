#version 330 core

layout (location = 0) in vec3 position;
// layout (location = 1) in vec3 color;

// out VS_OUTPUT {
//     vec3 Color;
// } OUT;

void main()
{
    gl_Position = vec4(position, 1.0);
    // OUT.Color = color;
}