#version 330 core
layout (location = 0) in vec3 vert_position;
layout (location = 1) in vec4 offset;
layout (location = 2) in vec4 tile_pos;
layout (location = 3) in vec3 size;
layout (location = 4) in float rotation;
layout (location = 5) in vec2 tile_dim;

out vec2 tex_pos;

void main()
{
    mat3 rot_matrix = mat3(vec2(cos(rotation), sin(rotation)), 0.0,
                           vec2(-1.0f * sin(rotation), cos(rotation)), 0.0,
                           vec2(0.0, 0.0), 1.0);
    mat4 world_matrix = mat4(vec4(1.0/960.0, 0.0, 0.0, 0.0),
                             vec4(0.0, -1.0/540.0, 0.0, 0.0),
                             vec4(0.0, 0.0, 1.0, 0.0),
                             vec4(0.0, 0.0, 0.0, 1.0));
    tex_pos = tile_pos.xy + vert_position.xy * tile_dim.xy;
    gl_Position = vec4(((rot_matrix * vert_position * size) + offset.xyz), 1.0) * world_matrix + vec4(-1.0, 1.0, 0.0, 0.0);
}
