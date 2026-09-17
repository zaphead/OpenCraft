use thiserror::Error;
use winit::window::Window;

use crate::device::Gpu;
use crate::gpu::Renderer;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error(transparent)]
    Render(#[from] crate::RenderError),
}

/// Window handoff. winit stays in this module; gameplay talks to `Renderer`.
pub fn create(window: &Window) -> Result<Renderer, PlatformError> {
    let gpu = Gpu::new(window)?;
    Ok(Renderer::from_gpu(gpu))
}
