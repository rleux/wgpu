#version 450
layout(set=0,binding=0) uniform sampler2D image;
layout(set=0,binding=1) uniform isampler2D signed_image;
layout(set=0,binding=2) uniform usampler2D unsigned_image;
vec4 sample_image(vec2 coordinate) { return textureLod(image, coordinate, 0.0); }
void main() {
    gl_Position = sample_image(vec2(0.5))
        + vec4(texelFetch(signed_image, ivec2(0), 0))
        + vec4(texelFetch(unsigned_image, ivec2(0), 0));
}
