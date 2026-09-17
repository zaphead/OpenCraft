use crate::device::{Gpu, GpuMesh, RenderError};
use crate::emit::{emit_held, emit_items, emit_particles, emit_player, emit_ui_quad};
use crate::submit::{FrameSubmit, MeshData, TextureId};
use engine_core::ChunkPos;
use rustc_hash::FxHashMap;

pub struct Renderer {
    gpu: Gpu,
    chunk_meshes: FxHashMap<ChunkPos, GpuMesh>,
    atlas: Option<usize>,
    ui_tex: Option<usize>,
    skin: Option<usize>,
    textures: Vec<wgpu::TextureView>,
}

impl Renderer {
    pub(crate) fn from_gpu(gpu: Gpu) -> Self {
        Self {
            gpu,
            chunk_meshes: FxHashMap::default(),
            atlas: None,
            ui_tex: None,
            skin: None,
            textures: Vec::new(),
        }
    }
}

impl Renderer {
    pub fn resize(&mut self, w: u32, h: u32) {
        self.gpu.resize(w, h);
    }

    pub fn upload_rgba(&mut self, w: u32, h: u32, data: &[u8]) -> TextureId {
        let view = self.gpu.upload_rgba(w, h, data);
        let id = TextureId(self.textures.len() as u32);
        self.textures.push(view);
        id
    }

    pub fn set_atlas(&mut self, id: TextureId) {
        self.atlas = Some(id.0 as usize);
    }

    pub fn set_ui_atlas(&mut self, id: TextureId) {
        self.ui_tex = Some(id.0 as usize);
    }

    pub fn set_skin(&mut self, id: TextureId) {
        self.skin = Some(id.0 as usize);
    }

    pub fn submit(&mut self, frame: FrameSubmit) -> Result<(), RenderError> {
        self.sync_chunks(&frame);
        let surface = self.acquire_surface()?;
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.write_uniforms(&frame);
        let atlas_bg = self.tex(self.atlas);
        let ui_bg_tex = self.tex(self.ui_tex);
        let skin_bg = self.tex(self.skin);

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("f") });
        let dynamic = self.emit_dynamic(&frame);
        Self::draw_world(
            &mut encoder,
            &view,
            &self.gpu,
            &self.chunk_meshes,
            frame.clear,
            &dynamic,
            &atlas_bg,
            &skin_bg,
        );
        Self::draw_ui(&mut encoder, &view, &self.gpu, &frame.ui, &ui_bg_tex);

        self.gpu.queue.submit(Some(encoder.finish()));
        surface.present();
        Ok(())
    }

    fn sync_chunks(&mut self, frame: &FrameSubmit) {
        for pos in &frame.remove_chunks {
            self.chunk_meshes.remove(pos);
        }
        for (pos, mesh) in &frame.upload_chunks {
            if let Some(g) = self.gpu.mesh(mesh) {
                self.chunk_meshes.insert(*pos, g);
            } else {
                self.chunk_meshes.remove(pos);
            }
        }
        let size = frame.size;
        if size != (0, 0) && size != self.gpu.size {
            self.gpu.resize(size.0, size.1);
        }
    }

    fn acquire_surface(&mut self) -> Result<wgpu::SurfaceTexture, RenderError> {
        match self.gpu.surface.get_current_texture() {
            Ok(s) => Ok(s),
            Err(_) => {
                self.gpu
                    .surface
                    .configure(&self.gpu.device, &self.gpu.config);
                self.gpu
                    .surface
                    .get_current_texture()
                    .map_err(|e| RenderError::Surface(e.to_string()))
            }
        }
    }

    fn write_uniforms(&self, frame: &FrameSubmit) {
        if let Some(cam) = frame.camera {
            let vp = cam.view_proj();
            self.gpu.queue.write_buffer(&self.gpu.cam_buf, 0, bytemuck::bytes_of(&vp));
        }
        let screen = [self.gpu.size.0 as f32, self.gpu.size.1 as f32, 0.0, 0.0];
        self.gpu
            .queue
            .write_buffer(&self.gpu.ui_buf, 0, bytemuck::cast_slice(&screen));
    }

    fn tex(&self, idx: Option<usize>) -> wgpu::BindGroup {
        idx.and_then(|i| self.textures.get(i))
            .map(|v| self.gpu.tex_bg(v))
            .unwrap_or_else(|| self.gpu.white_bg.clone())
    }

    fn emit_dynamic(&self, frame: &FrameSubmit) -> DynamicMeshes {
        let mut dyn_mesh = MeshData::default();
        if let Some(cam) = frame.camera {
            emit_particles(&mut dyn_mesh, &frame.particles, cam);
            emit_items(&mut dyn_mesh, &frame.items);
        }
        let mut player_mesh = MeshData::default();
        if let Some(p) = &frame.player {
            emit_player(&mut player_mesh, p);
        }
        let mut held_mesh = MeshData::default();
        if let Some(p) = &frame.player {
            emit_held(&mut held_mesh, p);
        }
        DynamicMeshes {
            dyn_gpu: self.gpu.mesh(&dyn_mesh),
            player_gpu: self.gpu.mesh(&player_mesh),
            held_gpu: self.gpu.mesh(&held_mesh),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_world(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        gpu: &Gpu,
        chunks: &FxHashMap<ChunkPos, GpuMesh>,
        clear: [f32; 4],
        dynamic: &DynamicMeshes,
        atlas_bg: &wgpu::BindGroup,
        skin_bg: &wgpu::BindGroup,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("world"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear[0] as f64,
                        g: clear[1] as f64,
                        b: clear[2] as f64,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &gpu.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&gpu.terrain_pipe);
        pass.set_bind_group(0, &gpu.cam_bg, &[]);
        pass.set_bind_group(1, atlas_bg, &[]);
        for mesh in chunks.values() {
            draw_mesh(&mut pass, mesh);
        }
        if let Some(m) = &dynamic.dyn_gpu {
            draw_mesh(&mut pass, m);
        }
        if let Some(m) = &dynamic.player_gpu {
            pass.set_bind_group(1, skin_bg, &[]);
            draw_mesh(&mut pass, m);
        }
        if let Some(m) = &dynamic.held_gpu {
            pass.set_bind_group(1, atlas_bg, &[]);
            draw_mesh(&mut pass, m);
        }
    }

    fn draw_ui(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        gpu: &Gpu,
        quads: &[crate::submit::UiQuad],
        ui_bg_tex: &wgpu::BindGroup,
    ) {
        let mut ui_mesh = MeshData::default();
        for q in quads {
            emit_ui_quad(&mut ui_mesh, q);
        }
        let Some(m) = gpu.mesh(&ui_mesh) else {
            return;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&gpu.ui_pipe);
        pass.set_bind_group(0, &gpu.ui_bg, &[]);
        pass.set_bind_group(1, ui_bg_tex, &[]);
        draw_mesh(&mut pass, &m);
    }
}

struct DynamicMeshes {
    dyn_gpu: Option<GpuMesh>,
    player_gpu: Option<GpuMesh>,
    held_gpu: Option<GpuMesh>,
}

fn draw_mesh(pass: &mut wgpu::RenderPass<'_>, mesh: &GpuMesh) {
    pass.set_vertex_buffer(0, mesh.verts.slice(..));
    pass.set_index_buffer(mesh.inds.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..mesh.nidx, 0, 0..1);
}
