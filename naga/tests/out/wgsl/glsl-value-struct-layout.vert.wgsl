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

struct Input_1 {
    value: Nested,
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

    let _e2 = source;
    local = Input_1(Nested(array<Values, 2>(Values(array<f32, 2>(_e2.value.values[0].items[0], _e2.value.values[0].items[1])), Values(array<f32, 2>(_e2.value.values[1].items[0], _e2.value.values[1].items[1]))), _e2.value.offset)).value;
    let _e31 = items;
    local.values[0i].items = _e31;
    let _e38 = source.value.values[1];
    let _e44 = copy_value(Values(array<f32, 2>(_e38.items[0], _e38.items[1])));
    local.values[1i] = _e44;
    let _e46 = local;
    target_.value = Nested_1(array<Values_1, 2>(Values_1(array<f32, 2>(_e46.values[0].items[0], _e46.values[0].items[1])), Values_1(array<f32, 2>(_e46.values[1].items[0], _e46.values[1].items[1]))), _e46.offset);
    let _e67 = target_.value.values[0i];
    local_1 = Values(array<f32, 2>(_e67.items[0], _e67.items[1]));
    change((&local_1));
    let _e74 = local_1;
    target_.value.values[0i] = Values_1(array<f32, 2>(_e74.items[0], _e74.items[1]));
    let _e85 = local.values[0].items[1];
    let _e90 = local.values[1].items[1];
    let _e91 = local;
    gl_Position = vec4<f32>(_e85, _e90, _e91.offset.x, _e91.offset.y);
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
