struct Camera {
    proj: mat4x4<f32>,
    eye_pos: vec4<f32>,
}

struct Light {
    position: vec3<f32>,
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
}

struct MaterialUniforms {
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
    shininess: f32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> light: Light;
@group(2) @binding(0) var<uniform> material: MaterialUniforms;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,  // Added for proper lighting calculations
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to clip space
    out.clip_position = camera.proj * in.position;

    // Store world position for lighting calculations
    out.world_position = in.position.xyz;

    // Pass through color
    out.color = in.color;

    // Pass through normal (assuming it's already in world space)
    out.normal = normalize(in.normal.xyz);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize vectors
    let N = normalize(in.normal);
    let V = normalize(camera.eye_pos.xyz - in.world_position);
    let L = normalize(light.position - in.world_position);
    let H = normalize(L + V);

    // Calculate distance for attenuation
    let distance = length(light.position - in.world_position);
    let attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);

    // Calculate lighting components
    let base_color = in.color.rgb;

    // Ambient
    let ambient = light.ambient * (material.ambient * base_color);

    // Diffuse
    let diff = max(dot(N, L), 0.0);
    let diffuse = light.diffuse * (diff * material.diffuse * base_color);

    // Specular
    let spec = pow(max(dot(N, H), 0.0), material.shininess);
    let specular = light.specular * (spec * material.specular);

    // Combine all components
    let final_color = (ambient + diffuse + specular) * attenuation;

    return vec4<f32>(final_color, in.color.a);
}