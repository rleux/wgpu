#version 450
struct Data { highp vec2 value; mediump int index; };
void adjust(inout highp vec2 value, out highp int index) {
    value += vec2(1.0);
    index = 2;
}
float read(highp const in float value) { return value; }
void main() {
    vec2 value = vec2(read(1.0));
    int index;
    adjust(value, index);
    Data data;
    data.value = value;
    data.index = index;
    gl_Position = vec4(data.value, float(data.index), 1.0);
}
