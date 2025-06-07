mod entity;
mod component;
mod system;

pub use entity::*;
pub use component::*;
pub use system::*;

use hecs::{World, Entity};
use std::collections::HashMap;
use uuid::Uuid;

/// The main ECS (Entity Component System) manager
pub struct EcsManager {
    /// The world containing all entities and components
    world: World,
    /// Map from UUID to Entity for easier lookup
    entity_map: HashMap<Uuid, Entity>,
}

impl EcsManager {
    /// Create a new ECS manager
    pub fn new() -> Self {
        Self {
            world: World::new(),
            entity_map: HashMap::new(),
        }
    }
    
    /// Get a reference to the world
    pub fn world(&self) -> &World {
        &self.world
    }
    
    /// Get a mutable reference to the world
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    /// Create a new entity with the given components
    pub fn create_entity(&mut self, components: impl hecs::DynamicBundle) -> EntityHandle {
        let entity = self.world.spawn(components);
        let uuid = Uuid::new_v4();
        self.entity_map.insert(uuid, entity);
        EntityHandle::new(uuid)
    }
    
    /// Get an entity by its handle
    pub fn get_entity(&self, handle: &EntityHandle) -> Option<Entity> {
        self.entity_map.get(&handle.uuid).copied()
    }
    
    /// Destroy an entity by its handle
    pub fn destroy_entity(&mut self, handle: &EntityHandle) -> bool {
        if let Some(entity) = self.entity_map.remove(&handle.uuid) {
            self.world.despawn(entity).is_ok()
        } else {
            false
        }
    }
    
    /// Run all registered systems
    pub fn run_systems(&mut self, delta_time: f32) {
        // Run transform system
        transform_system(&mut self.world, delta_time);
        
        // Run physics system
        physics_system(&mut self.world, delta_time);
        
        // Run rendering system (prepare data for renderer)
        rendering_system(&mut self.world);
    }
}

impl Default for EcsManager {
    fn default() -> Self {
        Self::new()
    }
} 