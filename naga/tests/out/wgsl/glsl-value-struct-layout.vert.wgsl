struct Values {
    items: array<f32, 2>,
}

struct Nested {
    values: array<Values, 2>,
    offset: vec2<f32>,
}

struct Values_1 {
    items: array<f32, 2>,
}

struct Nested_1 {
    values: array<Values_1, 2>,
    offset: vec2<f32>,
}

struct Input {
    value: Nested_1,
}

struct Output {
    value: Nested_1,
}

struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> source: Input;
@group(0) @binding(1)
var<storage, read_write> target_: Output;
var<private> gl_Position: vec4<f32>;

fn copy_value(value: Values) -> Values {
    var value_1: Values;

    value_1 = value;
    let _e2 = value_1;
    return _e2;
}

fn change(value_2: ptr<function, Values>) {
    let _e6 = (*value_2).items[1];
    (*value_2).items[1i] = (_e6 + 1f);
    return;
}

fn main_1() {
    var local: Nested;
    var items: array<f32, 2> = array<f32, 2>(2f, 3f);
    var local_1: Values;

    let _e3 = source.value;
    local = Nested(array<Values, 2>(Values(array<f32, 2>(_e3.values[0].items[0], _e3.values[0].items[1])), Values(array<f32, 2>(_e3.values[1].items[0], _e3.values[1].items[1]))), _e3.offset);
    let _e29 = items;
    local.values[0i].items = _e29;
    let _e36 = source.value.values[1];
    let _e42 = copy_value(Values(array<f32, 2>(_e36.items[0], _e36.items[1])));
    local.values[1i] = _e42;
    let _e44 = local;
    target_.value = Nested_1(array<Values_1, 2>(Values_1(array<f32, 2>(_e44.values[0].items[0], _e44.values[0].items[1])), Values_1(array<f32, 2>(_e44.values[1].items[0], _e44.values[1].items[1]))), _e44.offset);
    let _e65 = target_.value.values[0i];
    local_1 = Values(array<f32, 2>(_e65.items[0], _e65.items[1]));
    change((&local_1));
    let _e72 = local_1;
    target_.value.values[0i] = Values_1(array<f32, 2>(_e72.items[0], _e72.items[1]));
    let _e83 = local.values[0].items[1];
    let _e88 = local.values[1].items[1];
    let _e90 = local.offset;
    gl_Position = vec4<f32>(_e83, _e88, _e90.x, _e90.y);
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
