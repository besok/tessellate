struct Uniforms {
    delegate : mat4x4<f32>,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position : vec4<f32>,
    @location(0) color : vec4<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32>) -> Output {
    var output: Output;
    output.position = uniforms.delegate * pos;
    output.color = color;
    return output;
}

@fragment
fn fs_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}