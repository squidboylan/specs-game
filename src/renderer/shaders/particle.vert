#version 330 core
layout (location = 0) in vec3 vert_position;
layout (location = 1) in vec4 offset;
layout (location = 2) in vec4 color;
layout (location = 3) in vec2 dimensions;

out vec4 fcolor;

void main()
{
    mat4 world_matrix = mat4(vec4(1.0/960.0, 0.0, 0.0, 0.0),
                             vec4(0.0, -1.0/540.0, 0.0, 0.0),
                             vec4(0.0, 0.0, 1.0, 0.0),
                             vec4(0.0, 0.0, 0.0, 1.0));
    gl_Position = vec4(vert_position.xy * dimensions + offset.xy, vert_position.z + offset.z, 1.0) * world_matrix + vec4(-1.0, 1.0, 0.0, 0.0);
    fcolor = color;
}
