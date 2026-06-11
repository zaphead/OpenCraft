use bytemuck::{Pod, Zeroable};
use engine_assets::{PlayerSkin, TextureAtlas};

use crate::humanoid_pose::{part_local_matrix, PlayerRender};
use crate::mesh::MeshVertex;
use crate::pipeline::GpuMesh;
use crate::player_model::{HumanoidModelParts, HUMANOID_PART_COUNT};

const PART_UNIFORM_STRIDE: u64 = 256;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PlayerUniform {
    model: [f32; 16],
}

pub struct PlayerPipeline {
    color_pipeline: wgpu::RenderPipeline,
    depth_pipeline: wgpu::RenderPipeline,
    model_buffer: wgpu::Buffer,
    skin_bind_groups: [wgpu::BindGroup; HUMANOID_PART_COUNT],
    _skin_texture: wgpu::Texture,
    gpu_meshes: [GpuMesh; HUMANOID_PART_COUNT],
    pivots: [glam::Vec3; HUMANOID_PART_COUNT],
    part_mask: u32,
    visible: bool,
}

impl PlayerPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_format: wgpu::TextureFormat,
        scene_bind_group_layout: &wgpu::BindGroupLayout,
        lighting_bind_group_layout: &wgpu::BindGroupLayout,
        shadow_bind_group_layout: &wgpu::BindGroupLayout,
        parts: &HumanoidModelParts,
        skin: &PlayerSkin,
    ) -> Self {
        let skin_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("player_skin_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let model_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("player_model_uniforms"),
            size: PART_UNIFORM_STRIDE * HUMANOID_PART_COUNT as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (skin_texture, skin_view, skin_sampler) =
            upload_skin_texture(device, queue, &skin.atlas);
        let skin_bind_groups = std::array::from_fn(|part| {
            create_skin_bind_group(
                device,
                &skin_bind_group_layout,
                &skin_view,
                &skin_sampler,
                &model_buffer,
                part,
            )
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("player_shader"),
            source: crate::shader_source::player_shader_source(),
        });

        let vertex_buffers = vertex_buffer_layout();

        let color_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("player_color_pipeline_layout"),
            bind_group_layouts: &[
                scene_bind_group_layout,
                &skin_bind_group_layout,
                lighting_bind_group_layout,
                shadow_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let depth_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("player_depth_pipeline_layout"),
            bind_group_layouts: &[scene_bind_group_layout, &skin_bind_group_layout],
            push_constant_ranges: &[],
        });

        let primitive = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        };

        let color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("player_color_pipeline"),
            layout: Some(&color_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &vertex_buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_player"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: hdr_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive,
            depth_stencil: Some(color_depth_stencil()),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let depth_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("player_depth_pipeline"),
            layout: Some(&depth_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &vertex_buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_depth"),
                targets: &[],
                compilation_options: Default::default(),
            }),
            primitive,
            depth_stencil: Some(depth_prepass_stencil()),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let gpu_meshes =
            std::array::from_fn(|part| GpuMesh::from_mesh(device, &parts.meshes[part]));

        Self {
            color_pipeline,
            depth_pipeline,
            model_buffer,
            skin_bind_groups,
            _skin_texture: skin_texture,
            gpu_meshes,
            pivots: parts.pivots,
            part_mask: 0,
            visible: false,
        }
    }

    pub fn set_player(&mut self, queue: &wgpu::Queue, player: Option<PlayerRender>) {
        let Some(PlayerRender {
            base,
            pose,
            part_mask,
        }) = player
        else {
            self.visible = false;
            return;
        };
        self.visible = true;
        self.part_mask = part_mask;

        let rotations = pose.part_rotations();
        for (part, rotation) in rotations.into_iter().enumerate() {
            let model = base * part_local_matrix(self.pivots[part], rotation);
            let uniform = PlayerUniform {
                model: model.to_cols_array(),
            };
            queue.write_buffer(
                &self.model_buffer,
                part as u64 * PART_UNIFORM_STRIDE,
                bytemuck::bytes_of(&uniform),
            );
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn draw_depth<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        scene_bind_group: &'a wgpu::BindGroup,
    ) {
        if !self.visible {
            return;
        }
        pass.set_pipeline(&self.depth_pipeline);
        pass.set_bind_group(0, scene_bind_group, &[]);
        self.draw_parts(pass, |pass, part| {
            pass.set_bind_group(1, &self.skin_bind_groups[part], &[]);
        });
    }

    pub fn draw_color<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        scene_bind_group: &'a wgpu::BindGroup,
        lighting_bind_group: &'a wgpu::BindGroup,
        shadow_bind_group: &'a wgpu::BindGroup,
    ) {
        if !self.visible {
            return;
        }
        pass.set_pipeline(&self.color_pipeline);
        pass.set_bind_group(0, scene_bind_group, &[]);
        pass.set_bind_group(2, lighting_bind_group, &[]);
        pass.set_bind_group(3, shadow_bind_group, &[]);
        self.draw_parts(pass, |pass, part| {
            pass.set_bind_group(1, &self.skin_bind_groups[part], &[]);
        });
    }

    fn draw_parts<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        mut bind_skin: impl FnMut(&mut wgpu::RenderPass<'a>, usize),
    ) {
        for part in 0..HUMANOID_PART_COUNT {
            if self.part_mask & (1 << part) == 0 {
                continue;
            }
            bind_skin(pass, part);
            let mesh = &self.gpu_meshes[part];
            pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            pass.set_index_buffer(
                mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }
}

fn vertex_buffer_layout() -> [wgpu::VertexBufferLayout<'static>; 1] {
    [wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: 12,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: 24,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: 32,
                shader_location: 3,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: 40,
                shader_location: 4,
                format: wgpu::VertexFormat::Uint32,
            },
            wgpu::VertexAttribute {
                offset: 44,
                shader_location: 5,
                format: wgpu::VertexFormat::Uint32,
            },
            wgpu::VertexAttribute {
                offset: 48,
                shader_location: 6,
                format: wgpu::VertexFormat::Uint32,
            },
        ],
    }]
}

fn color_depth_stencil() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::LessEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

fn depth_prepass_stencil() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

fn upload_skin_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    atlas: &TextureAtlas,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("player_skin"),
        size: wgpu::Extent3d {
            width: atlas.width.max(1),
            height: atlas.height.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &atlas.pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * atlas.width),
            rows_per_image: Some(atlas.height),
        },
        wgpu::Extent3d {
            width: atlas.width,
            height: atlas.height,
            depth_or_array_layers: 1,
        },
    );
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("player_skin_sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    (texture, view, sampler)
}

fn create_skin_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
    model_buffer: &wgpu::Buffer,
    part: usize,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("player_skin_bind_group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: model_buffer,
                    offset: part as u64 * PART_UNIFORM_STRIDE,
                    size: std::num::NonZeroU64::new(std::mem::size_of::<PlayerUniform>() as u64),
                }),
            },
        ],
    })
}
