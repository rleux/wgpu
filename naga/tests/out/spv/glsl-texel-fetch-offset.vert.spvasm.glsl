#version 460
#extension GL_EXT_samplerless_texture_functions : require

struct _10
{
    vec4 _m0;
};

layout(set = 0, binding = 0) uniform texture2DArray _11;
layout(set = 0, binding = 2, rgba32f) uniform readonly image2D _13;

layout(location = 0) in ivec3 _58;
ivec3 _15 = ivec3(0);
vec4 _18 = vec4(0.0);

void _22()
{
    vec4 _30 = vec4(0.0);
    vec4 _33 = vec4(0.0);
    vec4 _35 = vec4(0.0);
    _30 = texelFetch(_11, ivec3(_15.xy + ivec2(2, -1), _15.z), 0);
    _33 = texelFetch(_11, ivec3(_15.xy, _15.z), 0);
    _35 = imageLoad(_13, _15.xy);
    _18 = (_30 + _33) + _35;
}

void main()
{
    _15 = _58;
    _22();
    gl_Position = _10(_18)._m0;
}

