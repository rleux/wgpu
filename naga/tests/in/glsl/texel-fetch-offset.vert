#version 450
layout(set=0,binding=0) uniform texture2DArray image;
layout(set=0,binding=1) uniform sampler sampling;
layout(set=0,binding=2,rgba32f) uniform readonly image2D storage_image;
layout(location=0) in ivec3 coordinate;
void main() {
    vec4 a = texelFetchOffset(sampler2DArray(image, sampling), coordinate, 0, ivec2(2,-1));
    vec4 b = texelFetch(sampler2DArray(image, sampling), coordinate, 0);
    vec4 c = imageLoad(storage_image, coordinate.xy);
    gl_Position = a + b + c;
}
