//! Our command/resource API. wgpu stays in `device` / `gpu`.

mod device;
mod emit;
mod gpu;
mod submit;

pub mod platform;

pub use device::RenderError;
pub use gpu::Renderer;
pub use submit::{
    Camera, FrameSubmit, GroundShadow, ItemDraw, MeshData, MobDraw, ParticleDraw, PlayerDraw, SelectionDraw,
    TextureId, UiQuad, Vertex, ANCHOR_UV,
};
