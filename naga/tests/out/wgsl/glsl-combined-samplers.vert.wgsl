struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

@group(0) @binding(0)
var image: texture_2d<f32>;
@group(4294967295) @binding(0)
var image_sampler: sampler;
@group(0) @binding(1)
var signed_image: texture_2d<i32>;
@group(0) @binding(2)
var unsigned_image: texture_2d<u32>;
var<private> gl_Position: vec4<f32>;

fn sample_image(coordinate: vec2<f32>) -> vec4<f32> {
    var coordinate_1: vec2<f32>;

    coordinate_1 = coordinate;
    let _e4 = coordinate_1;
    let _e6 = textureSampleLevel(image, image_sampler, _e4, 0f);
    return _e6;
}

fn main_1() {
    let _e5 = sample_image(vec2(0.5f));
    let _e9 = textureLoad(signed_image, vec2(0i), 0i);
    let _e15 = textureLoad(unsigned_image, vec2(0i), 0i);
    gl_Position = ((_e5 + vec4<f32>(_e9)) + vec4<f32>(_e15));
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
