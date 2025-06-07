use uuid::Uuid;

/// A handle to an entity in the ECS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityHandle {
    /// Unique identifier for the entity
    pub uuid: Uuid,
}

impl EntityHandle {
    /// Create a new entity handle with the given UUID
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
    
    /// Generate a new random entity handle
    pub fn generate() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
    
    /// Get the UUID as a string
    pub fn id(&self) -> String {
        self.uuid.to_string()
    }
    
    /// Get a short version of the UUID (first 8 chars)
    pub fn short_id(&self) -> String {
        self.id()[..8].to_string()
    }
} 