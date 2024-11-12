struct Camera {
    proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(1) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0) pos: vec4<f32>,
    @location(1) color: vec4<f32>,
}


struct VertexOutput {
    @builtin(position) position : vec4<f32>,
    @location(0) color : vec4<f32>,
}


@vertex
fn vs_main(
    @location(0) pos: vec4<f32>, @location(1) color: vec4<f32>
) -> VertexOutput {
    let scale = 0.25;
    var out: VertexOutput;
    out.position = camera.proj * pos * scale + vec4<f32>(light.position,1.0);
    out.color = vec4<f32>(light.color,1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}