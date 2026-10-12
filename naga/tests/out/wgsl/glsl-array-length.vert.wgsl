struct Data {
    values: array<f32>,
}

struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0)
var<storage, read_write> data: Data;
var<private> gl_Position: vec4<f32>;

fn main_1() {
    var fixed_values: array<f32, 3>;

    gl_Position = vec4(f32((i32(arrayLength((&data.values))) + 3i)));
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
