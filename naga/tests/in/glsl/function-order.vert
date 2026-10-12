#version 450
float later(float x);
float first(float x) {
    if (x > 0.0) { return later(x); }
    for (int i = 0; i < 2; ++i) { x += later(float(i)); }
    switch (int(x)) {
        case 0: x = later(2.0); break;
        default: break;
    }
    return x;
}
float later(float x) { return x + 1.0; }
void main() { gl_Position = vec4(first(1.0)); }
