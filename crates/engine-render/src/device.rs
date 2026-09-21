use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::submit::{MeshData, Vertex};
use thiserror::Error;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("no wgpu adapter")]
    Adapter,
    #[error("surface: {0}")]
    Surface(String),
    #[error("device: {0}")]
    Device(String),
}

pub(crate) struct GpuMesh {
    pub(crate) verts: wgpu::Buffer,
    pub(crate) inds: wgpu::Buffer,
    pub(crate) nidx: u32,
}

pub(crate) struct Gpu {
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) depth: wgpu::TextureView,
    pub(crate) terrain_pipe: wgpu::RenderPipeline,
    pub(crate) ui_pipe: wgpu::RenderPipeline,
    pub(crate) cam_buf: wgpu::Buffer,
    pub(crate) ui_buf: wgpu::Buffer,
    pub(crate) cam_bg: wgpu::BindGroup,
    pub(crate) ui_bg: wgpu::BindGroup,
    tex_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pub(crate) white_bg: wgpu::BindGroup,
    pub(crate) size: (u32, u32),
}

fn block_on<F: Future>(fut: F) -> F::Output {
    fn raw() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            raw()
        }
        fn nop(_: *const ()) {}
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, nop, nop, nop);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    let waker = unsafe {
        // SAFETY: vtable nops; wgpu's native adapter future completes without parking.
        Waker::from_raw(raw())
    };
    let mut cx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

impl Gpu {
    pub(crate) fn new(window: &Window) -> Result<Self, RenderError> {
        let size = window.inner_size();
        let w = size.width.max(1);
        let h = size.height.max(1);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .map_err(|e| RenderError::Surface(e.to_string()))?;
        // SAFETY: the window outlives the renderer (owned by the client event loop).
        let surface: wgpu::Surface<'static> = unsafe { std::mem::transmute(surface) };

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(RenderError::Adapter)?;

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("opencraft"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ))
        .map_err(|e| RenderError::Device(e.to_string()))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let present = if caps.present_modes.contains(&wgpu::PresentMode::Immediate) {
            wgpu::PresentMode::Immediate
        } else if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox
        } else {
            wgpu::PresentMode::AutoVsync
        };
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: w,
            height: h,
            present_mode: present,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);
        let depth = make_depth(&device, w, h);

        let cam_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cam"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                // Vertex reads view_proj; fragment reads sun_dir/brightness.
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let tex_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("tex"),
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
            ],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let cam_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cam buf"),
            // view_proj (64) + sun (16) + fog (16) + sway (16) + tint (16) + cam (16).
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let ui_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ui buf"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let cam_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cam bg"),
            layout: &cam_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cam_buf.as_entire_binding(),
            }],
        });
        let ui_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui bg"),
            layout: &cam_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: ui_buf.as_entire_binding(),
            }],
        });

        let white = device.create_texture_with_data(
            &queue,
            &wgpu::TextureDescriptor {
                label: Some("white"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &[255, 255, 255, 255],
        );
        let white_view = white.create_view(&wgpu::TextureViewDescriptor::default());
        let white_bg = bind_tex(&device, &tex_layout, &white_view, &sampler);

        let vlayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x4],
        };

        let terrain_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("terrain"),
            source: wgpu::ShaderSource::Wgsl(include_str!("terrain.wgsl").into()),
        });
        let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ui"),
            source: wgpu::ShaderSource::Wgsl(include_str!("ui.wgsl").into()),
        });

        let tlayout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("t"),
            bind_group_layouts: &[&cam_layout, &tex_layout],
            push_constant_ranges: &[],
        });

        let terrain_pipe = make_pipeline(
            &device,
            &tlayout,
            &terrain_shader,
            &vlayout,
            format,
            Some(wgpu::Face::Back),
            true,
            "terrain pipe",
        );

        let ui_pipe = make_pipeline(
            &device,
            &tlayout,
            &ui_shader,
            &vlayout,
            format,
            None,
            false,
            "ui pipe",
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            depth,
            terrain_pipe,
            ui_pipe,
            cam_buf,
            ui_buf,
            cam_bg,
            ui_bg,
            tex_layout,
            sampler,
            white_bg,
            size: (w, h),
        })
    }

    pub(crate) fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.size = (w, h);
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
        self.depth = make_depth(&self.device, w, h);
    }

    pub(crate) fn upload_rgba(&self, width: u32, height: u32, data: &[u8]) -> wgpu::TextureView {
        let tex = self.device.create_texture_with_data(
            &self.queue,
            &wgpu::TextureDescriptor {
                label: Some("tex"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            data,
        );
        tex.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub(crate) fn mesh(&self, data: &MeshData) -> Option<GpuMesh> {
        if data.indices.is_empty() {
            return None;
        }
        let verts = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh v"),
            contents: bytemuck::cast_slice(&data.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let inds = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh i"),
            contents: bytemuck::cast_slice(&data.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Some(GpuMesh {
            verts,
            inds,
            nidx: data.indices.len() as u32,
        })
    }

    pub(crate) fn tex_bg(&self, view: &wgpu::TextureView) -> wgpu::BindGroup {
        bind_tex(&self.device, &self.tex_layout, view, &self.sampler)
    }
}

fn bind_tex(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("tex bg"),
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
        ],
    })
}

fn make_depth(device: &wgpu::Device, w: u32, h: u32) -> wgpu::TextureView {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    t.create_view(&wgpu::TextureViewDescriptor::default())
}

#[allow(clippy::too_many_arguments)]
fn make_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vlayout: &wgpu::VertexBufferLayout,
    format: wgpu::TextureFormat,
    cull: Option<wgpu::Face>,
    depth: bool,
    label: &'static str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: std::slice::from_ref(vlayout),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: cull,
            ..Default::default()
        },
        depth_stencil: depth.then(|| wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

