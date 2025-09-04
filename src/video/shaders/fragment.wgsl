// Fragment shader for emulator video output

@group(0) @binding(0)
var texture_sampler: sampler;

@group(0) @binding(1)
var texture_view: texture_2d<f32>;

struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
}

@fragment
fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    return textureSample(texture_view, texture_sampler, in.tex_coords);
}
