struct Data {
    value: vec2<f32>,
    index: i32,
}

struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

var<private> gl_Position: vec4<f32>;

fn adjust(value: ptr<function, vec2<f32>>, index: ptr<function, i32>) {
    let _e2 = (*value);
    (*value) = (_e2 + vec2(1f));
    (*index) = 2i;
    return;
}

fn read_(value_1: f32) -> f32 {
    return value_1;
}

fn main_1() {
    var value_2: vec2<f32>;
    var index_1: i32;
    var data: Data;

    let _e1 = read_(1f);
    value_2 = vec2(_e1);
    adjust((&value_2), (&index_1));
    let _e7 = value_2;
    data.value = _e7;
    let _e9 = index_1;
    data.index = _e9;
    let _e11 = data;
    let _e13 = data;
    gl_Position = vec4<f32>(_e11.value.x, _e11.value.y, f32(_e13.index), 1f);
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
