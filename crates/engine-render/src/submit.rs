use engine_core::{ChunkPos, Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);

/// Atlas contract shared with the client's pack atlas: pixel (255, 255) of
/// the 256px atlas is reserved opaque white. Ground shadows sample it, so
/// their darkening never depends on whatever art lives at the origin tile.
pub const ANCHOR_UV: ([f32; 2], [f32; 2]) = ([255.0 / 256.0, 255.0 / 256.0], [1.0, 1.0]);

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
    /// 1.0 in sun, ~0.38 in a cast shadow. Same tone the mesher bakes.
    pub shade: f32,
}

#[derive(Clone, Debug)]
pub struct PlayerDraw {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub sneaking: bool,
    pub first_person_arm: bool,
    pub held_uv: Option<(Vec2, Vec2, bool)>,
    /// 1.0 in sun, ~0.38 in a cast shadow. Same tone the mesher bakes.
    pub shade: f32,
}

/// One hard-edged streak of shade on the ground, thrown by a body standing
/// in the world. Not a blob: it starts at the caster's feet, runs away from
/// the sun, and keeps sharp block-crisp edges.
#[derive(Clone, Copy, Debug)]
pub struct GroundShadow {
    /// Feet position lifted to just above the ground block top.
    pub foot: Vec3,
    /// Unit XZ run away from the sun.
    pub run: [f32; 2],
    /// Streak length in blocks, from sun elevation.
    pub len: f32,
    /// Half width in blocks.
    pub half: f32,
}

#[derive(Clone, Debug)]
pub struct FrameSubmit {
    pub camera: Option<Camera>,
    pub upload_chunks: Vec<(ChunkPos, MeshData)>,
    pub remove_chunks: Vec<ChunkPos>,
    pub particles: Vec<ParticleDraw>,
    pub items: Vec<ItemDraw>,
    pub player: Option<PlayerDraw>,
    /// Body-cast streaks drawn with the world, in the one existing pass.
    pub ground_shadows: Vec<GroundShadow>,
    pub ui: Vec<UiQuad>,
    pub size: (u32, u32),
    pub clear: [f32; 4],
    /// Unit sun direction the frame is lit from.
    pub sun_dir: Vec3,
    /// Whole-world day brightness. Night goes dark here, never per-mesh.
    pub brightness: f32,
}

impl Default for FrameSubmit {
    // Manual (not derived): brightness and sun must default to a lit noon,
    // never to zeroes that would boot the world black.
    fn default() -> Self {
        Self {
            camera: None,
            upload_chunks: Vec::new(),
            remove_chunks: Vec::new(),
            particles: Vec::new(),
            items: Vec::new(),
            player: None,
            ground_shadows: Vec::new(),
            ui: Vec::new(),
            size: (0, 0),
            clear: [0.45, 0.70, 0.95, 1.0],
            sun_dir: Vec3::new(0.3, 1.0, 0.2).normalize_or_zero(),
            brightness: 1.0,
        }
    }
}
