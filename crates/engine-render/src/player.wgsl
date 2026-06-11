struct SceneUniform {
    view_proj: mat4x4<f32>,
    animation_tick: u32,
    _align_colormap: u32,
    colormap_min: vec2<f32>,
    colormap_max: vec2<f32>,
    _struct_pad: vec2<u32>,
};

struct PlayerUniform {
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> scene: SceneUniform;

@group(1) @binding(0)
var skin_tex: texture_2d<f32>;
@group(1) @binding(1)
var skin_sampler: sampler;
@group(1) @binding(2)
var<uniform> player: PlayerUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv2: vec2<f32>,
    @location(4) tint_index: u32,
    @location(5) flags: u32,
    @location(6) anim_packed: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = (player.model * vec4<f32>(input.position, 1.0)).xyz;
    let world_normal = normalize((player.model * vec4<f32>(input.normal, 0.0)).xyz);
    out.clip_position = scene.view_proj * vec4<f32>(world_position, 1.0);
    out.uv = input.uv;
    out.normal = world_normal;
    out.world_position = world_position;
    return out;
}

@fragment
fn fs_player(input: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(skin_tex, skin_sampler, input.uv);
    if base.a < 0.1 {
        discard;
    }
    return vec4<f32>(shade_lit(base.rgb, input.normal, input.world_position), 1.0);
}

@fragment
fn fs_depth(_input: VertexOutput) {
}
