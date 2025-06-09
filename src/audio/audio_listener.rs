use glam::Vec3;

/// Structure representing an audio listener for 3D audio
#[derive(Debug, Clone, Copy)]
pub struct AudioListener {
    /// Listener position in 3D space
    pub position: Vec3,
    
    /// Listener forward direction
    pub forward: Vec3,
}

impl AudioListener {
    /// Create a new AudioListener instance
    pub fn new(position: Vec3, forward: Vec3) -> Self {
        Self {
            position,
            forward: forward.normalize(),
        }
    }
    
    /// Set the listener position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    
    /// Set the listener forward direction
    pub fn set_forward(&mut self, forward: Vec3) {
        self.forward = forward.normalize();
    }
    
    /// Calculate distance to target position
    pub fn distance_to(&self, position: Vec3) -> f32 {
        self.position.distance(position)
    }
} 