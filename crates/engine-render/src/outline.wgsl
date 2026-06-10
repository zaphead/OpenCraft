struct SceneUniform {
    view_proj: mat4x4<f32>,
    animation_tick: u32,
    _align_colormap: u32,
    colormap_min: vec2<f32>,
    colormap_max: vec2<f32>,
    _struct_pad: vec2<u32>,
};

@group(0) @binding(0)
var<uniform> scene: SceneUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> @builtin(position) vec4<f32> {
    return scene.view_proj * vec4<f32>(input.position, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.5);
}
