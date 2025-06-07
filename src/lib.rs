pub mod core;
pub mod scene;
pub mod ui;
pub mod rendering;
pub mod utils;
pub mod ecs;

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