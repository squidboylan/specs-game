#version 330 core
layout (location = 0) in vec3 vert_position;
layout (location = 1) in vec4 offset;

out vec2 tex_pos;

void main()
{
    mat4 world_matrix = mat4(vec4(1.0/960.0, 0.0, 0.0, 0.0),
                             vec4(0.0, -1.0/540.0, 0.0, 0.0),
                             vec4(0.0, 0.0, 1.0, 0.0),
                             vec4(0.0, 0.0, 0.0, 1.0));
    tex_pos = vert_position.xy + vec2(0.5, 0.5);
    gl_Position = vec4(vert_position * 32.0 + offset.xyz, 1.0) * world_matrix + vec4(-1.0, 1.0, 0.0, 0.0);
}
