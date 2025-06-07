// Rendering module
mod renderer;
mod camera;
mod mesh;
mod texture;
mod shader;
mod material;
mod model;
mod light;

// Re-export for public use
pub use renderer::Renderer;
pub use camera::{Camera, OrthographicCamera};
pub use mesh::{Mesh, Vertex};
pub use texture::Texture;
pub use shader::Shader;
pub use material::Material;
pub use model::{Model, Transform};
pub use light::{Light, DirectionalLight, PointLight, LightManager};