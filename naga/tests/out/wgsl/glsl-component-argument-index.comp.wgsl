struct Output {
    values: array<u32, 12>,
}

@group(0) @binding(0)
var<storage, read_write> output_data: Output;

fn increment(value: ptr<function, f32>) {
    let _e1 = (*value);
    (*value) = (_e1 + 1f);
    return;
}

fn assign(value_1: ptr<function, f32>) {
    (*value_1) = 5f;
    return;
}

fn add(value_2: ptr<function, f32>, amount: f32) {
    var amount_1: f32;

    amount_1 = amount;
    let _e3 = (*value_2);
    let _e4 = amount_1;
    (*value_2) = (_e3 + _e4);
    return;
}

fn change_pair(first: ptr<function, f32>, second: ptr<function, f32>) {
    (*first) = 3f;
    (*second) = 4f;
    return;
}

fn record(offset: i32, first_1: f32, second_1: f32, index: i32) {
    var offset_1: i32;
    var first_2: f32;
    var second_2: f32;
    var index_1: i32;

    offset_1 = offset;
    first_2 = first_1;
    second_2 = second_1;
    index_1 = index;
    let _e9 = offset_1;
    let _e12 = first_2;
    output_data.values[_e9] = u32(_e12);
    let _e14 = offset_1;
    let _e19 = second_2;
    output_data.values[(_e14 + 1i)] = u32(_e19);
    let _e21 = offset_1;
    let _e26 = index_1;
    output_data.values[(_e21 + 2i)] = u32(_e26);
    return;
}

fn main_1() {
    var matrix: mat4x4<f32> = mat4x4<f32>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
    var index_2: i32 = 0i;
    var local: f32;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;

    let _e10 = index_2;
    index_2 = (_e10 + 1i);
    let _e15 = matrix[_e10][0];
    local = _e15;
    increment((&local));
    let _e17 = local;
    matrix[_e10][0] = _e17;
    let _e20 = matrix[0];
    let _e23 = matrix[1];
    let _e25 = index_2;
    record(0i, _e20.x, _e23.x, _e25);
    matrix = mat4x4<f32>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
    index_2 = 0i;
    let _e34 = index_2;
    index_2 = (_e34 + 1i);
    assign((&local_1));
    let _e40 = local_1;
    matrix[_e34][1] = _e40;
    let _e43 = matrix[0];
    let _e46 = matrix[1];
    let _e48 = index_2;
    record(3i, _e43.y, _e46.y, _e48);
    matrix = mat4x4<f32>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
    matrix[0i][0] = 2f;
    index_2 = 0i;
    let _e61 = index_2;
    index_2 = (_e61 + 1i);
    let _e66 = matrix[_e61][0];
    matrix[0i][0] = 9f;
    local_2 = _e66;
    add((&local_2), 9f);
    let _e72 = local_2;
    matrix[_e61][0] = _e72;
    let _e75 = matrix[0];
    let _e78 = matrix[1];
    let _e80 = index_2;
    record(6i, _e75.x, _e78.x, _e80);
    matrix = mat4x4<f32>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
    index_2 = 0i;
    let _e89 = index_2;
    index_2 = (_e89 + 1i);
    let _e94 = matrix[_e89][0];
    let _e95 = index_2;
    index_2 = (_e95 + 1i);
    let _e100 = matrix[_e95][1];
    local_3 = _e94;
    local_4 = _e100;
    change_pair((&local_3), (&local_4));
    let _e103 = local_3;
    matrix[_e89][0] = _e103;
    let _e104 = local_4;
    matrix[_e95][1] = _e104;
    let _e107 = matrix[0];
    let _e110 = matrix[1];
    let _e112 = index_2;
    record(9i, _e107.x, _e110.y, _e112);
    return;
}

@compute @workgroup_size(1, 1, 1)
fn main() {
    main_1();
    return;
}
