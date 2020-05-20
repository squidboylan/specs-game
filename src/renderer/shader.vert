#version 330 core
layout (location = 0) in vec3 vert_position;
layout (location = 1) in vec4 offset;
layout (location = 2) in vec4 color;
layout (location = 3) in float size;
layout (location = 4) in float rotation;


out vec4 fcolor;

void main()
{
    mat2 rot_matrix = mat2(cos(rotation), sin(rotation),
                           -1.0f * sin(rotation), cos(rotation));
    mat2 world_matrix = mat2(1.0/960.0, 0.0,
                             0.0, 1.0/540.0);
    gl_Position = vec4(((vert_position.xy * rot_matrix * size) + offset.xy) * world_matrix, 0.0, 1.0) + vec4(-1.0, 1.0, 0.0, 0.0);
    fcolor = color;
}
