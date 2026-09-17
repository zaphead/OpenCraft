struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>,
}

struct Camera {
    view_proj: mat4x4<f32>,
    sun_dir: vec3<f32>,
    brightness: f32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

@vertex
fn vs_main(
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
) -> VsOut {
    var o: VsOut;
    o.clip = camera.view_proj * vec4<f32>(pos, 1.0);
    o.uv = uv;
    o.color = color;
    o.normal = normal;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let sun = normalize(camera.sun_dir);
    let lit = 0.35 + 0.65 * max(dot(normalize(i.normal), sun), 0.0);
    let texel = textureSample(tex, samp, i.uv);
    let rgb = texel.rgb * i.color.rgb * lit * camera.brightness;
    return vec4<f32>(rgb, texel.a * i.color.a);
}
