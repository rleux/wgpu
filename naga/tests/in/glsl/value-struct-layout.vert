#version 450
struct Values { float items[2]; };
struct Nested { Values values[2]; vec2 offset; };
layout(set=0,binding=0,std140) uniform Input { Nested value; } source;
layout(set=0,binding=1,std140) buffer Output { Nested value; } target;
Values copy_value(Values value) { return value; }
void change(inout Values value) { value.items[1] += 1.0; }
void main() {
    Nested local = source.value;
    float items[2] = float[2](2.0, 3.0);
    local.values[0].items = items;
    local.values[1] = copy_value(source.value.values[1]);
    target.value = local;
    change(target.value.values[0]);
    gl_Position = vec4(local.values[0].items[1], local.values[1].items[1], local.offset);
}
