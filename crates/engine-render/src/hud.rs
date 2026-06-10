use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct HudVertex {
    pos: [f32; 2],
    color: [f32; 4],
}

const MAX_HUD_LINES: usize = 18;
const MAX_HUD_CHARS_PER_LINE: usize = 24;
const MAX_HUD_VERTICES: usize = MAX_HUD_LINES * MAX_HUD_CHARS_PER_LINE * 5 * 7 * 6 + 6;
const HUD_VERTEX_BUFFER_SIZE: u64 =
    (MAX_HUD_VERTICES * std::mem::size_of::<HudVertex>()) as u64;

const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.72];
const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.95];
const CROSSHAIR_OUTLINE: [f32; 4] = [0.0, 0.0, 0.0, 0.82];
const CROSSHAIR_CORE: [f32; 4] = [1.0, 1.0, 1.0, 0.92];

pub struct HudPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    crosshair_vertices: Vec<HudVertex>,
    crosshair_size: (u32, u32),
}

impl HudPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hud_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("hud.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hud_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("hud_pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<HudVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 8,
                            shader_location: 1,
                        },
                    ],
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
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hud_vertex_buffer"),
            size: HUD_VERTEX_BUFFER_SIZE,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertex_count: 0,
            crosshair_vertices: Vec::new(),
            crosshair_size: (0, 0),
        }
    }

    pub fn set_text(&mut self, queue: &wgpu::Queue, text: &str, width: u32, height: u32) {
        if self.crosshair_size != (width, height) {
            self.crosshair_vertices.clear();
            append_crosshair(&mut self.crosshair_vertices, width, height);
            self.crosshair_size = (width, height);
        }
        let mut vertices = build_hud_vertices(text, width, height);
        vertices.extend_from_slice(&self.crosshair_vertices);
        vertices.truncate(MAX_HUD_VERTICES);
        self.vertex_count = vertices.len() as u32;
        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.vertex_count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }
}

fn build_hud_vertices(text: &str, width: u32, height: u32) -> Vec<HudVertex> {
    const CELL: f32 = 5.0;
    const SCALE: f32 = 2.0;
    const PADDING: f32 = 16.0;
    const BG_PAD: f32 = 8.0;
    const GAP: f32 = 1.0;
    const LINE_GAP: f32 = 10.0;
    const LINE_HEIGHT: f32 = CELL * SCALE + LINE_GAP;
    const CHAR_WIDTH: f32 = (CELL + GAP) * SCALE;

    let lines: Vec<&str> = text.lines().take(MAX_HUD_LINES).collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let max_line_chars = lines
        .iter()
        .map(|line| line.chars().count().min(MAX_HUD_CHARS_PER_LINE))
        .max()
        .unwrap_or(0) as f32;
    let panel_w = max_line_chars * CHAR_WIDTH + BG_PAD * 2.0;
    let panel_h = lines.len() as f32 * LINE_HEIGHT - LINE_GAP + BG_PAD * 2.0;

    let mut vertices = Vec::new();
    push_quad(
        &mut vertices,
        PADDING - BG_PAD,
        PADDING - BG_PAD,
        PADDING - BG_PAD + panel_w,
        PADDING - BG_PAD + panel_h,
        width,
        height,
        BG_COLOR,
    );

    for (line_index, line) in lines.iter().enumerate() {
        let py = PADDING + line_index as f32 * LINE_HEIGHT;
        let mut cursor_x = PADDING;

        for ch in line.chars().take(MAX_HUD_CHARS_PER_LINE) {
            if ch == ' ' {
                cursor_x += CHAR_WIDTH;
                continue;
            }
            let Some(glyph) = glyph_rows(ch) else {
                continue;
            };
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..CELL as usize {
                    if bits & (1 << (CELL as usize - 1 - col)) != 0 {
                        let x0 = cursor_x + col as f32 * SCALE;
                        let y0 = py + row as f32 * SCALE;
                        let x1 = x0 + SCALE;
                        let y1 = y0 + SCALE;
                        push_quad(&mut vertices, x0, y0, x1, y1, width, height, TEXT_COLOR);
                    }
                }
            }
            cursor_x += CHAR_WIDTH;
        }
    }

    vertices
}

fn push_quad(
    vertices: &mut Vec<HudVertex>,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    width: u32,
    height: u32,
    color: [f32; 4],
) {
    let w = width.max(1) as f32;
    let h = height.max(1) as f32;
    let vtx = |x: f32, y: f32| HudVertex {
        pos: [(x / w) * 2.0 - 1.0, 1.0 - (y / h) * 2.0],
        color,
    };
    vertices.push(vtx(x0, y0));
    vertices.push(vtx(x1, y0));
    vertices.push(vtx(x0, y1));
    vertices.push(vtx(x1, y0));
    vertices.push(vtx(x1, y1));
    vertices.push(vtx(x0, y1));
}

fn append_crosshair(vertices: &mut Vec<HudVertex>, width: u32, height: u32) {
    let cx = width.max(1) as f32 * 0.5;
    let cy = height.max(1) as f32 * 0.5;
    const ARM: f32 = 11.0;
    const GAP: f32 = 3.0;
    const CORE: f32 = 1.25;
    const OUTLINE: f32 = 2.75;

    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - ARM - 1.0,
        cy - OUTLINE,
        cx - GAP + 1.0,
        cy + OUTLINE,
        CROSSHAIR_OUTLINE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx + GAP - 1.0,
        cy - OUTLINE,
        cx + ARM + 1.0,
        cy + OUTLINE,
        CROSSHAIR_OUTLINE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - OUTLINE,
        cy - ARM - 1.0,
        cx + OUTLINE,
        cy - GAP + 1.0,
        CROSSHAIR_OUTLINE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - OUTLINE,
        cy + GAP - 1.0,
        cx + OUTLINE,
        cy + ARM + 1.0,
        CROSSHAIR_OUTLINE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - ARM,
        cy - CORE,
        cx - GAP,
        cy + CORE,
        CROSSHAIR_CORE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx + GAP,
        cy - CORE,
        cx + ARM,
        cy + CORE,
        CROSSHAIR_CORE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - CORE,
        cy - ARM,
        cx + CORE,
        cy - GAP,
        CROSSHAIR_CORE,
    );
    push_crosshair_bar(
        vertices,
        width,
        height,
        cx - CORE,
        cy + GAP,
        cx + CORE,
        cy + ARM,
        CROSSHAIR_CORE,
    );
}

fn push_crosshair_bar(
    vertices: &mut Vec<HudVertex>,
    width: u32,
    height: u32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: [f32; 4],
) {
    push_quad(vertices, x0, y0, x1, y1, width, height, color);
}

fn glyph_rows(ch: char) -> Option<[u8; 7]> {
    match ch.to_ascii_uppercase() {
        '0' => Some([0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110]),
        '1' => Some([0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110]),
        '2' => Some([0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111]),
        '3' => Some([0b11110, 0b00001, 0b00010, 0b00110, 0b00001, 0b10001, 0b01110]),
        '4' => Some([0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010]),
        '5' => Some([0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110]),
        '6' => Some([0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110]),
        '7' => Some([0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000]),
        '8' => Some([0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110]),
        '9' => Some([0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100]),
        '-' => Some([0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000]),
        '.' => Some([0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100]),
        ':' => Some([0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000]),
        'A' => Some([0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001]),
        'C' => Some([0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110]),
        'D' => Some([0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110]),
        'E' => Some([0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111]),
        'F' => Some([0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000]),
        'G' => Some([0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110]),
        'H' => Some([0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001]),
        'I' => Some([0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110]),
        'J' => Some([0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100]),
        'K' => Some([0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001]),
        'L' => Some([0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111]),
        'M' => Some([0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001, 0b10001]),
        'N' => Some([0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001]),
        'O' => Some([0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110]),
        'P' => Some([0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000]),
        'R' => Some([0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001]),
        'S' => Some([0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110]),
        'T' => Some([0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100]),
        'U' => Some([0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110]),
        'V' => Some([0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100]),
        'W' => Some([0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001]),
        'X' => Some([0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001]),
        'Y' => Some([0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100]),
        'Z' => Some([0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111]),
        _ => None,
    }
}
