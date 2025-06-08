mod core;
mod scene;
mod ui;
mod rendering;
mod utils;
mod ecs;
pub mod audio;

pub use core::*;
pub use scene::*;
pub use ui::*;
pub use rendering::*;
pub use utils::*;
pub use ecs::*;
pub use audio::*;

// Re-export common types
pub use rendering::{
    Renderer, Camera, OrthographicCamera, Mesh, Vertex, Texture, Shader, Material,
    Model, Transform, Light, DirectionalLight, PointLight, LightManager
};
pub use scene::{Scene, SceneManager, SceneState};
pub use core::{GameLoop, DeltaTime, EventSystem};
pub use ecs::{
    EcsManager, EntityHandle, 
    NameComponent, Transform2DComponent, Transform3DComponent, 
    PhysicsComponent, RenderableComponent, CameraComponent
}; 