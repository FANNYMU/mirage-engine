use glam::{Vec2, Vec3, Quat, Mat4};
use std::sync::Arc;
use crate::rendering::{Mesh, Material, Model, Transform};

/// Component that stores the name of an entity
#[derive(Debug, Clone)]
pub struct NameComponent {
    /// The name of the entity
    pub name: String,
}

impl NameComponent {
    /// Create a new name component
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Component that stores the 2D transform of an entity
#[derive(Debug, Clone)]
pub struct Transform2DComponent {
    /// Position in 2D space
    pub position: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// Scale in 2D space
    pub scale: Vec2,
}

impl Transform2DComponent {
    /// Create a new transform component
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self { position, rotation, scale }
    }
    
    /// Create a new transform component with default values
    pub fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
    
    /// Get the transformation matrix
    pub fn matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(Vec3::new(self.position.x, self.position.y, 0.0));
        let rotation = Mat4::from_rotation_z(self.rotation);
        let scale = Mat4::from_scale(Vec3::new(self.scale.x, self.scale.y, 1.0));
        
        translation * rotation * scale
    }
}

/// Component that stores the 3D transform of an entity
#[derive(Debug, Clone)]
pub struct Transform3DComponent {
    /// Position in 3D space
    pub position: Vec3,
    /// Rotation as a quaternion
    pub rotation: Quat,
    /// Scale in 3D space
    pub scale: Vec3,
}

impl Transform3DComponent {
    /// Create a new transform component
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }
    
    /// Create a new transform component with default values
    pub fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
    
    /// Get the transformation matrix
    pub fn matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);
        let rotation = Mat4::from_quat(self.rotation);
        let scale = Mat4::from_scale(self.scale);
        
        translation * rotation * scale
    }
}

/// Component for physics properties
#[derive(Debug, Clone)]
pub struct PhysicsComponent {
    /// Velocity in 2D space
    pub velocity: Vec2,
    /// Angular velocity in radians per second
    pub angular_velocity: f32,
    /// Mass of the entity
    pub mass: f32,
    /// Whether the entity is affected by gravity
    pub use_gravity: bool,
}

impl PhysicsComponent {
    /// Create a new physics component
    pub fn new(velocity: Vec2, angular_velocity: f32, mass: f32, use_gravity: bool) -> Self {
        Self { velocity, angular_velocity, mass, use_gravity }
    }
    
    /// Create a new physics component with default values
    pub fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            mass: 1.0,
            use_gravity: true,
        }
    }
}

/// Component for rendering a model
#[derive(Clone)]
pub struct RenderableComponent {
    /// The model to render
    pub model: Arc<Model>,
    /// Whether the entity is visible
    pub visible: bool,
}

impl RenderableComponent {
    /// Create a new renderable component
    pub fn new(model: Arc<Model>) -> Self {
        Self { model, visible: true }
    }
    
    /// Create a new renderable component with a mesh and material
    pub fn from_parts(device: &wgpu::Device, mesh: Arc<Mesh>, material: Arc<Material>) -> Self {
        let transform = Transform::default();
        let model = Arc::new(Model::new_with_device(
            device,
            mesh,
            material,
            transform,
        ));
        
        Self { model, visible: true }
    }
}

/// Component for camera properties
#[derive(Debug, Clone)]
pub struct CameraComponent {
    /// Whether this is the active camera
    pub is_active: bool,
    /// Field of view in radians (for perspective cameras)
    pub fov: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Whether this is an orthographic camera
    pub is_orthographic: bool,
    /// Orthographic size (half height)
    pub ortho_size: f32,
}

impl CameraComponent {
    /// Create a new perspective camera component
    pub fn new_perspective(fov: f32, near: f32, far: f32) -> Self {
        Self {
            is_active: false,
            fov,
            near,
            far,
            is_orthographic: false,
            ortho_size: 5.0,
        }
    }
    
    /// Create a new orthographic camera component
    pub fn new_orthographic(ortho_size: f32, near: f32, far: f32) -> Self {
        Self {
            is_active: false,
            fov: 60.0_f32.to_radians(),
            near,
            far,
            is_orthographic: true,
            ortho_size,
        }
    }
} 