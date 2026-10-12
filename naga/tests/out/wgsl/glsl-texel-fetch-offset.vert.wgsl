struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0)
var image: texture_2d_array<f32>;
@group(0) @binding(2)
var storage_image: texture_storage_2d<rgba32float,read>;
var<private> coordinate_1: vec3<i32>;
var<private> gl_Position: vec4<f32>;

fn main_1() {
    var a: vec4<f32>;
    var b: vec4<f32>;
    var c: vec4<f32>;

    let _e3 = coordinate_1;
    let _e11 = textureLoad(image, (_e3.xy + vec2<i32>(2i, -1i)), _e3.z, 0i);
    a = _e11;
    let _e13 = coordinate_1;
    let _e17 = textureLoad(image, _e13.xy, _e13.z, 0i);
    b = _e17;
    let _e19 = coordinate_1;
    let _e21 = textureLoad(storage_image, _e19.xy);
    c = _e21;
    let _e24 = a;
    let _e25 = b;
    let _e27 = c;
    gl_Position = ((_e24 + _e25) + _e27);
    return;
}

@vertex
fn main(@location(0) coordinate: vec3<i32>) -> VertexOutput {
    coordinate_1 = coordinate;
    main_1();
    let _e3 = gl_Position;
    return VertexOutput(_e3);
}
