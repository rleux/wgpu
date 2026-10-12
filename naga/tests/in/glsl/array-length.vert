#version 450
layout(set=0,binding=0,std430) buffer Data { float values[]; } data;
void main() {
    float fixed_values[3];
    gl_Position = vec4(float(data.values.length() + fixed_values.length()));
}
