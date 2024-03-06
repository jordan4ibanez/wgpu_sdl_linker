///
/// Attempts to link the SDL2 window into wgpu to create a surface.
///
pub fn link_wgpu_to_sdl2(
  instance: wgpu::Instance,
  window: &sdl2::video::Window,
) -> Result<wgpu::Surface, wgpu::CreateSurfaceError> {
  unsafe { instance.create_surface(window) }
}
