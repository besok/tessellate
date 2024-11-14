struct Camera {
    proj: mat4x4<f32>,
    eye_pos: vec4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;


struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}
@group(1) @binding(0)
var<uniform> light: Light;

struct MaterialUniforms {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
};
@group(2) @binding(0) var<uniform> material : MaterialUniforms;


fn diffuse_specular(N:vec3<f32>, L:vec3<f32>, V:vec3<f32>) -> vec2<f32>{
    let H = normalize(L + V);
    var diffuse = material.diffuse * max(dot(N, L), 0.0);
    var specular = material.specular * pow(max(dot(N, H), 0.0), material.shininess);
    return vec2<f32>(diffuse, specular);
}


struct VertexInput {
    @location(0) pos: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec4<f32>,
}


struct VertexOutput {
    @builtin(position) pos : vec4<f32>,
    @location(0) color : vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) eye_pos: vec4<f32>,

};

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32> , @location(2) normal: vec4<f32>) -> VertexOutput {
       var out: VertexOutput;
       out.pos = camera.proj * pos;
       out.color = color;
       out.normal = normal;
       out.eye_pos = camera.eye_pos - pos;
    return out;
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>,
           @location(0) color: vec4<f32>,
           @location(1) normal: vec4<f32>,
           @location(2) eye_pos: vec4<f32>) -> @location(0) vec4<f32> {
    // Normalize vectors needed for lighting calculations
    let N = normalize(normal.xyz);
    let V = normalize(eye_pos.xyz);
    let L = normalize(light.position - pos.xyz);

    // Calculate diffuse and specular components
    let ds = diffuse_specular(N, L, V);
    let diffuse = ds.x;
    let specular = ds.y;

    // Calculate ambient component
    let ambient = material.ambient;

    // Combine all lighting components
    let finalColor = color.rgb * (ambient + diffuse) + light.color * specular;

    return vec4<f32>(finalColor, color.a);
}