#version 330 core

uniform vec4 colorUniform;

out vec4 Color;

void main()
{
    Color = colorUniform;
}