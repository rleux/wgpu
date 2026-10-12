#version 450
void change(inout float a, out float b) { a += 2.0; b = 3.0; }
void change_pair(inout vec2 value) { value += vec2(4.0, 5.0); }
struct Data { vec2 vector; float scalar; };
void main() {
    vec3 value = vec3(1.0);
    mat2 matrix = mat2(1.0);
    Data data = Data(vec2(1.0), 2.0);
    change_pair(data.vector);
    change(data.scalar, value.x);
    change(value.x, value.z);
    change(matrix[0].y, matrix[1].x);
    change_pair(value.yx);
    gl_Position = vec4(value, matrix[0].y + matrix[1].x) + vec4(data.vector, data.scalar, 0.0);
}
