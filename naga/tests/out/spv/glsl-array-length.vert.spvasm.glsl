#version 460

struct _10
{
    vec4 _m0;
};

layout(set = 0, binding = 0, std430) buffer _5_11
{
    float _m0[];
} _11;

vec4 _13 = vec4(0.0);

void _17()
{
    float _21[3] = float[](0.0, 0.0, 0.0);
    _13 = vec4(float(int(uint(_11._m0.length())) + 3));
}

void main()
{
    _17();
    gl_Position = _10(_13)._m0;
}

