#version 330 core
layout (location = 0) in vec3 vert_position;
layout (location = 1) in vec4 offset;
layout (location = 2) in vec4 color;
layout (location = 3) in vec3 size;
layout (location = 4) in float rotation;


out vec4 fcolor;

void main()
{
    mat3 rot_matrix = mat3(vec2(cos(rotation), sin(rotation)), 0.0,
                           vec2(-1.0f * sin(rotation), cos(rotation)), 0.0,
                           vec2(0.0, 0.0), 1.0);
    mat4 world_matrix = mat4(vec4(1.0/960.0, 0.0, 0.0, 0.0),
                             vec4(0.0, -1.0/540.0, 0.0, 0.0),
                             vec4(0.0, 0.0, 1.0, 0.0),
                             vec4(0.0, 0.0, 0.0, 1.0));
    gl_Position = vec4(((rot_matrix * vert_position * size) + offset.xyz), 1.0) * world_matrix + vec4(-1.0, 1.0, 0.0, 0.0);
    fcolor = color;
}
