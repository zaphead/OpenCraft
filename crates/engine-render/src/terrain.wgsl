struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world: vec3<f32>,
}

struct Camera {
    view_proj: mat4x4<f32>,
    sun_dir: vec3<f32>,
    brightness: f32,
    fog_color: vec3<f32>,
    fog_density: f32,
    sway: f32,
    sway_amp: f32,
    _pad: f32,
    _pad2: f32,
    tint: vec3<f32>,
    _pad3: f32,
    cam_pos: vec3<f32>,
    _pad4: f32,
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
    var wp = pos;
    let flag = color.a;
    if (flag < 0.97) {
        let w = sin(camera.sway + pos.x * 1.7 + pos.z * 1.3) * camera.sway_amp;
        wp.x += w;
        wp.z += w * 0.65;
    }
    if (flag > 0.7 && flag < 0.82) {
        let lean = max(camera.sun_dir.y, 0.0);
        wp.x += camera.sun_dir.x * 0.28 * lean;
        wp.z += camera.sun_dir.z * 0.28 * lean;
    }
    if (flag > 0.35 && flag < 0.45 && camera.brightness >= 0.45) {
        let center = floor(pos) + vec3<f32>(0.5, 0.5, 0.5);
        wp = mix(wp, center, 0.82);
    }
    var o: VsOut;
    o.clip = camera.view_proj * vec4<f32>(wp, 1.0);
    o.uv = uv;
    o.color = color;
    o.normal = normal;
    o.world = wp;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let sun = normalize(camera.sun_dir);
    let emissive = (i.color.a > 0.5 && i.color.a < 0.62) || (i.color.a > 0.35 && i.color.a < 0.45 && camera.brightness < 0.45);
    let lit = 0.35 + 0.65 * max(dot(normalize(i.normal), sun), 0.0);
    let texel = textureSample(tex, samp, i.uv);
    let bright = select(camera.brightness, max(camera.brightness, 0.92), emissive);
    var rgb = texel.rgb * i.color.rgb * lit * bright * camera.tint;
    let dist = distance(i.world, camera.cam_pos);
    let fog = clamp(1.0 - exp(-dist * camera.fog_density), 0.0, 0.82);
    rgb = mix(rgb, camera.fog_color, fog);
    return vec4<f32>(rgb, texel.a);
}
