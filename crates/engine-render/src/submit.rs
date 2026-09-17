use engine_core::{ChunkPos, Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Debug, Default)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn view_proj(self) -> [[f32; 4]; 4] {
        let f = self.look();
        let target = self.pos + f;
        let view = engine_core::Mat4::look_at_rh(self.pos, target, Vec3::Y);
        let proj = engine_core::Mat4::perspective_rh(self.fov_y, self.aspect.max(0.01), 0.05, 512.0);
        (proj * view).to_cols_array_2d()
    }

    pub fn look(self) -> Vec3 {
        engine_core::yaw_pitch_dir(self.yaw, self.pitch)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UiQuad {
    pub min: Vec2,
    pub max: Vec2,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
    pub color: [f32; 4],
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ParticleDraw {
    pub pos: Vec3,
    pub size: f32,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct ItemDraw {
    pub pos: Vec3,
    pub yaw: f32,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
    pub is_block: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerDraw {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub sneaking: bool,
    pub first_person_arm: bool,
    pub held_uv: Option<(Vec2, Vec2, bool)>,
}

#[derive(Clone, Debug, Default)]
pub struct FrameSubmit {
    pub camera: Option<Camera>,
    pub upload_chunks: Vec<(ChunkPos, MeshData)>,
    pub remove_chunks: Vec<ChunkPos>,
    pub particles: Vec<ParticleDraw>,
    pub items: Vec<ItemDraw>,
    pub player: Option<PlayerDraw>,
    pub ui: Vec<UiQuad>,
    pub size: (u32, u32),
    pub clear: [f32; 4],
}
