#version 460

struct _6
{
    vec2 _m0;
    int _m1;
};

struct _10
{
    vec4 _m0;
};

vec4 _11 = vec4(0.0);

float _27(float _26)
{
    return _26;
}

void _17(inout vec2 _15, out int _16)
{
    _15 += vec2(1.0);
    _16 = 2;
}

void _31()
{
    vec2 _33 = vec2(0.0);
    int _35 = 0;
    _6 _37 = _6(vec2(0.0), 0);
    _33 = vec2(_27(1.0));
    _17(_33, _35);
    _37._m0 = _33;
    _37._m1 = _35;
    _11 = vec4(_37._m0, float(_37._m1), 1.0);
}

void main()
{
    _31();
    gl_Position = _10(_11)._m0;
}

