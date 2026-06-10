use bytemuck::{Pod, Zeroable};
use engine_world::BlockPos;
use glam::Vec3;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct OutlineVertex {
    position: [f32; 3],
}

pub struct OutlinePipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uploaded_block: Option<BlockPos>,
}

impl OutlinePipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        scene_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("outline_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("outline_pipeline_layout"),
            bind_group_layouts: &[scene_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("outline_pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<OutlineVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let (vertices, indices) = block_outline_geometry(BlockPos::new(0, 0, 0));
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("outline_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("outline_index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uploaded_block: None,
        }
    }

    pub fn sync_block(&mut self, queue: &wgpu::Queue, block: Option<BlockPos>) {
        if self.uploaded_block == block {
            return;
        }
        self.uploaded_block = block;
        let Some(block) = block else {
            return;
        };
        let (vertices, indices) = block_outline_geometry(block);
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
    }

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        scene_bind_group: &'a wgpu::BindGroup,
        index_count: u32,
    ) {
        if index_count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, scene_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..index_count, 0, 0..1);
    }

    pub fn index_count_for_block() -> u32 {
        block_outline_geometry(BlockPos::new(0, 0, 0)).1.len() as u32
    }
}

fn block_outline_geometry(block: BlockPos) -> (Vec<OutlineVertex>, Vec<u16>) {
    let o = block.0.as_vec3();
    let p = [
        Vec3::new(o.x, o.y, o.z),
        Vec3::new(o.x + 1.0, o.y, o.z),
        Vec3::new(o.x + 1.0, o.y + 1.0, o.z),
        Vec3::new(o.x, o.y + 1.0, o.z),
        Vec3::new(o.x, o.y, o.z + 1.0),
        Vec3::new(o.x + 1.0, o.y, o.z + 1.0),
        Vec3::new(o.x + 1.0, o.y + 1.0, o.z + 1.0),
        Vec3::new(o.x, o.y + 1.0, o.z + 1.0),
    ];
    let edges: [(usize, usize); 12] = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    let vertices: Vec<OutlineVertex> = p
        .iter()
        .map(|v| OutlineVertex {
            position: v.to_array(),
        })
        .collect();
    let mut indices = Vec::with_capacity(edges.len() * 2);
    for (a, b) in edges {
        indices.push(a as u16);
        indices.push(b as u16);
    }
    (vertices, indices)
}
