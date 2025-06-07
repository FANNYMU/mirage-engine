use hecs::World;
use crate::ecs::{Transform2DComponent, Transform3DComponent, PhysicsComponent, RenderableComponent};
use wgpu::Queue;

/// System that updates transforms based on physics
pub fn transform_system(world: &mut World, delta_time: f32) {
    // Update 2D transforms based on physics
    for (_id, (transform, physics)) in world.query_mut::<(&mut Transform2DComponent, &PhysicsComponent)>() {
        // Update position based on velocity
        transform.position += physics.velocity * delta_time;
        
        // Update rotation based on angular velocity
        transform.rotation += physics.angular_velocity * delta_time;
    }
    
    // Update 3D transforms (if needed)
    // This is a placeholder for future 3D physics
}

/// System that handles physics calculations
pub fn physics_system(world: &mut World, delta_time: f32) {
    // Apply gravity to physics components
    let gravity = glam::Vec2::new(0.0, -9.81);
    
    for (_id, physics) in world.query_mut::<&mut PhysicsComponent>() {
        if physics.use_gravity {
            physics.velocity += gravity * delta_time;
        }
        
        // Add collision detection and resolution here
        // ...
    }
}

/// System that prepares data for rendering
pub fn rendering_system(world: &mut World) {
    // Update model transforms from entity transforms
    
    // For 2D entities
    for (_id, (transform, renderable)) in world.query_mut::<(&Transform2DComponent, &RenderableComponent)>() {
        if renderable.visible {
            // Convert 2D transform to 3D transform for the model
            let position = glam::Vec3::new(transform.position.x, transform.position.y, 0.0);
            let rotation = glam::Quat::from_rotation_z(transform.rotation);
            let scale = glam::Vec3::new(transform.scale.x, transform.scale.y, 1.0);
            
            // Create a transform for the model
            let transform_3d = crate::rendering::Transform {
                position,
                rotation,
                scale,
            };
            
            // Update the model's transform if we have access to a queue
            if let Some(queue) = get_render_queue() {
                renderable.model.update_transform(queue, &transform_3d);
            }
        }
    }
    
    // For 3D entities
    for (_id, (transform, renderable)) in world.query_mut::<(&Transform3DComponent, &RenderableComponent)>() {
        if renderable.visible {
            // Create a transform for the model
            let transform_3d = crate::rendering::Transform {
                position: transform.position,
                rotation: transform.rotation,
                scale: transform.scale,
            };
            
            // Update the model's transform if we have access to a queue
            if let Some(queue) = get_render_queue() {
                renderable.model.update_transform(queue, &transform_3d);
            }
        }
    }
}

/// Get the render queue for updating models
/// This is a placeholder - in a real implementation, you would
/// have a way to access the render queue from the ECS
fn get_render_queue() -> Option<&'static Queue> {
    None
} 