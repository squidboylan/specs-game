#version 330 core
in vec2 tex_pos;

out vec4 FragColor;

uniform sampler2D tex;
//uniform vec3 color;

void main()
{
    float tmp = texture(tex, tex_pos).r;
    //FragColor = vec4(color, tmp);
    FragColor = vec4(1.0, 1.0, 0.0, tmp);
}
