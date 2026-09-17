struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Ui {
    screen: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> ui: Ui;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

@vertex
fn vs_main(
    @location(0) pos: vec3<f32>,
    @location(1) _n: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
) -> VsOut {
    var o: VsOut;
    let x = (pos.x / ui.screen.x) * 2.0 - 1.0;
    let y = 1.0 - (pos.y / ui.screen.y) * 2.0;
    o.clip = vec4<f32>(x, y, pos.z, 1.0);
    o.uv = uv;
    o.color = color;
    return o;
}

@fragment
fn fs_main(i: VsOut) -> @location(0) vec4<f32> {
    let texel = textureSample(tex, samp, i.uv);
    return texel * i.color;
}
