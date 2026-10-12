struct Data {
    vector: vec2<f32>,
    scalar: f32,
}

struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

var<private> gl_Position: vec4<f32>;

fn change(a: ptr<function, f32>, b: ptr<function, f32>) {
    let _e2 = (*a);
    (*a) = (_e2 + 2f);
    (*b) = 3f;
    return;
}

fn change_pair(value: ptr<function, vec2<f32>>) {
    let _e1 = (*value);
    (*value) = (_e1 + vec2<f32>(4f, 5f));
    return;
}

fn main_1() {
    var value_1: vec3<f32> = vec3(1f);
    var matrix: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 0f), vec2<f32>(0f, 1f));
    var data: Data = Data(vec2(1f), 2f);
    var local: vec2<f32>;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;
    var local_5: f32;
    var local_6: f32;
    var local_7: vec2<f32>;

    let _e15 = data.vector;
    local = _e15;
    change_pair((&local));
    let _e17 = local;
    data.vector = _e17;
    let _e19 = data.scalar;
    local_1 = _e19;
    change((&local_1), (&local_2));
    let _e23 = local_1;
    data.scalar = _e23;
    let _e24 = local_2;
    value_1.x = _e24;
    let _e26 = value_1.x;
    local_3 = _e26;
    change((&local_3), (&local_4));
    let _e30 = local_3;
    value_1.x = _e30;
    let _e31 = local_4;
    value_1.z = _e31;
    let _e34 = matrix[0][1];
    local_5 = _e34;
    change((&local_5), (&local_6));
    let _e39 = local_5;
    matrix[0][1] = _e39;
    let _e40 = local_6;
    matrix[1][0] = _e40;
    let _e41 = value_1;
    local_7 = _e41.yx;
    change_pair((&local_7));
    let _e48 = local_7.x;
    value_1.y = _e48;
    let _e49 = local_7.y;
    value_1.x = _e49;
    let _e51 = value_1;
    let _e53 = matrix[0];
    let _e56 = matrix[1];
    let _e63 = data;
    let _e65 = data;
    gl_Position = (vec4<f32>(_e51.x, _e51.y, _e51.z, (_e53.y + _e56.x)) + vec4<f32>(_e63.vector.x, _e63.vector.y, _e65.scalar, 0f));
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
