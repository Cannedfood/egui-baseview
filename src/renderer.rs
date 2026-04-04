#[cfg(feature = "opengl")]
mod opengl;
#[cfg(all(feature = "opengl", not(feature = "wgpu")))]
pub use opengl::renderer::{GraphicsConfig, Renderer};
#[cfg(all(feature = "opengl", feature = "wgpu"))]
pub use opengl::renderer::{GraphicsConfig, Renderer};

#[cfg(feature = "wgpu")]
mod wgpu;
#[allow(unused_imports)]
#[cfg(all(feature = "wgpu", not(feature = "opengl")))]
pub use wgpu::renderer::{GraphicsConfig, Renderer};
#[allow(unused_imports)]
#[cfg(all(feature = "wgpu", feature = "opengl"))]
pub use wgpu::renderer::{GraphicsConfig as WgpuGraphicsConfig, Renderer as WgpuRenderer};
